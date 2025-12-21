#[allow(warnings)]
mod messages;
pub use messages::*;

pub mod pgn {
    use saelient::Pgn;

    /// Output control messages.
    pub const CONTROL: Pgn = Pgn::ProprietaryA;
    /// Power-on reset message.
    pub const STARTUP: Pgn = Pgn::ProprietaryB(0x00);
    /// System status message.
    pub const STATUS: Pgn = Pgn::ProprietaryB(0x10);
    /// Analog readings message.
    pub const ANALOG: Pgn = Pgn::ProprietaryB(0x11);
}
