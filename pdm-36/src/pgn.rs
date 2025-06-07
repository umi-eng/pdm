///! PGNs used by J1939 identifiers.
use j1939::id::Pgn;

/// Output control message.
pub const OUTPUT_CONTROL: Pgn = Pgn::ProprietaryA;

pub const OUTPUT_CONFIGURE: Pgn = Pgn::ProprietaryA2;

/// Power-on reset message.
pub const POWER_ON_RESET: Pgn = Pgn::ProprietaryB(0x00);

/// System status message.
pub const SYSTEM_STATUS: Pgn = Pgn::ProprietaryB(0x12);

/// Current sense message.
pub const CURRENT_SENSE: Pgn = Pgn::ProprietaryB(0x10);

/// Analog readings message.
pub const ANALOG_READINGS: Pgn = Pgn::ProprietaryB(0x11);
