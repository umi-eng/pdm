# Changelog

# [Unreleased]

# v0.1.3

- Fix possible underflow when calculating end of firmware position.
- Ignore RTR frames.
- Add wait after CAN error to avoid locking up for too long.
- Fix backend being thumbv6 instead of thumbv7.
- Remove use of `delay_until` and prefer simple `delay`.
- Use 32-bit SysTick timer.
- Show hardware version and serial number on startup.

# v0.1.2

## Added

- Fixed soft limits for low and high-current outputs.
- Automatic latch-off recovery.

## Changed

- Replace local `st-driver` crate with with external repo version.

# v0.1.1

## Changed

- Show error instead of panic when reading public key from VPD.
- Show error instead of panic when CAN bus error occurs.
- Respond with transport error when firmware block write fails.
- Show error when marking booted fails.
- Show error instead of panic when reading DFU signature fails.

# [v0.1.0](https://github.com/umi-eng/pdm/releases/tag/pdm36-v0.1.0)

Initial version.
