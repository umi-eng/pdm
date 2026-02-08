#![cfg_attr(not(test), no_std)]

#[cfg(feature = "pdm20")]
pub mod pdm20;
#[cfg(feature = "pdm36")]
pub mod pdm36;

/// Output state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
