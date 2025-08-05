# PDM Platform

## Firmware

Firmware for the PDM36 can be found in the `pdm-36` crate and the bootloader in the `pdm-36-bootloader` crate.

For more details on the firmware, see the readme within the `pdm-36` crate.

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
