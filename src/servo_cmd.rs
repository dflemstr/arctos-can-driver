#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum WorkMode {
    CrOpen = 0,
    CrClose = 1,
    CrVFoc = 2,
    SrOpen = 3,
    SrClose = 4,
    SrVFoc = 5,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum EnPinActiveMode {
    Low = 0,
    High = 1,
    Always = 2,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum CanBitRate {
    B125K = 0,
    B250K = 1,
    B500K = 2,
    B1M = 3,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum ZeroMode {
    Disable = 0,
    DirMode = 1,
    NearMode = 2,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum ZeroModeSpeed {
    Speed0 = 0,
    Speed1 = 1,
    Speed2 = 2,
    Speed3 = 3,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum HomeTrig {
    Low = 0,
    High = 1,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    CW = 0,
    CCW = 1,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum SaveState {
    Save = 0xc8,
    Clean = 0xca,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum ProgressStatus {
    Busy = 0,
    Success = 1,
    Fail = 2,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum MotionStatus {
    Fail = 0,
    Busy = 1,
    Success = 2,
    LimitReached = 3,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
pub enum MotorStatus {
    MotorStopped,
    MotorSpeedingUp,
    MotorSpeedingDown,
    MotorFullSpeed,
    MotorHoming,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum ServoOpcode {
    ReadEncoderValueCarry = 0x30,
    ReadEncoderValueAddition = 0x31,
    ReadSpeed = 0x32,
    ReadPulses = 0x33,
    ReadIOPorts = 0x34,
    ReadError = 0x39,
    ReadEnPin = 0x3a,
    ReadGoBackToZeroOnPowerOnStatus = 0x3b,
    ReleaseMotorShaft = 0x3d,
    ReadMotorShaftLockedRotor = 0x3e,
    Calibrate = 0x80,
    SetWorkMode = 0x82,
    SetCurrent = 0x83,
    SetSubdivision = 0x84,
    SetEnPinActiveMode = 0x85,
    SetDir = 0x86,
    SetAutoSSD = 0x87,
    SetMotorShaftLockedRotor = 0x88,
    SetSubdivisionInterpolation = 0x89,
    SetCanBitRate = 0x8a,
    SetCanId = 0x8b,
    SetCanEnableResponses = 0x8c,
    SetKeyLocked = 0x8f,
    SetGroupId = 0x8d,
    SetHome = 0x90,
    GoHome = 0x91,
    SetAxisZero = 0x92,
    SetZeroOnPowerOnMode = 0x9a,
    RestoreDefaults = 0x3f,
    QueryStatus = 0xf1,
    Enable = 0xf3,
    RunSpeedMode = 0xf6,
    //StopSpeedMode = 0xf6,
    SaveRunModeParams = 0xff,
    RunPositionRelativePulsesMode = 0xfd,
    RunPositionRelativeMotionMode = 0xf4,
    RunPositionAbsoluteMotionMode = 0xf5,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ServoRequest {
    ReadEncoderValueCarry,
    ReadEncoderValueAddition,
    ReadSpeed,
    ReadPulses,
    ReadIOPorts,
    ReadError,
    ReadEnPin,
    ReadGoBackToZeroOnPowerOnStatus,
    ReleaseMotorShaft,
    ReadMotorShaftLockedRotor,
    Calibrate,
    SetWorkMode {
        work_mode: WorkMode,
    },
    SetCurrent {
        /// Current in mA
        current: u16,
    },
    SetSubdivision {
        microsteps: u8,
    },
    SetEnPinActiveMode {
        active: EnPinActiveMode,
    },
    SetDir {
        dir: Direction,
    },
    SetAutoSSD {
        enable: bool,
    },
    SetMotorShaftLockedRotor {
        enable: bool,
    },
    SetSubdivisionInterpolation {
        enable: bool,
    },
    SetCanBitRate {
        bit_rate: CanBitRate,
    },
    SetCanId {
        id: u16,
    },
    SetCanEnableResponses {
        enable: bool,
    },
    SetKeyLocked {
        enable: bool,
    },
    SetGroupId {
        id: u16,
    },
    SetHome {
        home_trig: HomeTrig,
        home_dir: Direction,
        home_speed: u16,
        end_limit: bool,
    },
    GoHome,
    SetAxisZero,
    SetZeroOnPoweronMode {
        zero_mode: ZeroMode,
        enable: bool,
        speed: ZeroModeSpeed,
        dir: Direction,
    },
    RestoreDefaults,
    QueryStatus,
    Enable {
        enabled: bool,
    },
    RunSpeedMode {
        dir: Direction,
        speed: u16,
        acc: u8,
    },
    SaveRunModeParams {
        save_state: SaveState,
    },
    RunPositionRelativePulsesMode {
        dir: Direction,
        speed: u16,
        acc: u8,
        pulses: u16,
    },
    RunPositionRelativeMotionMode {
        speed: u16,
        acc: u8,
        rel_axis: i32,
    },
    RunPositionAbsoluteMotionMode {
        speed: u16,
        accel: u8,
        abs_axis: i32,
    },
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ServoResponse {
    ReadEncoderValueCarry {
        /// Number of turns the encoder has completed (CCW positive, CW negative).
        carry: i32,
        /// Current encoder position in `0x0-0x3fff`
        value: u16,
    },
    ReadEncoderValueAddition {
        /// Current encoder position from zero, where `0x4000` is a full turn.
        value: i64,
    },
    ReadSpeed {
        /// Current speed in RPM.
        speed: i16,
    },
    ReadPulses {
        /// Running count of pulses received.
        pulses: i32,
    },
    ReadIOPorts {
        out_1: bool,
        out_2: bool,
        in_1: bool,
        in_2: bool,
    },
    ReadError {
        /// Current error (difference from set pos) in `0..51200 ~= 0..360 deg`
        error: i32,
    },
    ReadEnPin {
        /// Whether the `en` pin is enabled.
        enabled: bool,
    },
    ReadGoBackToZeroOnPowerOnStatus {
        /// The status of going back to zero.
        status: ProgressStatus,
    },
    ReleaseMotorShaft {
        /// Whether the motor shaft was successfully unlocked.
        success: bool,
    },
    ReadMotorShaftLockedRotor {
        /// Whether the rotor is currently locked.
        locked: bool,
    },
    Calibrate {
        /// The status of the ongoing calibration.
        status: ProgressStatus,
    },
    SetWorkMode {
        success: bool,
    },
    SetCurrent {
        success: bool,
    },
    SetSubdivision {
        success: bool,
    },
    SetEnPinActiveMode {
        success: bool,
    },
    SetDir {
        success: bool,
    },
    SetAutoSSD {
        success: bool,
    },
    SetMotorShaftLockedRotor {
        success: bool,
    },
    SetSubdivisionInterpolation {
        success: bool,
    },
    SetCanBitRate {
        success: bool,
    },
    SetCanId {
        success: bool,
    },
    SetCanEnableResponses {
        success: bool,
    },
    SetKeyLocked {
        success: bool,
    },
    SetGroupId {
        success: bool,
    },
    SetHome {
        success: bool,
    },
    GoHome {
        progress: ProgressStatus,
    },
    SetAxisZero {
        success: bool,
    },
    SetZeroOnPowerOnMode {
        success: bool,
    },
    RestoreDefaults {
        success: bool,
    },
    QueryStatus {
        status: Option<MotorStatus>,
    },
    Enable {
        success: bool,
    },
    RunSpeedMode {
        status: MotionStatus,
    },
    SaveRunModeParams {
        success: bool,
    },
    RunPositionRelativePulsesMode {
        status: MotionStatus,
    },
    RunPositionRelativeMotionMode {
        status: MotionStatus,
    },
    RunPositionAbsoluteMotionMode {
        status: MotionStatus,
    },
}

impl ServoRequest {
    pub fn to_frame(&self, id: socketcan::Id) -> anyhow::Result<socketcan::CanFrame> {
        // Ensure there's always a zero byte for CRC at the end of every slice passed to Self::add_crc
        match *self {
            ServoRequest::ReadEncoderValueCarry => {
                Self::add_crc(id, &mut [ServoOpcode::ReadEncoderValueCarry as u8, 0])
            }
            ServoRequest::ReadEncoderValueAddition => {
                Self::add_crc(id, &mut [ServoOpcode::ReadEncoderValueAddition as u8, 0])
            }
            ServoRequest::ReadSpeed => Self::add_crc(id, &mut [ServoOpcode::ReadSpeed as u8, 0]),
            ServoRequest::ReadPulses => Self::add_crc(id, &mut [ServoOpcode::ReadPulses as u8, 0]),
            ServoRequest::ReadIOPorts => {
                Self::add_crc(id, &mut [ServoOpcode::ReadIOPorts as u8, 0])
            }
            ServoRequest::ReadError => Self::add_crc(id, &mut [ServoOpcode::ReadError as u8, 0]),
            ServoRequest::ReadEnPin => Self::add_crc(id, &mut [ServoOpcode::ReadEnPin as u8, 0]),
            ServoRequest::ReadGoBackToZeroOnPowerOnStatus => Self::add_crc(
                id,
                &mut [ServoOpcode::ReadGoBackToZeroOnPowerOnStatus as u8, 0],
            ),
            ServoRequest::ReleaseMotorShaft => {
                Self::add_crc(id, &mut [ServoOpcode::ReleaseMotorShaft as u8, 0])
            }
            ServoRequest::ReadMotorShaftLockedRotor => {
                Self::add_crc(id, &mut [ServoOpcode::ReadMotorShaftLockedRotor as u8, 0])
            }
            ServoRequest::Calibrate => Self::add_crc(id, &mut [ServoOpcode::Calibrate as u8, 0]),
            ServoRequest::SetWorkMode { work_mode } => Self::add_crc(
                id,
                &mut [ServoOpcode::SetWorkMode as u8, work_mode as u8, 0],
            ),
            ServoRequest::SetCurrent { current } => {
                let [b0, b1] = current.to_be_bytes();
                Self::add_crc(id, &mut [ServoOpcode::SetCurrent as u8, b0, b1, 0])
            }
            ServoRequest::SetSubdivision { microsteps } => {
                Self::add_crc(id, &mut [ServoOpcode::SetSubdivision as u8, microsteps, 0])
            }
            ServoRequest::SetEnPinActiveMode { active } => Self::add_crc(
                id,
                &mut [ServoOpcode::SetEnPinActiveMode as u8, active as u8, 0],
            ),
            ServoRequest::SetDir { dir } => {
                Self::add_crc(id, &mut [ServoOpcode::SetDir as u8, dir as u8, 0])
            }
            ServoRequest::SetAutoSSD { enable } => {
                Self::add_crc(id, &mut [ServoOpcode::SetAutoSSD as u8, enable as u8, 0])
            }
            ServoRequest::SetMotorShaftLockedRotor { enable } => Self::add_crc(
                id,
                &mut [ServoOpcode::SetMotorShaftLockedRotor as u8, enable as u8, 0],
            ),
            ServoRequest::SetSubdivisionInterpolation { enable } => Self::add_crc(
                id,
                &mut [
                    ServoOpcode::SetSubdivisionInterpolation as u8,
                    enable as u8,
                    0,
                ],
            ),
            ServoRequest::SetCanBitRate { bit_rate } => Self::add_crc(
                id,
                &mut [ServoOpcode::SetCanBitRate as u8, bit_rate as u8, 0],
            ),
            ServoRequest::SetCanId { id: can_id } => {
                assert!(can_id <= 0x7ff);
                let [b0, b1] = can_id.to_be_bytes();
                Self::add_crc(id, &mut [ServoOpcode::SetCanId as u8, b0, b1, 0])
            }
            ServoRequest::SetCanEnableResponses { enable } => Self::add_crc(
                id,
                &mut [ServoOpcode::SetCanEnableResponses as u8, enable as u8, 0],
            ),
            ServoRequest::SetKeyLocked { enable } => {
                Self::add_crc(id, &mut [ServoOpcode::SetKeyLocked as u8, enable as u8, 0])
            }
            ServoRequest::SetGroupId { id: group_id } => {
                assert!(group_id <= 0x7ff);
                let [b0, b1] = group_id.to_be_bytes();
                Self::add_crc(id, &mut [ServoOpcode::SetGroupId as u8, b0, b1, 0])
            }
            ServoRequest::SetHome {
                home_trig,
                home_dir,
                home_speed,
                end_limit,
            } => {
                assert!(home_speed <= 3000);
                let [b0, b1] = home_speed.to_be_bytes();
                Self::add_crc(
                    id,
                    &mut [
                        ServoOpcode::SetHome as u8,
                        home_trig as u8,
                        home_dir as u8,
                        b0,
                        b1,
                        end_limit as u8,
                        0,
                    ],
                )
            }
            ServoRequest::GoHome => Self::add_crc(id, &mut [ServoOpcode::GoHome as u8, 0]),
            ServoRequest::SetAxisZero => {
                Self::add_crc(id, &mut [ServoOpcode::SetAxisZero as u8, 0])
            }
            ServoRequest::SetZeroOnPoweronMode {
                zero_mode,
                enable,
                speed,
                dir,
            } => Self::add_crc(
                id,
                &mut [
                    ServoOpcode::SetZeroOnPowerOnMode as u8,
                    zero_mode as u8,
                    enable as u8,
                    speed as u8,
                    dir as u8,
                    0,
                ],
            ),
            ServoRequest::RestoreDefaults => {
                Self::add_crc(id, &mut [ServoOpcode::RestoreDefaults as u8, 0])
            }
            ServoRequest::QueryStatus => {
                Self::add_crc(id, &mut [ServoOpcode::QueryStatus as u8, 0])
            }
            ServoRequest::Enable { enabled } => {
                Self::add_crc(id, &mut [ServoOpcode::Enable as u8, enabled as u8, 0])
            }
            ServoRequest::RunSpeedMode { dir, speed, acc } => Self::add_crc(
                id,
                &mut [
                    ServoOpcode::RunSpeedMode as u8,
                    ((dir as u8) << 7) | ((speed >> 8) as u8),
                    speed as u8,
                    acc,
                    0,
                ],
            ),
            ServoRequest::SaveRunModeParams { save_state } => Self::add_crc(
                id,
                &mut [ServoOpcode::SaveRunModeParams as u8, save_state as u8, 0],
            ),
            ServoRequest::RunPositionRelativePulsesMode {
                dir,
                speed,
                acc,
                pulses,
            } => {
                let [b0, b1] = pulses.to_be_bytes();
                Self::add_crc(
                    id,
                    &mut [
                        ServoOpcode::RunPositionRelativePulsesMode as u8,
                        ((dir as u8) << 7) | ((speed >> 8) as u8),
                        speed as u8,
                        acc,
                        b0,
                        b1,
                        0,
                    ],
                )
            }
            ServoRequest::RunPositionRelativeMotionMode {
                speed,
                acc,
                rel_axis,
            } => {
                assert!(rel_axis <= (2i32.pow(23) - 1));
                assert!(rel_axis >= -(2i32.pow(23)));
                let [s0, s1] = speed.to_be_bytes();
                let [_, b1, b2, b3] = rel_axis.to_be_bytes();
                Self::add_crc(
                    id,
                    &mut [
                        ServoOpcode::RunPositionRelativeMotionMode as u8,
                        s0,
                        s1,
                        acc,
                        b1,
                        b2,
                        b3,
                        0,
                    ],
                )
            }
            ServoRequest::RunPositionAbsoluteMotionMode {
                speed,
                accel: acc,
                abs_axis,
            } => {
                assert!(abs_axis <= (2i32.pow(23) - 1));
                assert!(abs_axis >= -(2i32.pow(23)));
                let [s0, s1] = speed.to_be_bytes();
                let [_, b1, b2, b3] = abs_axis.to_be_bytes();
                Self::add_crc(
                    id,
                    &mut [
                        ServoOpcode::RunPositionAbsoluteMotionMode as u8,
                        s0,
                        s1,
                        acc,
                        b1,
                        b2,
                        b3,
                        0,
                    ],
                )
            }
        }
    }

    fn add_crc(id: socketcan::Id, data: &mut [u8]) -> anyhow::Result<socketcan::CanFrame> {
        use socketcan::EmbeddedFrame as _;

        let (crc_ref, rest) = data
            .split_last_mut()
            .ok_or_else(|| anyhow::format_err!("empty frame"))?;
        assert_eq!(
            0, *crc_ref,
            "must pass in zero CRC byte at the end of the frame data"
        );

        let crc = compute_crc(id, rest);

        *crc_ref = crc;
        socketcan::CanFrame::new(id, data).ok_or_else(|| anyhow::format_err!("malformed frame"))
    }
}

impl ServoResponse {
    pub fn from_frame(id: socketcan::Id, frame: socketcan::CanFrame) -> anyhow::Result<Self> {
        use num_traits::FromPrimitive as _;
        use socketcan::EmbeddedFrame as _;

        let data = frame.data();
        let data = Self::check_crc(id, data)?;
        let (&opcode, data) = data
            .split_first()
            .ok_or_else(|| anyhow::format_err!("frame has no opcode"))?;
        let opcode = ServoOpcode::from_u8(opcode)
            .ok_or_else(|| anyhow::format_err!("unrecognized opcode: {:#x}", opcode))?;

        match opcode {
            ServoOpcode::ReadEncoderValueCarry => {
                if let [c0, c1, c2, c3, v0, v1, ..] = *data {
                    return Ok(ServoResponse::ReadEncoderValueCarry {
                        carry: i32::from_be_bytes([c0, c1, c2, c3]),
                        value: u16::from_be_bytes([v0, v1]),
                    });
                }
            }
            ServoOpcode::ReadEncoderValueAddition => {
                if let [v0, v1, v2, v3, v4, v5, ..] = *data {
                    let sign_extend = if v0 > 0x80 { 0xff } else { 0x00 };
                    return Ok(ServoResponse::ReadEncoderValueAddition {
                        value: i64::from_be_bytes([
                            sign_extend,
                            sign_extend,
                            v0,
                            v1,
                            v2,
                            v3,
                            v4,
                            v5,
                        ]),
                    });
                }
            }
            ServoOpcode::ReadSpeed => {
                if let [v0, v1, ..] = *data {
                    return Ok(ServoResponse::ReadSpeed {
                        speed: i16::from_be_bytes([v0, v1]),
                    });
                }
            }
            ServoOpcode::ReadPulses => {
                if let [v0, v1, v2, v3, ..] = *data {
                    return Ok(ServoResponse::ReadPulses {
                        pulses: i32::from_be_bytes([v0, v1, v2, v3]),
                    });
                }
            }
            ServoOpcode::ReadIOPorts => {
                if let [status, ..] = *data {
                    return Ok(ServoResponse::ReadIOPorts {
                        in_1: (status & 0b0001) != 0,
                        in_2: (status & 0b0010) != 0,
                        out_1: (status & 0b0100) != 0,
                        out_2: (status & 0b1000) != 0,
                    });
                }
            }
            ServoOpcode::ReadError => {
                if let [v0, v1, v2, v3, ..] = *data {
                    return Ok(ServoResponse::ReadError {
                        error: i32::from_be_bytes([v0, v1, v2, v3]),
                    });
                }
            }
            ServoOpcode::ReadEnPin => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::ReadEnPin { enabled: v0 != 0 });
                }
            }
            ServoOpcode::ReadGoBackToZeroOnPowerOnStatus => {
                if let [v0, ..] = *data {
                    let status = ProgressStatus::from_u8(v0).ok_or_else(|| {
                        anyhow::format_err!("invalid value for ProgressStatus: {}", v0)
                    })?;
                    return Ok(ServoResponse::ReadGoBackToZeroOnPowerOnStatus { status });
                }
            }
            ServoOpcode::ReleaseMotorShaft => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::ReleaseMotorShaft { success: v0 != 0 });
                }
            }
            ServoOpcode::ReadMotorShaftLockedRotor => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::ReadMotorShaftLockedRotor { locked: v0 != 0 });
                }
            }
            ServoOpcode::Calibrate => {
                if let [v0, ..] = *data {
                    let status = ProgressStatus::from_u8(v0).ok_or_else(|| {
                        anyhow::format_err!("invalid value for ProgressStatus: {}", v0)
                    })?;
                    return Ok(ServoResponse::Calibrate { status });
                }
            }
            ServoOpcode::SetWorkMode => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetWorkMode { success: v0 != 0 });
                }
            }
            ServoOpcode::SetCurrent => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetCurrent { success: v0 != 0 });
                }
            }
            ServoOpcode::SetSubdivision => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetSubdivision { success: v0 != 0 });
                }
            }
            ServoOpcode::SetEnPinActiveMode => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetEnPinActiveMode { success: v0 != 0 });
                }
            }
            ServoOpcode::SetDir => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetDir { success: v0 != 0 });
                }
            }
            ServoOpcode::SetAutoSSD => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetAutoSSD { success: v0 != 0 });
                }
            }
            ServoOpcode::SetMotorShaftLockedRotor => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetMotorShaftLockedRotor { success: v0 != 0 });
                }
            }
            ServoOpcode::SetSubdivisionInterpolation => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetSubdivisionInterpolation { success: v0 != 0 });
                }
            }
            ServoOpcode::SetCanBitRate => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetCanBitRate { success: v0 != 0 });
                }
            }
            ServoOpcode::SetCanId => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetCanId { success: v0 != 0 });
                }
            }
            ServoOpcode::SetCanEnableResponses => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetCanEnableResponses { success: v0 != 0 });
                }
            }
            ServoOpcode::SetKeyLocked => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetKeyLocked { success: v0 != 0 });
                }
            }
            ServoOpcode::SetGroupId => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetGroupId { success: v0 != 0 });
                }
            }
            ServoOpcode::SetHome => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetHome { success: v0 != 0 });
                }
            }
            ServoOpcode::GoHome => {
                if let [v0, ..] = *data {
                    let progress = match v0 {
                        0 => ProgressStatus::Fail,
                        1 => ProgressStatus::Busy,
                        2 => ProgressStatus::Success,
                        n => anyhow::bail!(
                            "invalid value for ProgressStatus (GoHome specific mapping): {}",
                            n
                        ),
                    };
                    return Ok(ServoResponse::GoHome { progress });
                }
            }
            ServoOpcode::SetAxisZero => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetAxisZero { success: v0 != 0 });
                }
            }
            ServoOpcode::SetZeroOnPowerOnMode => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SetZeroOnPowerOnMode { success: v0 != 0 });
                }
            }
            ServoOpcode::RestoreDefaults => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::RestoreDefaults { success: v0 != 0 });
                }
            }
            ServoOpcode::QueryStatus => {
                if let [v0, ..] = *data {
                    let status = if v0 == 0 {
                        None
                    } else {
                        Some(MotorStatus::from_u8(v0).ok_or_else(|| {
                            anyhow::format_err!("invalid value for MotorStatus: {}", v0)
                        })?)
                    };
                    return Ok(ServoResponse::QueryStatus { status });
                }
            }
            ServoOpcode::Enable => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::Enable { success: v0 != 0 });
                }
            }
            ServoOpcode::RunSpeedMode => {
                if let [v0, ..] = *data {
                    let status = MotionStatus::from_u8(v0).ok_or_else(|| {
                        anyhow::format_err!("invalid value for MotionStatus: {}", v0)
                    })?;
                    return Ok(ServoResponse::RunSpeedMode { status });
                }
            }
            ServoOpcode::SaveRunModeParams => {
                if let [v0, ..] = *data {
                    return Ok(ServoResponse::SaveRunModeParams { success: v0 != 0 });
                }
            }
            ServoOpcode::RunPositionRelativePulsesMode => {
                if let [v0, ..] = *data {
                    let status = MotionStatus::from_u8(v0).ok_or_else(|| {
                        anyhow::format_err!("invalid value for MotionStatus: {}", v0)
                    })?;
                    return Ok(ServoResponse::RunPositionRelativePulsesMode { status });
                }
            }
            ServoOpcode::RunPositionRelativeMotionMode => {
                if let [v0, ..] = *data {
                    let status = MotionStatus::from_u8(v0).ok_or_else(|| {
                        anyhow::format_err!("invalid value for MotionStatus: {}", v0)
                    })?;
                    return Ok(ServoResponse::RunPositionRelativeMotionMode { status });
                }
            }
            ServoOpcode::RunPositionAbsoluteMotionMode => {
                if let [v0, ..] = *data {
                    let status = MotionStatus::from_u8(v0).ok_or_else(|| {
                        anyhow::format_err!("invalid value for MotionStatus: {}", v0)
                    })?;
                    return Ok(ServoResponse::RunPositionAbsoluteMotionMode { status });
                }
            }
        }

        anyhow::bail!(
            "response data too short; opcode={:?}, len={}",
            opcode,
            data.len()
        );
    }
    fn check_crc(id: socketcan::Id, data: &[u8]) -> anyhow::Result<&[u8]> {
        let (&crc_actual, rest) = data
            .split_last()
            .ok_or_else(|| anyhow::format_err!("cannot compute CRC for empty frame"))?;
        let crc_expected = compute_crc(id, rest);
        if crc_actual != crc_expected {
            anyhow::bail!(
                "CRC mismatch; expected {} but got {}",
                crc_expected,
                crc_actual
            );
        }
        Ok(rest)
    }
}

fn compute_crc(id: socketcan::Id, data: &[u8]) -> u8 {
    // This is not really Cyclic Redundancy Checking, but the manual calls it CRC, so...
    match id {
        socketcan::Id::Standard(id) => {
            compute_crc_id_bytes(id.as_raw().to_be_bytes().as_slice(), data)
        }
        socketcan::Id::Extended(id) => {
            compute_crc_id_bytes(id.as_raw().to_be_bytes().as_slice(), data)
        }
    }
}

fn compute_crc_id_bytes(id_bytes: &[u8], data: &[u8]) -> u8 {
    // This is not really Cyclic Redundancy Checking, but the manual calls it CRC, so...
    id_bytes
        .iter()
        .copied()
        .chain(data.iter().copied())
        // Cast to wider type to prevent overflow (which is undefined for normal addition)
        .map(|b| b as u16)
        // Cast to u8 to finally truncate result
        .sum::<u16>() as u8
}
