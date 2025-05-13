//! ST smart high-side driver client implementation.

#![no_std]

mod driver;
pub mod vn9e30f;

pub use driver::*;

use device_driver::AsyncRegisterInterface;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiBus;
use embedded_hal_async::spi::SpiDevice;
use embedded_hal_bus::spi::DeviceError;
use rtic_sync::arbiter::spi::ArbiterDevice;

pub struct DriverInterface<'a, BUS, CS, D>
where
    BUS: SpiBus<u8>,
    CS: OutputPin,
    D: DelayNs,
{
    bus: ArbiterDevice<'a, BUS, CS, D>,
    last_global_status: GlobalStatus,
}

impl<'a, BUS, CS, D> DriverInterface<'a, BUS, CS, D>
where
    BUS: SpiBus<u8>,
    CS: OutputPin,
    D: DelayNs,
{
    /// Create a new device interface.
    pub fn new(bus: ArbiterDevice<'a, BUS, CS, D>) -> Self {
        Self {
            bus,
            last_global_status: GlobalStatus::default(),
        }
    }

    /// Last global status received during a transaction.
    pub fn last_global_status(&mut self) -> GlobalStatus {
        self.last_global_status
    }
}

impl<'a, BUS, CS, D> AsyncRegisterInterface for DriverInterface<'a, BUS, CS, D>
where
    BUS: SpiBus<u8>,
    CS: OutputPin,
    D: DelayNs,
{
    type Error = DeviceError<BUS, CS>;
    type AddressType = u8;

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        assert_eq!(size_bits, 16);
        assert_eq!(data.len(), 2);

        let mut dout = [0; 3];
        dout[0] = u8::from(OperatingCode::Read) | address; // header

        let count = dout[0].count_ones() + dout[1].count_ones() + (dout[2] & 0xFE).count_ones();
        let parity = if count % 2 == 0 { 1 } else { 0 };
        dout[2] &= 0xFE;
        dout[2] |= parity;

        let mut din = [0; 3];

        self.bus.transfer(&mut din, &dout).await.unwrap();

        let status = din[0];
        assert_ne!(status, 0xFF, "SDI stuck high");
        assert_ne!(status, 0x00, "SDI stuck low");
        self.last_global_status = GlobalStatus(status);
        assert!(!self.last_global_status.spie(), "SPI error");

        data.clone_from_slice(&din[1..=2]);

        Ok(())
    }

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        assert_eq!(size_bits, 16);
        assert_eq!(data.len(), 2);

        let mut dout = [0; 3];
        dout[0] = u8::from(OperatingCode::Write) | address;
        dout[1..=2].copy_from_slice(data);

        // apply parity
        let count = dout[0].count_ones() + dout[1].count_ones() + (dout[2] & 0xFE).count_ones();
        let parity = if count % 2 == 0 { 1 } else { 0 };
        dout[2] &= 0xFE;
        dout[2] |= parity;

        let mut din = [0; 3];

        self.bus.transfer(&mut din, &dout).await.unwrap();

        let status = din[0];
        assert_ne!(status, 0xFF, "SDI stuck high");
        assert_ne!(status, 0x00, "SDI stuck low");
        self.last_global_status = GlobalStatus(status);

        Ok(())
    }
}

/// Operating code
pub enum OperatingCode {
    /// Write operation
    Write = 0b00,
    /// Read operation
    Read = 0b01,
    /// Read and clear status operation
    ReadAndClearStatus = 0b10,
    /// Read device information
    ReadDeviceInformation = 0b11,
}

impl From<OperatingCode> for u8 {
    fn from(value: OperatingCode) -> Self {
        (value as u8) << 6
    }
}

/// Global status byte
#[derive(Debug, Clone, Copy)]
pub struct GlobalStatus(u8);

#[cfg(feature = "defmt")]
impl defmt::Format for GlobalStatus {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "GlobalStatus {{ fs: {}, oloff: {}, loff: {}, tcase: {}, tsd_otovl: {}, spie: {}, rstb: {}, gsbn: {} }}",
            self.fs(),
            self.oloff(),
            self.loff(),
            self.tcase(),
            self.tsd_otovl(),
            self.spie(),
            self.rstb(),
            self.gsbn(),
        )
    }
}

impl Default for GlobalStatus {
    fn default() -> Self {
        Self(0)
    }
}

impl GlobalStatus {
    /// The bit is set in case the device operates in fail-safe mode. A detailed
    /// description of these root-causes and the fail-safe state itself is
    /// specified in the paragraph “Fail-safe state”
    pub fn fs(&self) -> bool {
        ((1 << 0) & self.0) > 0
    }

    /// The open-load at off state bit is set when an open-load off state or an
    /// output shorted to VCC condition is detected on any channel
    pub fn oloff(&self) -> bool {
        ((1 << 1) & self.0) > 0
    }

    /// The device error bit is set in case when one or more channels are latched OFF
    pub fn loff(&self) -> bool {
        ((1 << 2) & self.0) > 0
    }

    /// This bit is set if the frame temperature is greater than the threshold
    /// and can be used as a temperature prewarning. The bit is cleared
    /// automatically when the frame temperature drops below the
    /// case-temperature reset threshold (TCR).
    pub fn tcase(&self) -> bool {
        ((1 << 3) & self.0) > 0
    }

    /// This bit is set in case of thermal shutdown, power limitation or in case
    /// of high VDS (VDS) at turn-off detected on any channel. The contribution
    /// of high VDS failure is maskable.
    pub fn tsd_otovl(&self) -> bool {
        ((1 << 4) & self.0) > 0
    }

    /// The SPIE is a logical OR combination of errors related to a wrong SPI
    /// communication (SCK count and SDI stuck at errors).
    ///
    /// The SPIE bit is
    /// automatically set when SDI is stuck at High or Low.
    ///
    /// The SPIE is automatically cleared by a valid SPI communication.
    pub fn spie(&self) -> bool {
        ((1 << 5) & self.0) > 0
    }

    /// The RSTB indicates a device reset. In case this bit is set, all internal
    /// control registers are set to default and kept in that state until the
    /// bit is cleared.
    ///
    /// The reset bit is automatically cleared by any valid SPI communication
    pub fn rstb(&self) -> bool {
        ((1 << 6) & self.0) > 0
    }

    /// The GSBN is a logically NOR combination of bit 0 to bit 6. This bit can
    /// also be used as the global status flag without starting a complete
    /// communication frame as it is present directly after pulling CSN low.
    pub fn gsbn(&self) -> bool {
        ((1 << 7) & self.0) > 0
    }
}
