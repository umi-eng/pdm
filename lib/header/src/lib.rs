#![cfg_attr(not(test), no_std)]

use core::cmp::Ordering;
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::IntoBytes;
use zerocopy::KnownLayout;

/// Magic number to indicate the start of the header.
pub const HEADER_MAGIC: u32 = 0xB2_87_51_3B;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    Magic,
    Checksum,
}

/// Firmware image header.
///
/// Can be used for both bootloader and application images.
#[repr(C)]
#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct ImageHeader {
    pub magic: u32,
    /// Total firmware image length including this header.
    pub total_image_len: u32,
    /// Firmware version.
    pub fw_version: Version,
    /// Firmware flags.
    pub flags: Flags,
    /// Integrity check for this header.
    pub checksum: u32,
}

impl ImageHeader {
    pub fn validate(&self) -> Result<(), Error> {
        if !self.magic_correct() {
            Err(Error::Magic)
        } else if !self.verify_checksum() {
            Err(Error::Checksum)
        } else {
            Ok(())
        }
    }

    pub fn magic_correct(&self) -> bool {
        self.magic == HEADER_MAGIC
    }

    /// Calculates the 32-bit one's complement checksum with end-around carry.
    pub fn calculate_checksum(&self) -> u32 {
        let bytes = self.as_bytes();
        let len = bytes.len();
        let bytes = &bytes[..len - 4]; // exclude the checksum field

        let mut sum: u32 = 0;
        for chunk in bytes.chunks(4) {
            let word = match chunk.len() {
                4 => u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
                3 => {
                    ((chunk[0] as u32) << 24) | ((chunk[1] as u32) << 16) | ((chunk[2] as u32) << 8)
                }
                2 => ((chunk[0] as u32) << 24) | ((chunk[1] as u32) << 16),
                1 => (chunk[0] as u32) << 24,
                _ => unreachable!("zero chunk length"),
            };

            sum = sum.wrapping_add(word);
        }

        !sum
    }

    pub fn verify_checksum(&self) -> bool {
        let calculated = self.calculate_checksum();
        calculated == self.checksum
    }
}

/// Firmware image flags.
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoBytes, FromBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct Flags(u8);

bitflags::bitflags! {
    impl Flags: u8 {
        const FIRMWARE_IMAGE = 1 << 0;
        const BOOTLOADER_IMAGE = 1 << 1;
    }
}

#[repr(C)]
#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => self.patch.cmp(&other.patch),
                other => other,
            },
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_checksum() {
        let mut header = ImageHeader {
            magic: HEADER_MAGIC,
            total_image_len: 1024,
            fw_version: Version {
                major: 0,
                minor: 1,
                patch: 2,
            },
            flags: Flags::FIRMWARE_IMAGE,
            checksum: 0,
        };

        assert!(!header.verify_checksum());

        header.checksum = header.calculate_checksum();

        assert!(header.verify_checksum());
    }

    #[test]
    fn version_ordering() {
        let v1_0_0 = Version {
            major: 1,
            minor: 0,
            patch: 0,
        };
        let v1_0_1 = Version {
            major: 1,
            minor: 0,
            patch: 1,
        };
        let v1_1_0 = Version {
            major: 1,
            minor: 1,
            patch: 0,
        };
        let v2_0_0 = Version {
            major: 2,
            minor: 0,
            patch: 0,
        };

        assert!(v1_0_0 < v1_0_1);
        assert!(v1_0_1 < v1_1_0);
        assert!(v1_1_0 < v2_0_0);
        assert!(
            v1_0_0
                == Version {
                    major: 1,
                    minor: 0,
                    patch: 0
                }
        );
    }
}
