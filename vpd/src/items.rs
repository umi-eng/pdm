use crate::Item;
use serde::Deserialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Hardware semantic-version.
#[derive(Debug, FromBytes, IntoBytes, Immutable, Unaligned, KnownLayout, Deserialize)]
#[repr(C, packed)]
#[cfg_attr(feature = "defmt-1", defmt::Format)]
pub struct HardwareVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Item for HardwareVersion {
    fn tag() -> [u8; 4] {
        *b"HW  "
    }
}

/// Serial number.
#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout, Deserialize)]
#[repr(C)]
#[cfg_attr(feature = "defmt-1", defmt::Format)]
pub struct SerialNumber {
    pub year: u8,
    pub week: u8,
    pub sequence: u16,
}

impl Item for SerialNumber {
    fn tag() -> [u8; 4] {
        *b"SN  "
    }
}

/// Public key for signing firmware.
#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
#[cfg_attr(feature = "defmt-1", defmt::Format)]
pub struct PubKey {
    pub key: [u8; 32],
}

impl Item for PubKey {
    fn tag() -> [u8; 4] {
        *b"PUBK"
    }
}
