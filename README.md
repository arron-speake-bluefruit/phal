# Phal

[![builds.sr.ht status](https://builds.sr.ht/~cdo/phal.svg)](https://builds.sr.ht/~cdo/phal?)

Phal (Pokes Hardware And Listens) is a reusable component for
end-to-end testing / remote control of embedded devices, written in
[Rust](https://www.rust-lang.org/). The rough idea is to have
peripherals of your device connected to another device running phal;
you then get a configurable REST interface to .

## Development Environment

If you use [Nix](https://nixos.org/), `nix-shell` should land you at a
shell with everything you need.

Alternatively, you can manually install the prequisites:

- [Rust](https://rustup.rs/), with Cargo and `rustfmt`
- [Python 3](https://www.python.org/), with the pytest and requests
  libraries (only for the end-to-end test)

## Building

```sh
cargo build
cargo test
```

## Usage

Do `cargo run --release` to start the server on port 8000. The server
is configured by making HTTP POST requests to `/config`. The
configuration file must be a JSON object, with members containing
their own specific configuration. For example, the body of such a
request could be:

```json
{
  "my_first_limb": { ... },
  "limb_2": { ... },
  "another": { ... }
}
```

Each limb must be one of a few types, with their own configurations,
these are:

### Pin

A pin can either have type `input-pin` or `output-pin`, where it can
either be read or written to, respectively.

To read an input pin called `hello`, make a GET request to
`/limb/hello`.

To set the state of an output pin called `something`, make a POST
request to `/limb/something` with either `High` or `Low` in the body.

```json
{
  "gpio_10": {
      "type": "input-pin",
      "chip": "/dev/gpiochip0",
      "line": 10,
      "pin-type": "push-pull"
  },
  "gpio_13": {
      "type": "output-pin",
      "chip": "/dev/gpiochip0",
      "line": 13,
      "pin-type": "push-pull"
  }
}
```

### Serial

To read from a serial interface called `s`, make a GET request to
`/limb/s`. To write to it, POST to `/limb/s` with content in the
request's body.

```json
{
  "serial": {
    "type": "serial",
    "device": "/dev/ttyUSB0",
    "baud-rate": 9600,
    "char-size": 8,
    "parity": "none",
    "stop-bits": 1,
    "flow-control": "none"
  }
}
```

### XMODEM

XMODEM limbs internally function identically to serial interfaces,
but act as XMODEM transmitters instead of raw serial interfaces.

To send a file over an XMODEM interface make a POST request with the
filename of the file to be transmitted. This must be a file on the
remote host (the machine running PHAL).

Note: An XMODEM and Serial limb cannot both be configured for the
same device.

```json
{
  "my_xmodem": {
    "type": "xmodem",
    "device": "/dev/ttyUSB0",
    "baud-rate": 9600,
    "char-size": 8,
    "parity": "none",
    "stop-bits": 1,
    "flow-control": "none"
  }
}
```

## Info

The configuration of the server can be queried by making GET requests
to `/info/types` to return available limb types, and `/info/limbs` to
return currently registered limbs, along with their type.

## End-to-End Test

The `self-test` directory contains a Python test suite using phal to
test itself. The intended set-up here is two Odroids XU4s with their
UARTs connected, and lines connecting pins GPX2.0 on one to GPA2.6 on
the other and vice versa. See [Odroid XU4 GPIO](odroid-xu4-gpio.md)
for more information on using the XU4's GPIO pins.

Change the `IP_A` and `IP_B` constants in the python file to the IPs
of your running test rigs. Then, running `pytest` in the directory
should run a quick end-to-end test of the system.

## Contributing

Please report bugs to the [issue tracker](https://todo.sr.ht/~cdo/phal)
and send patches to the [phal-dev mailing list](https://lists.sr.ht/~cdo/phal-dev).
