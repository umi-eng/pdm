pub mod pdm36;

pub use messages::OutputState;

/// Selection of one or more channels.
#[derive(Debug)]
pub struct Outputs<const N: usize>([OutputState; N]);

impl<const N: usize> Outputs<N> {
    /// Create a new empty channel selection.
    pub fn new() -> Self {
        Self([OutputState::NoChange; N])
    }

    /// Add a channel to the selection.
    pub fn ch(mut self, number: usize, state: OutputState) -> Self {
        self.set_ch(number, state);
        self
    }

    /// Set a given channel output state.
    pub fn set_ch(&mut self, number: usize, state: OutputState) {
        assert!(number > 0);
        self.0[number - 1] = state;
    }

    /// Set the state for a range of channels.
    pub fn range<R>(mut self, range: R, state: OutputState) -> Self
    where
        R: IntoIterator<Item = usize>,
    {
        self.set_range(range, state);
        self
    }

    pub fn set_range<R>(&mut self, range: R, state: OutputState)
    where
        R: IntoIterator<Item = usize>,
    {
        for n in range {
            assert!(n > 0);
            self.0[n - 1] = state;
        }
    }

    /// Returns a slice containing the entire array.
    pub fn as_slice(&self) -> &[OutputState] {
        &self.0
    }

    /// Returns a mutable slice containing the entire array.
    pub fn as_mut_slice(&mut self) -> &mut [OutputState] {
        &mut self.0
    }
}

impl<const N: usize> Default for Outputs<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs() {
        let simple = Outputs::<5>::new()
            .ch(1, OutputState::On)
            .ch(3, OutputState::Off)
            .ch(5, OutputState::NoChange);
        assert_eq!(
            simple.as_slice(),
            &[
                OutputState::On,
                OutputState::NoChange,
                OutputState::Off,
                OutputState::NoChange,
                OutputState::NoChange
            ]
        );

        let mixed = Outputs::<6>::new()
            .ch(2, OutputState::On)
            .range(4..=6, OutputState::Off);
        assert_eq!(
            mixed.as_slice(),
            &[
                OutputState::NoChange,
                OutputState::On,
                OutputState::NoChange,
                OutputState::Off,
                OutputState::Off,
                OutputState::Off,
            ]
        );
    }
}
