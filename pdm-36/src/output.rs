/// Output driver.
#[derive(Debug, defmt::Format, Clone, Copy, PartialEq, Eq)]
pub enum Driver {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
}

impl From<Driver> for usize {
    fn from(value: Driver) -> Self {
        value as usize
    }
}

impl From<usize> for Driver {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            _ => unreachable!("Driver index out of bounds"),
        }
    }
}

/// Output driver channel.
#[derive(Debug, defmt::Format, Clone, Copy, PartialEq, Eq)]
pub struct Channel(pub usize);

impl Channel {
    pub fn new(n: usize) -> Self {
        assert!(n <= 36);
        Self(n)
    }
}

impl From<Channel> for usize {
    fn from(value: Channel) -> Self {
        value.0
    }
}

/// Output lookup.
pub const OUTPUT_MAP: [(Driver, usize); 36] = [
    (Driver::A, 1),
    (Driver::A, 2),
    (Driver::A, 3),
    (Driver::A, 0),
    (Driver::B, 0),
    (Driver::B, 3),
    (Driver::B, 2),
    (Driver::B, 1),
    (Driver::C, 1),
    (Driver::C, 2),
    (Driver::C, 3),
    (Driver::C, 0),
    (Driver::D, 5),
    (Driver::D, 4),
    (Driver::D, 3),
    (Driver::D, 2),
    (Driver::D, 1),
    (Driver::D, 0),
    (Driver::E, 5),
    (Driver::E, 4),
    (Driver::E, 3),
    (Driver::E, 2),
    (Driver::E, 1),
    (Driver::E, 0),
    (Driver::F, 5),
    (Driver::F, 4),
    (Driver::F, 3),
    (Driver::F, 2),
    (Driver::F, 1),
    (Driver::F, 0),
    (Driver::G, 5),
    (Driver::G, 4),
    (Driver::G, 3),
    (Driver::G, 2),
    (Driver::G, 1),
    (Driver::G, 0),
];

/// Output state.
#[derive(Debug, defmt::Format, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Off = 0,
    On = 1,
    NoChange = 3,
}

impl From<State> for u8 {
    fn from(value: State) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for State {
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
