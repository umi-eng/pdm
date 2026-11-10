MEMORY
{
/* Bank 1 */
  FLASH             : ORIGIN = 0x08000000, LENGTH = 32K
  BOOTLOADER_STATE  : ORIGIN = 0x08008000, LENGTH = 4K
  ACTIVE            : ORIGIN = 0x08009000, LENGTH = 192K
  /* 28K at the end of bank 1 for calibration constants. */
  CAL               : ORIGIN = 0x08039000, LENGTH = 28K
  /* Bank 2 */
  DFU               : ORIGIN = 0x08040000, LENGTH = 194K
  CFG               : ORIGIN = 0x08070800, LENGTH = 62K

  /* Main SRAM (SRAM1/SRAM2) */
  RAM (rwx)         : ORIGIN = 0x20000000, LENGTH = 96K
  /* CCM SRAM */
  CCMRAM            : ORIGIN = 0x10000000, LENGTH = 32K
}

__bootloader_state_start = ORIGIN(BOOTLOADER_STATE) - ORIGIN(FLASH);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE) - ORIGIN(FLASH);

__bootloader_active_start = ORIGIN(ACTIVE) - ORIGIN(FLASH);
__bootloader_active_end = ORIGIN(ACTIVE) + LENGTH(ACTIVE) - ORIGIN(FLASH);

/* The DFU partition is in bank 2, so its offsets are relative to the bank 2
   base (the DFU region origin) rather than the flash base. */
__bootloader_dfu_start = ORIGIN(DFU) - ORIGIN(DFU);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU) - ORIGIN(DFU);
