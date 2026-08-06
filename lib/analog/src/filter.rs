use num_traits::Float;

/// Exponential moving average filter.
pub struct ExpMovingAvg<T: Float> {
    alpha: T,
    value: Option<T>,
}

impl<T: Float> ExpMovingAvg<T> {
    pub fn new(alpha: T) -> Self {
        assert!(
            alpha > T::zero() && alpha <= T::one(),
            "alpha must be in (0, 1]"
        );
        Self { alpha, value: None }
    }

    pub fn update(&mut self, sample: T) -> T {
        let new_value = match self.value {
            Some(prev) => self.alpha * sample + (T::one() - self.alpha) * prev,
            None => sample,
        };
        self.value = Some(new_value);
        new_value
    }

    pub fn value(&self) -> Option<T> {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = None;
    }
}

#[derive(Debug, Clone)]
pub struct MovingAvg<T, const N: usize>(heapless::HistoryBuf<T, N>);

impl<const N: usize> MovingAvg<f32, N> {
    /// Create a new moving average buffer.
    pub fn new() -> Self {
        Self(heapless::HistoryBuf::new())
    }

    /// Add a new value to the history buffer.
    pub fn push(&mut self, value: f32) {
        self.0.write(value);
    }

    /// Get the moving average.
    ///
    /// Only returns [`Some`] if one or more elements are in the buffer.
    pub fn avg(&self) -> Option<f32> {
        if self.0.is_empty() {
            return None;
        }

        let mut value = 0.0;
        for el in self.0.iter() {
            value += el;
        }

        Some(value / self.0.len() as f32)
    }
}

impl<const N: usize> Default for MovingAvg<f32, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential_moving_average() {
        let mut avg = ExpMovingAvg::<f32>::new(1.0);
        assert!(avg.value().is_none());

        // comparing floats is not great, but good enough to smoke test this.
        avg.update(0.0);
        assert_eq!(avg.value(), Some(0.0));
        avg.update(1.0);
        assert_eq!(avg.value(), Some(1.0));
        avg.update(0.5);
        assert_eq!(avg.value(), Some(0.5));
    }

    #[test]
    fn moving_average() {
        let mut avg = MovingAvg::<f32, 32>::new();
        assert!(avg.avg().is_none());

        // comparing floats is not great, but good enough to smoke test this.
        avg.push(0.0);
        assert_eq!(avg.avg(), Some(0.0));
        avg.push(1.0);
        assert_eq!(avg.avg(), Some(0.5));
    }
}
