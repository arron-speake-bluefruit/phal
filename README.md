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
