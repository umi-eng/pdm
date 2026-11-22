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
pub struct Board(pub [u8; 4]);

impl Item for Board {
    fn tag() -> [u8; 4] {
        *b"BRD "
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
        defmt::write!(
            fmt,
            "{:02}{:02}-{:04X}",
            self.year,
            self.week,
            self.sequence
        );
    }
}

impl Item for SerialNumber {
    fn tag() -> [u8; 4] {
        *b"SERN"
    }
}

/// Check type sizes at compile time.
const _CHECK_SIZE: () = {
    assert!(size_of::<HardwareVersion>() == 3);
    assert!(size_of::<Board>() == 4);
    assert!(size_of::<SerialNumber>() == 4);
};
