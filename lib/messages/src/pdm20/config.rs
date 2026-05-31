use serde::{Deserialize, Serialize};

pub trait ConfigKey {
    /// Configuration key.
    ///
    /// Must be alpha-numeric characters.
    fn key() -> [u8; 4];
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanBusBitrate {
    pub bitrate: u32,
}

impl Default for CanBusBitrate {
    fn default() -> Self {
        Self { bitrate: 500_000 }
    }
}

impl ConfigKey for CanBusBitrate {
    fn key() -> [u8; 4] {
        *b"CBBR"
    }
}

#[cfg(feature = "device")]
impl<'a> sequential_storage::map::PostcardValue<'a> for CanBusBitrate {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanBusAddress {
    pub address: u8,
}

impl Default for CanBusAddress {
    fn default() -> Self {
        Self { address: 0x50 }
    }
}

impl ConfigKey for CanBusAddress {
    fn key() -> [u8; 4] {
        *b"CBSA"
    }
}

#[cfg(feature = "device")]
impl<'a> sequential_storage::map::PostcardValue<'a> for CanBusAddress {}
