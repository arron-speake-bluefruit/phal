# Phal

[![builds.sr.ht status](https://builds.sr.ht/~cdo/phal.svg)](https://builds.sr.ht/~cdo/phal?)

Phal (Pokes Hardware And Listens) is a reusable component for
end-to-end testing of embedded devices. The rough idea is to have
peripherals of your system-under-test connected to a test rig device
(currently only Odroid XU4s are supported); phal then will provide a
REST interface for these peripherals which your tests can consume.

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

## Example Configuration

```json
{
	"digital-input": {
		"type": "input-pin",
		"chip": "gpa2",
		"line": 6
	},
	"digital-output": {
		"type": "output-pin",
		"chip": "gpx2",
		"line": 0
	},
	"uart": {
		"type": "serial",
		"device": "/dev/ttySAC0",
		"baud_rate": 9600,
		"char-size": 8,
		"parity": "none",
		"stop-bits": 1,
		"flow-control": "none"
	}
}
```
