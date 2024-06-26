use futures::{future, sink, stream};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::time;

mod servo_cmd;

/// A simple controller for an Arctos robot arm using canbus.
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Interface name for the CAN network to use.
    #[arg(short, long, default_value = "can0")]
    ifname: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Axes {
        #[arg(short, long)]
        all: bool,
        #[arg(long, value_enum)]
        axes: Vec<Axis>,
        #[command(subcommand)]
        axes_command: AxesCommand,
    },
}

#[derive(Debug, clap::Subcommand)]
enum AxesCommand {
    /// Initialize (configure settings for) axis motors.
    Init,
    /// Enable (power on) axis motors.
    Enable,
    /// Set the origin of the specified axes to whatever the current position of the robot is.
    SetOrigin,
    /// Get the current axis positions, from the point of view of the motor(s).
    GetMotorPos,
    /// Set the axis positions, from the point of view of the motor(s).
    SetMotorPos {
        /// The raw position in number of servo rotations from origin.
        position: f64,
        /// The acceleration of the motor, in RPM/s².
        #[arg(long)]
        accel: Option<f64>,
        /// The acceleration of the motor, with more control compared to the `accel` flag.
        /// Determines the rate at which `speed` is ramped up, according to the formula:
        /// `t2 – t1 = (256 - accel_raw) * 50 µs`.
        ///
        /// A value of `0` corresponds to infinity acceleration, ie the motor will try to run at
        /// the specified speed immediately.
        ///
        /// In other words:
        ///
        ///   * A value of `accel=216` will mean the motor accelerates by 1 RPM with
        ///     `(256-216) * 50µs = 40 * 50µs = 1ms` intervals, ie. `500 RPM / s²` acceleration.
        ///   * A value of `accel=236` will mean the motor accelerates by 1 RPM with
        ///     `(256-236) * 50µs = 20 * 50µs = 1ms` intervals, ie. `1000 RPM / s²` acceleration.
        ///   * A value of `accel=1` (slowest possible) will mean the motor accelerates by 1 RPM
        ///     with `(256-1) * 50µs = 255 * 50µs = 12.75ms` intervals, ie. `78.4313725 RPM / s²`
        ///     acceleration.
        ///     `(256-236) * 50µs = 20 * 50µs = 1ms` intervals, ie. `1000 RPM / s²` acceleration.
        ///   * A value of `accel=255` (fastest possible) will mean the motor accelerates by 1 RPM
        ///     with `(256-255) * 50µs = 1 * 50µs = 50µs` intervals, ie. `20_000 RPM / s²`
        ///     acceleration.
        #[arg(long, verbatim_doc_comment)]
        accel_raw: Option<u8>,
        /// The speed of the motor in RPM.
        #[arg(short, long)]
        speed: Option<f64>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Axis {
    X,
    Y,
    Z,
    A,
    B,
    C,
}

impl Axis {
    pub fn id(&self) -> socketcan::Id {
        match *self {
            Axis::X => socketcan::Id::Standard(socketcan::StandardId::new(1).unwrap()),
            Axis::Y => socketcan::Id::Standard(socketcan::StandardId::new(2).unwrap()),
            Axis::Z => socketcan::Id::Standard(socketcan::StandardId::new(3).unwrap()),
            Axis::A => socketcan::Id::Standard(socketcan::StandardId::new(4).unwrap()),
            Axis::B => socketcan::Id::Standard(socketcan::StandardId::new(5).unwrap()),
            Axis::C => socketcan::Id::Standard(socketcan::StandardId::new(6).unwrap()),
        }
    }

    fn default_speed(&self) -> u16 {
        match *self {
            Axis::X => 300,
            Axis::Y => 300,
            Axis::Z => 300,
            Axis::A => 500,
            Axis::B => 500,
            Axis::C => 500,
        }
    }

    fn default_accel(&self) -> u8 {
        match *self {
            Axis::X => 176,
            Axis::Y => 176,
            Axis::Z => 176,
            Axis::A => 216,
            Axis::B => 236,
            Axis::C => 236,
        }
    }

    fn gearing_factor(&self) -> Option<f64> {
        match *self {
            Axis::X => Some(13.6),
            Axis::Y => None,
            Axis::Z => None,
            Axis::A => Some(5.1),
            Axis::B => None,
            Axis::C => None,
        }
    }

    fn actuation_range(&self) -> Option<(f64, f64)> {
        match *self {
            Axis::X => None,
            Axis::Y => Some((-60.0, 30.0)),
            Axis::Z => Some((50.0, 0.0)),
            Axis::A => None,
            Axis::B => None,
            Axis::C => None,
        }
    }
}

type CanFrameTx = sink::SinkErrInto<
    tokio_util::sync::PollSender<socketcan::CanFrame>,
    socketcan::CanFrame,
    anyhow::Error,
>;
type CanFrameRx =
    stream::ErrInto<tokio_stream::wrappers::BroadcastStream<socketcan::CanFrame>, anyhow::Error>;

async fn par_map_canbus<A, I, Tx, Rx, F, R, RV>(
    values: I,
    mut can_tx: Tx,
    mut can_rx: Rx,
    ref action: F,
) -> anyhow::Result<Vec<RV>>
where
    I: IntoIterator<Item = A>,
    Tx: sink::Sink<socketcan::CanFrame, Error = anyhow::Error> + Unpin,
    Rx: stream::Stream<Item = anyhow::Result<socketcan::CanFrame>> + Unpin,
    F: Fn(A, CanFrameTx, CanFrameRx) -> R,
    R: future::Future<Output = anyhow::Result<RV>>,
{
    use future::FutureExt as _;
    use futures_util::stream::TryStreamExt as _;
    use sink::SinkExt as _;
    use stream::StreamExt as _;

    let (ref can_broadcast_tx, _) = broadcast::channel(16);
    let (done_tx_send, mut done_tx_recv) = oneshot::channel();
    let (done_rx_send, mut done_rx_recv) = oneshot::channel();
    let (can_collect_tx, mut can_collect_rx) = mpsc::channel(1);
    let rx_task = async move {
        loop {
            tokio::select! {
                item = can_rx.next() => {
                    if let Some(item) = item.transpose()? {
                        can_broadcast_tx.send(item)?;
                    }
                }
                _ = &mut done_rx_recv => {
                    break;
                }
            }
        }
        anyhow::Ok(())
    };
    let tx_task = async move {
        loop {
            tokio::select! {
                item = can_collect_rx.recv() => {
                    if let Some(item) = item {
                        can_tx.send(item).await?;
                    }
                }
                _ = &mut done_tx_recv => {
                    break;
                }
            }
        }
        anyhow::Ok(())
    };
    let workers_task = future::try_join_all(values.into_iter().map(|value| {
        let can_tx = tokio_util::sync::PollSender::new(can_collect_tx.clone()).sink_err_into();
        let can_rx =
            tokio_stream::wrappers::BroadcastStream::new(can_broadcast_tx.subscribe()).err_into();
        action(value, can_tx, can_rx)
    }))
    .inspect(|_| done_tx_send.send(()).unwrap())
    .inspect(|_| done_rx_send.send(()).unwrap());
    let (_, _, results) = future::try_join3(rx_task, tx_task, workers_task).await?;
    anyhow::Ok(results)
}

#[tracing::instrument(skip(can_tx, can_rx))]
async fn init_axis(
    axis: Axis,
    mut can_tx: impl sink::Sink<socketcan::CanFrame, Error = anyhow::Error> + Unpin,
    mut can_rx: impl stream::Stream<Item = anyhow::Result<socketcan::CanFrame>> + Unpin,
) -> anyhow::Result<()> {
    use sink::SinkExt as _;

    let set_work_mode = can_tx.send(
        servo_cmd::ServoRequest::SetWorkMode {
            work_mode: servo_cmd::WorkMode::SrVFoc,
        }
        .to_frame(axis.id())?,
    );
    let await_work_mode = await_axis_response(&mut can_rx, axis, |response| async move {
        if let servo_cmd::ServoResponse::SetWorkMode { success } = response {
            let status = if success { "success" } else { "fail" };
            tracing::info!("set SR_vFOC work mode: {status}");
            if success {
                Ok(Some(()))
            } else {
                anyhow::bail!("failed to set work mode for axis {axis:?}")
            }
        } else {
            Ok(None)
        }
    });

    futures::try_join!(set_work_mode, await_work_mode)?;

    let set_display_off =
        can_tx.send(servo_cmd::ServoRequest::SetAutoSSD { enable: true }.to_frame(axis.id())?);
    let await_display_off = await_axis_response(&mut can_rx, axis, |response| async move {
        if let servo_cmd::ServoResponse::SetAutoSSD { success } = response {
            let status = if success { "success" } else { "fail" };
            tracing::info!("turn off display: {status}");
            if success {
                Ok(Some(()))
            } else {
                anyhow::bail!("failed to turn off display for axis {axis:?}")
            }
        } else {
            Ok(None)
        }
    });

    futures::try_join!(set_display_off, await_display_off)?;

    Ok(())
}

#[tracing::instrument(skip(can_tx, can_rx))]
async fn enable_axis(
    axis: Axis,
    mut can_tx: impl sink::Sink<socketcan::CanFrame, Error = anyhow::Error> + Unpin,
    can_rx: impl stream::Stream<Item = anyhow::Result<socketcan::CanFrame>> + Unpin,
) -> anyhow::Result<()> {
    use futures_util::SinkExt as _;

    let set_enabled =
        can_tx.send(servo_cmd::ServoRequest::Enable { enabled: true }.to_frame(axis.id())?);
    let await_enabled = await_axis_response(can_rx, axis, |response| async move {
        if let servo_cmd::ServoResponse::Enable { success } = response {
            let status = if success { "success" } else { "fail" };
            tracing::info!("enable: {status}");
            if success {
                Ok(Some(()))
            } else {
                anyhow::bail!("failed to enable axis {axis:?}")
            }
        } else {
            Ok(None)
        }
    });

    futures::try_join!(set_enabled, await_enabled)?;

    Ok(())
}

#[tracing::instrument(skip(can_tx, can_rx))]
async fn set_origin(
    axis: Axis,
    mut can_tx: impl sink::Sink<socketcan::CanFrame, Error = anyhow::Error> + Unpin,
    can_rx: impl stream::Stream<Item = anyhow::Result<socketcan::CanFrame>> + Unpin,
) -> anyhow::Result<()> {
    use futures_util::SinkExt as _;

    let set_axis_zero = can_tx.send(servo_cmd::ServoRequest::SetAxisZero.to_frame(axis.id())?);
    let await_axis_zero = await_axis_response(can_rx, axis, |response| async move {
        if let servo_cmd::ServoResponse::SetAxisZero { success } = response {
            let status = if success { "success" } else { "fail" };
            tracing::info!("set origin: {status}");
            if success {
                Ok(Some(()))
            } else {
                anyhow::bail!("failed to set origin for axis {axis:?}")
            }
        } else {
            Ok(None)
        }
    });

    futures::try_join!(set_axis_zero, await_axis_zero)?;

    Ok(())
}

#[tracing::instrument(skip(can_tx, can_rx))]
async fn get_axis_pos_raw(
    axis: Axis,
    mut can_tx: impl sink::Sink<socketcan::CanFrame, Error = anyhow::Error> + Unpin,
    can_rx: impl stream::Stream<Item = anyhow::Result<socketcan::CanFrame>> + Unpin,
) -> anyhow::Result<Option<i64>> {
    use futures_util::SinkExt as _;

    let read_encoder_value =
        can_tx.send(servo_cmd::ServoRequest::ReadEncoderValueAddition.to_frame(axis.id())?);
    let await_encoder_value = await_axis_response(can_rx, axis, |response| async move {
        if let servo_cmd::ServoResponse::ReadEncoderValueAddition { value } = response {
            tracing::info!("read encoder value: {value}");
            Ok(Some(value))
        } else {
            Ok(None)
        }
    });

    let value = futures::try_join!(read_encoder_value, await_encoder_value)?.1;

    Ok(value)
}

#[tracing::instrument(skip(can_tx, can_rx))]
async fn set_axis_pos_raw(
    axis: Axis,
    position: f64,
    speed: u16,
    accel: u8,
    mut can_tx: impl sink::Sink<socketcan::CanFrame, Error = anyhow::Error> + Unpin,
    can_rx: impl stream::Stream<Item = anyhow::Result<socketcan::CanFrame>> + Unpin,
) -> anyhow::Result<()> {
    use futures_util::SinkExt as _;

    let request = servo_cmd::ServoRequest::RunPositionAbsoluteMotionMode {
        speed,
        accel,
        abs_axis: (position * 0x4000 as f64) as i32,
    }
    .to_frame(axis.id())?;
    can_tx.send(request).await?;

    await_axis_response(can_rx, axis, |response| async move {
        if let servo_cmd::ServoResponse::RunPositionAbsoluteMotionMode { status } = response {
            tracing::info!("set axis pos: {status:?}");
            match status {
                servo_cmd::MotionStatus::Fail => {
                    anyhow::bail!("failed to set origin for axis {axis:?}")
                }
                servo_cmd::MotionStatus::Busy => {
                    Ok(None)
                }
                servo_cmd::MotionStatus::Success => {
                    Ok(Some(()))
                }
                servo_cmd::MotionStatus::LimitReached => {
                    tracing::warn!("endstop triggered when trying to set axis position to {position} for axis {axis:?}");
                    Ok(Some(()))
                }
            }
        } else {
            Ok(None)
        }
    })
    .await?;

    Ok(())
}

async fn await_axis_response<Rx, H, F, A>(
    mut can_rx: Rx,
    axis: Axis,
    mut response_handler: H,
) -> anyhow::Result<Option<A>>
where
    Rx: stream::Stream<Item = anyhow::Result<socketcan::CanFrame>> + Unpin,
    H: FnMut(servo_cmd::ServoResponse) -> F,
    F: future::Future<Output = anyhow::Result<Option<A>>>,
{
    use anyhow::Context as _;
    use futures_util::StreamExt as _;
    use socketcan::EmbeddedFrame as _;

    let await_response = async {
        while let Some(frame) = can_rx.next().await {
            let frame = frame?;
            if frame.id() == axis.id() {
                let response = servo_cmd::ServoResponse::from_frame(axis.id(), frame)?;
                if let Some(a) = response_handler(response).await? {
                    return anyhow::Ok(Some(a));
                }
            }
        }
        anyhow::Ok(None)
    };
    let timeout = time::timeout(time::Duration::from_millis(100), await_response);
    timeout
        .await
        .unwrap_or_else(|e| Err(e.into()))
        .context(format!("didn't get a response for axis {axis:?}"))
}

#[tokio::main]
async fn main() {
    use clap::Parser as _;
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    if let Err(err) = run(args).await {
        tracing::error!("{err:#}");
        std::process::exit(1);
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    use clap::ValueEnum as _;
    use futures::TryStreamExt as _;
    use futures_util::SinkExt as _;
    use futures_util::StreamExt as _;

    let socket = socketcan::tokio::CanSocket::open(&args.ifname)?;
    let (can_tx, can_rx) = socket.split();
    let can_tx = can_tx.sink_err_into();
    let can_rx = can_rx.err_into();
    match args.command {
        Command::Axes {
            all,
            axes,
            axes_command,
        } => {
            let axes = if all { Axis::value_variants() } else { &axes }
                .into_iter()
                .copied();
            match axes_command {
                AxesCommand::Init => {
                    par_map_canbus(axes, can_tx, can_rx, init_axis).await?;
                }
                AxesCommand::Enable => {
                    par_map_canbus(axes, can_tx, can_rx, enable_axis).await?;
                }
                AxesCommand::GetMotorPos => {
                    par_map_canbus(axes, can_tx, can_rx, get_axis_pos_raw).await?;
                }
                AxesCommand::SetMotorPos {
                    position,
                    speed,
                    accel: _, // TODO
                    accel_raw,
                } => {
                    par_map_canbus(axes, can_tx, can_rx, |a, t, r| {
                        set_axis_pos_raw(
                            a,
                            position,
                            speed.map(|s| s as u16).unwrap_or(a.default_speed()),
                            accel_raw.unwrap_or(a.default_accel()),
                            t,
                            r,
                        )
                    })
                    .await?;
                }
                AxesCommand::SetOrigin => {
                    par_map_canbus(axes, can_tx, can_rx, set_origin).await?;
                }
            }
        }
    }

    Ok(())
}
