# PDM36 Bootloader

The embassy-boot framework is used to provide a firmware update process that is both resilient to power-failure and corruption with signed firmware binaries.

## Signing

For a firmware binary to be accepted by the application firmware update subsystem, it must end with a 64-byte signature which is created using an ed25519 private key. The public key to verify this signature is programmed into the OTP memory of the STM as part of the vital product data.

## Rollback and Watchdog

When the bootloader starts, it configures the independant watchdog of the STM32 which will continue to run even after jumping to the application. After application startup is complete, it will mark the boot as sucessful and continue to feed the watchdog as normal. If the boot isn't marked as sucessful before the watchdog trips, the STM32 will reset and the bootloader will roll back to the previous firmware image.

## Bootloader Upgrade

Right now, the bootloader can only be upgraded by attaching a debug probe and writing a new binary to the flash region. The [STLINK-V3MINIE](https://www.st.com/en/development-tools/stlink-v3minie.html) is a good affordable debug proeb that can be used to do this.

Run the following probe-rs command to flash a new bootloader image:

```shell
probe-rs download \
  --chip STM32G474CE \
  --base-address 0x08000000 \
  --binary-format bin \
  pdm-36-bootloader-v0.1.1.bin
```
