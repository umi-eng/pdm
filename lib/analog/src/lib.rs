#![cfg_attr(not(test), no_std)]

pub mod filter;

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
}
