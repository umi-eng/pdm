#![cfg_attr(not(test), no_std)]

use core::marker::Copy;
use core::ops::{Add, Div, Mul};

/// Voltage divider Vin from Vout.
pub fn divider_vin<T>(r1: T, r2: T, vout: T) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    vout * (r1 + r2) / r2
}

/// Voltage divider Vout from Vin.
pub fn divider_vout<T>(r1: T, r2: T, vin: T) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    (vin * r2) / (r1 + r2)
}

/// Get the voltage read by the ADC.
pub fn count_to_volts<T>(vref: T, max: T, count: T) -> T
where
    T: Mul<Output = T> + Div<Output = T> + Copy,
{
    (count / max) * vref
}

#[derive(Debug, Clone)]
pub struct MovingAvg<T, const N: usize>(heapless::HistoryBuffer<T, N>);

impl<const N: usize> MovingAvg<f32, N> {
    /// Create a new moving average buffer.
    pub fn new() -> Self {
        Self(heapless::HistoryBuffer::new())
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
    fn voltage_divider() {
        let vout = divider_vin(100.0, 10.0, 5.0);
        assert_eq!(vout, 55.0);

        let vin = divider_vout(100.0, 10.0, 55.0);
        assert_eq!(vin, 5.0);
    }

    #[test]
    fn counts_to_volts() {
        let volts = count_to_volts(3.3, 1023.0, 1023.0);
        assert_eq!(volts, 3.3);

        let volts = count_to_volts(3.3, 1023.0, 0.0);
        assert_eq!(volts, 0.0);
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
