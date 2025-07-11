MEMORY
{
  FLASH             : ORIGIN = 0x08000000, LENGTH = 24K
  BOOTLOADER_STATE  : ORIGIN = 0x08006000, LENGTH = 8K
  ACTIVE            : ORIGIN = 0x08008000, LENGTH = 192K
  DFU               : ORIGIN = 0x08038000, LENGTH = 196K
  /* 28K (7*4K) reserved */
  CFG               : ORIGIN = 0x08060000, LENGTH = 64K
  RAM (rwx)         : ORIGIN = 0x20000000, LENGTH = 16K
}

__bootloader_state_start = ORIGIN(BOOTLOADER_STATE) - ORIGIN(FLASH);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE) - ORIGIN(FLASH);

__bootloader_active_start = ORIGIN(ACTIVE) - ORIGIN(FLASH);
__bootloader_active_end = ORIGIN(ACTIVE) + LENGTH(ACTIVE) - ORIGIN(FLASH);

__bootloader_dfu_start = ORIGIN(DFU) - ORIGIN(FLASH);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU) - ORIGIN(FLASH);
