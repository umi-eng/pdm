use crate::signal::{Param8, Param16, Signal};
use num::{FromPrimitive, cast::AsPrimitive};

pub trait Slot<T: Signal>: Sized {
    /// Unit of measurement.
    const UNIT: &str;
    /// Value offset.
    const OFFSET: f32 = 0.0;
    /// Value scale factor.
    const SCALE: f32;

    /// Create a new instance of this slot from the underlying parameter.
    fn new(parameter: T) -> Self;

    /// Get the underlying paramter from this slot.
    fn parameter(&self) -> T;

    /// Try converting from an f32.
    fn from_f32(value: f32) -> Option<Self> {
        let value = (value - Self::OFFSET) / Self::SCALE;
        let value = T::Base::from_f32(value)?;
        let parameter = T::from_raw(value)?;
        Some(Self::new(parameter))
    }

    /// Try converting to an f32.
    fn as_f32(&self) -> Option<f32> {
        let parameter = self.parameter();
        let value: u32 = parameter.value()?.as_();
        let value = (value as f32 + Self::OFFSET) * Self::SCALE;
        Some(value)
    }
}

#[macro_export]
macro_rules! slot_impl {
    ($type:ident, $param:ident, $offset:expr, $scale:expr, $unit:expr, $comment:expr) => {
        #[doc = $comment]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $type($param);

        impl Slot<$param> for $type {
            const UNIT: &str = $unit;
            const OFFSET: f32 = $offset;
            const SCALE: f32 = $scale;

            fn new(parameter: $param) -> Self {
                Self(parameter)
            }

            fn parameter(&self) -> $param {
                self.0
            }
        }
    };
}

slot_impl!(
    SaeTP01,
    Param8,
    -40.0,
    1.0,
    "°C",
    "Temperature - 1 °C per bit"
);

slot_impl!(
    SaeEV06,
    Param16,
    0.0,
    0.001,
    "V",
    "Voltage - 0.001 V per bit"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_sae_tp01() {
        let slot = SaeTP01::from_f32(210.0).unwrap();
        assert_eq!(slot.parameter().value().unwrap(), 250);
        assert_eq!(slot.as_f32(), Some(210.0));

        let slot = SaeTP01::from_f32(-40.0).unwrap();
        assert_eq!(slot.parameter().value().unwrap(), 0);
        assert_eq!(slot.as_f32(), Some(-40.0));

        let slot = SaeTP01::from_f32(0.0).unwrap();
        assert_eq!(slot.parameter().value().unwrap(), 40);
        assert_eq!(slot.as_f32(), Some(0.0));
    }

    #[test]
    fn slot_sae_ev06() {
        let slot = SaeEV06::from_f32(0.0).unwrap();
        assert_eq!(slot.parameter().value().unwrap(), 0);
        assert_eq!(slot.as_f32(), Some(0.0));

        // "rounded" to the nearest representable float
        let slot = SaeEV06::from_f32(24.000002).unwrap();
        assert_eq!(slot.parameter().value().unwrap(), 24000);
        assert_eq!(slot.as_f32(), Some(24.000002));

        // "rounded" to the nearest representable float
        let slot = SaeEV06::from_f32(64.225006).unwrap();
        assert_eq!(slot.parameter().value().unwrap(), 64225);
        assert_eq!(slot.as_f32(), Some(64.225006));
    }
}
