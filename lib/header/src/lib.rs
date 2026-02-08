#![cfg_attr(not(test), no_std)]

use core::cmp::Ordering;
use zerocopy::CastError;
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::IntoBytes;
use zerocopy::KnownLayout;

const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);

/// Magic number to indicate the start of the header.
pub const HEADER_MAGIC: u32 = 0xB2_87_51_3B;

pub const TARGET_PDM36: [u8; 4] = *b"PD36";
pub const TARGET_PDM20: [u8; 4] = *b"PD20";

pub fn read() -> Result<&'static ImageHeader, Error<'static>> {
    unsafe extern "C" {
        static HEADER: u8;
    }

    let header_ptr = core::ptr::addr_of!(HEADER);
    // SAFETY: The following data doesn't get used until it has passed validation.
    let header_bytes: &[u8] =
        unsafe { core::slice::from_raw_parts(header_ptr, size_of::<ImageHeader>()) };
    header_bytes.try_into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error<'a> {
    Magic,
    Checksum,
    CastError(CastError<&'a [u8], ImageHeader>),
}

#[cfg(feature = "defmt")]
impl defmt::Format for Error<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::Magic => defmt::write!(fmt, "Magic"),
            Self::Checksum => defmt::write!(fmt, "Checksum"),
            Self::CastError(e) => defmt::write!(fmt, "{}", defmt::Debug2Format(&e)),
        }
    }
}

impl<'a> From<CastError<&'a [u8], ImageHeader>> for Error<'a> {
    fn from(value: CastError<&'a [u8], ImageHeader>) -> Self {
        Error::CastError(value)
    }
}

/// Firmware image header.
///
/// Can be used for both bootloader and application images.
#[repr(C)]
#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct ImageHeader {
    magic: u32,
    /// Total firmware image length including this header.
    pub total_image_len: u32,
    /// Firmware version.
    pub fw_version: Version,
    /// Firmware flags.
    pub flags: Flags,
    /// Tag indicating the intended recipient of this image.
    pub target: [u8; 4],
    /// Integrity check for this header.
    checksum: u32,
}

impl ImageHeader {
    pub fn new(total_image_len: u32, fw_version: Version, flags: Flags, target: [u8; 4]) -> Self {
        let mut new = Self {
            magic: HEADER_MAGIC,
            total_image_len,
            fw_version,
            flags,
            target,
            checksum: 0,
        };
        new.checksum = new.calculate_checksum();
        new
    }

    pub fn validate(&self) -> Result<(), Error<'_>> {
        if !self.magic_correct() {
            Err(Error::Magic)
        } else if !self.verify_checksum() {
            Err(Error::Checksum)
        } else {
            Ok(())
        }
    }

    fn magic_correct(&self) -> bool {
        self.magic == HEADER_MAGIC
    }

    /// Calculates the 32-bit one's complement checksum with end-around carry.
    fn calculate_checksum(&self) -> u32 {
        let bytes = self.as_bytes();
        let len = bytes.len();
        let bytes = &bytes[..len - 4]; // exclude the checksum field
        CRC.checksum(bytes)
    }

    fn verify_checksum(&self) -> bool {
        let calculated = self.calculate_checksum();
        calculated == self.checksum
    }
}

impl<'a> TryFrom<&'a [u8]> for &'a ImageHeader {
    type Error = Error<'a>;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let this = ImageHeader::ref_from_bytes(value)?;
        this.validate()?;
        Ok(this)
    }
}

/// Firmware image flags.
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoBytes, FromBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct Flags(u8);

bitflags::bitflags! {
    impl Flags: u8 {
        const BOOTLOADER_IMAGE = 1 << 0;
    }
}

#[repr(C)]
#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
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
            target: *b"TEST",
            flags: Flags::BOOTLOADER_IMAGE,
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
