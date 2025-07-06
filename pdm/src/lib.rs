pub mod pdm36;

/// Selection of one or more channels.
#[derive(Debug)]
pub struct Channels<const N: usize>(pub(crate) u64);

impl<const N: usize> Channels<N> {
    /// Create a new empty channel selection.
    pub fn new() -> Self {
        Self(0)
    }

    /// Add a channel to the selection.
    pub fn ch(mut self, number: usize) -> Self {
        assert!(number > 0);

        let number = number as u64 - 1;
        self.0 |= 1 << number;

        self
    }

    pub fn range<R>(mut self, range: R) -> Self
    where
        R: IntoIterator<Item = usize>,
    {
        for ch in range {
            assert!(ch > 0, "Channel numbers must be greater than 0");
            assert!(
                ch <= N,
                "Channel number {} exceeds maximum channel count {}",
                ch,
                N
            );

            let ch = ch as u64 - 1;
            self.0 |= 1 << ch;
        }

        self
    }
}

impl<const N: usize> Default for Channels<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> From<usize> for Channels<N> {
    fn from(value: usize) -> Self {
        Self::new().ch(value)
    }
}

impl<const N: usize> IntoIterator for Channels<N> {
    type Item = usize;
    type IntoIter = ChannelsIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        ChannelsIterator {
            channels: self.0,
            position: 0,
        }
    }
}

pub struct ChannelsIterator<const N: usize> {
    channels: u64,
    position: u8,
}

impl<const N: usize> Iterator for ChannelsIterator<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < N as u8 {
            let current_position = self.position;
            self.position += 1;

            if (self.channels & (1 << current_position)) != 0 {
                // Add 1 to convert from 0-based to 1-based channel numbering
                return Some((current_position + 1) as usize);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_iterator() {
        let channels = Channels::<36>::new().ch(1).ch(3).ch(5);
        let channel_vec: Vec<usize> = channels.into_iter().collect();
        assert_eq!(channel_vec, vec![1, 3, 5]);

        let mixed_channels = Channels::<36>::new().ch(2).range(4..=6);
        let mixed_vec: Vec<usize> = mixed_channels.into_iter().collect();
        assert_eq!(mixed_vec, vec![2, 4, 5, 6]);
    }
}
