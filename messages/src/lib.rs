#![cfg_attr(not(test), no_std)]

#[allow(warnings)]
mod messages;
pub use messages::*;

use saelient::Pgn;

/// Output control messages.
pub const CONTROL: Pgn = Pgn::ProprietaryA;
/// Configuration messages.
pub const CONFIGURE: Pgn = Pgn::ProprietaryA2;
/// Power-on reset message.
pub const STARTUP: Pgn = Pgn::ProprietaryB(0x00);
/// System status message.
pub const SYSTEM_STATUS: Pgn = Pgn::ProprietaryB(0x12);
/// Current sense message.
pub const CURRENT_SENSE: Pgn = Pgn::ProprietaryB(0x10);
/// Analog readings message.
pub const ANALOG_READINGS: Pgn = Pgn::ProprietaryB(0x11);

/// Output state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum OutputState {
    Off = 0,
    On = 1,
    NoChange = 3,
}

impl From<OutputState> for u8 {
    fn from(value: OutputState) -> Self {
        value as u8
    }
}

impl From<bool> for OutputState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::On,
            false => Self::Off,
        }
    }
}

impl TryFrom<u8> for OutputState {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::On),
            3 => Ok(Self::NoChange),
            _ => Err(value),
        }
    }
}
