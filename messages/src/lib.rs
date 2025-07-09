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
