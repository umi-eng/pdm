use crate::Item;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Hardware semantic-version.
#[derive(Debug, FromBytes, IntoBytes, Immutable, Unaligned, KnownLayout)]
#[cfg_attr(feature = "std", derive(facet::Facet))]
#[repr(C)]
pub struct HardwareVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[cfg(feature = "defmt")]
impl defmt::Format for HardwareVersion {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "v{}.{}.{}", self.major, self.minor, self.patch);
    }
}

impl Item for HardwareVersion {
    fn tag() -> [u8; 4] {
        *b"HW  "
    }
}

/// Part number.
#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[cfg_attr(feature = "std", derive(facet::Facet))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct PartNumber(pub [u8; 5]);

impl PartNumber {
    pub fn is_pdm36(&self) -> bool {
        self.0 == *b"PDM36"
    }
}

impl Item for PartNumber {
    fn tag() -> [u8; 4] {
        *b"PART"
    }
}

/// Serial number.
#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[cfg_attr(feature = "std", derive(facet::Facet))]
#[repr(C)]
pub struct SerialNumber {
    pub year: u8,
    pub week: u8,
    pub sequence: u16,
}

#[cfg(feature = "defmt")]
impl defmt::Format for SerialNumber {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:02}{:02}-{:X}", self.year, self.week, self.sequence);
    }
}

impl Item for SerialNumber {
    fn tag() -> [u8; 4] {
        *b"SERN"
    }
}

/// Public key for signing firmware.
#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PubKey {
    pub key: [u8; 32],
}

impl Item for PubKey {
    fn tag() -> [u8; 4] {
        *b"PUBK"
    }
}

/// Check type sizes at compile time.
const _CHECK_SIZE: () = {
    assert!(size_of::<HardwareVersion>() == 3);
    assert!(size_of::<PartNumber>() == 5);
    assert!(size_of::<SerialNumber>() == 4);
    assert!(size_of::<PubKey>() == 32);
};
