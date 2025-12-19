use embassy_stm32::Peri;
use embassy_stm32::gpio::*;

pub struct SingleChannel<'d> {
    pub output: Output<'d>,
    pub fault_reset: Output<'d>,
}

impl<'d> SingleChannel<'d> {
    pub fn new(output: Peri<'d, impl Pin>, fault_reset: Peri<'d, impl Pin>) -> Self {
        Self {
            output: Output::new(output, Level::Low, Speed::Low),
            fault_reset: Output::new(fault_reset, Level::Low, Speed::Low),
        }
    }
}

pub struct DualChannel<'d> {
    pub output1: Output<'d>,
    pub output2: Output<'d>,
    pub fault_reset: Output<'d>,
}

impl<'d> DualChannel<'d> {
    pub fn new(
        output1: Peri<'d, impl Pin>,
        output2: Peri<'d, impl Pin>,
        fault_reset: Peri<'d, impl Pin>,
    ) -> Self {
        Self {
            output1: Output::new(output1, Level::Low, Speed::Low),
            output2: Output::new(output2, Level::Low, Speed::Low),
            fault_reset: Output::new(fault_reset, Level::Low, Speed::Low),
        }
    }
}
