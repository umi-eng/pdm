# Changelog

# [Unreleased]

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
