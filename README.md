<img src="assets/banner-light.png#gh-light-mode-only" alt="Power Distribution Module Firmware">
<img src="assets/banner-dark.png#gh-dark-mode-only" alt="Power Distribution Module Firmware">

## Targets

- PDM20 - 4 high-current outputs, 16 low-current outputs, 3 analog inputs.
- PDM36 - 6 high-current outputs, 30 low-current outputs, 3 analog inputs.

## Command Line Tool

A command line tool is provided for configuration, control and updating PDMs.

To install the command line tool from source, run the following:

```shell
cargo install pdm-tool --git https://github.com/umi-eng/pdm --locked
```

## Rust Library

We provide a Rust library for interfacing with PDMs which can be found in the `pdm` crate.

## DBC Files and Messages

A DBC file with definitions for most CAN messages used by the PDM can be found the `messages` crate folder.
