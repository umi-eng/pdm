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

/// Output channel kind.
#[derive(Debug, defmt::Format, Clone, Copy, PartialEq, Eq)]
pub enum ChanKind {
    /// Low current.
    Low,
    /// High current.
    High,
}

/// Output lookup.
pub const OUTPUT_MAP: [(Driver, usize, ChanKind); 36] = [
    (Driver::A, 1, ChanKind::High),
    (Driver::A, 2, ChanKind::Low),
    (Driver::A, 3, ChanKind::Low),
    (Driver::A, 0, ChanKind::High),
    (Driver::B, 0, ChanKind::High),
    (Driver::B, 3, ChanKind::Low),
    (Driver::B, 2, ChanKind::Low),
    (Driver::B, 1, ChanKind::High),
    (Driver::C, 1, ChanKind::High),
    (Driver::C, 2, ChanKind::Low),
    (Driver::C, 3, ChanKind::Low),
    (Driver::C, 0, ChanKind::High),
    (Driver::D, 5, ChanKind::Low),
    (Driver::D, 4, ChanKind::Low),
    (Driver::D, 3, ChanKind::Low),
    (Driver::D, 2, ChanKind::Low),
    (Driver::D, 1, ChanKind::Low),
    (Driver::D, 0, ChanKind::Low),
    (Driver::E, 5, ChanKind::Low),
    (Driver::E, 4, ChanKind::Low),
    (Driver::E, 3, ChanKind::Low),
    (Driver::E, 2, ChanKind::Low),
    (Driver::E, 1, ChanKind::Low),
    (Driver::E, 0, ChanKind::Low),
    (Driver::F, 5, ChanKind::Low),
    (Driver::F, 4, ChanKind::Low),
    (Driver::F, 3, ChanKind::Low),
    (Driver::F, 2, ChanKind::Low),
    (Driver::F, 1, ChanKind::Low),
    (Driver::F, 0, ChanKind::Low),
    (Driver::G, 5, ChanKind::Low),
    (Driver::G, 4, ChanKind::Low),
    (Driver::G, 3, ChanKind::Low),
    (Driver::G, 2, ChanKind::Low),
    (Driver::G, 1, ChanKind::Low),
    (Driver::G, 0, ChanKind::Low),
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
