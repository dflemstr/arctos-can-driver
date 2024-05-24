# `arctos-can-driver`

A simple CLI for managing an Arctos robotic arm over a CAN-bus network.  It is assumed that all motors are controlled by MKS SERVO 42D/57D, and optionally that there are additional encoders controlled via my [sensor board](https://github.com/dflemstr/canbus-sensor-board).

New features are constantly being added and the below summary might be incomplete.

Summary of the CLI interface (using `--help`):

```
A simple controller for an Arctos robot arm using canbus

Usage: arctos-can-driver [OPTIONS] <COMMAND>

Commands:
  axes  
  help  Print this message or the help of the given subcommand(s)

Options:
  -i, --ifname <IFNAME>  Interface name for the CAN network to use [default: can0]
  -h, --help             Print help
  -V, --version          Print version
```

Summary of the `axes` subcommand:

```
Usage: arctos-can-driver axes [OPTIONS] <COMMAND>

Commands:
  init           Initialize (configure settings for) axis motors
  enable         Enable (power on) axis motors
  set-origin     Set the origin of the specified axes to whatever the current position of the robot is
  get-motor-pos  Get the current axis positions, from the point of view of the motor(s)
  set-motor-pos  Set the axis positions, from the point of view of the motor(s)
  help           Print this message or the help of the given subcommand(s)

Options:
  -a, --all          
      --axes <AXES>  [possible values: x, y, z, a, b, c]
  -h, --help         Print help
```
