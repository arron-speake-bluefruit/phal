# Phal

[![builds.sr.ht status](https://builds.sr.ht/~cdo/phal.svg)](https://builds.sr.ht/~cdo/phal?)

Phal (Pokes Hardware And Listens) is a reusable component for
end-to-end testing of embedded devices. The rough idea is to have
peripherals of your system-under-test connected to a test rig device
(currently only Odroid XU4s are supported); phal then will provide a
REST interface for these peripherals which your tests can consume.

The `example` directory contains a Python test suite using `phal` to
test itself. The intended set-up here is two XU4s with their UARTs
connected, and lines connecting pins GPX2.0 on one to GPA2.6 on the
other and vice versa.

Change the `MASTER` and `SLAVE` constants in the python file to
reflect your setup. Then, running `pytest` in the directory should
run a quick end-to-end test of the system.

## Building

Phal is written in Rust; some of the features used are still unstable
(as of 2020-03-11), so before trying to build make sure you're using
nightly:

```sh
rustup default nightly
```

Then build the package with `cargo`

```sh
cargo build
```

## Contributing

Please report bugs to the [issue tracker](https://todo.sr.ht/~cdo/phal)
and send patches to the [phal-dev mailing list](https://lists.sr.ht/~cdo/phal-dev).
