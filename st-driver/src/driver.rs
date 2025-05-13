use crate::{
    DriverInterface, GlobalStatus,
    vn9e30f::{ChannelKind, CurrentSamplePoint, PwmFreq, SlopeControl, Vn9e30f},
};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{
    delay::DelayNs,
    spi::{SpiBus, SpiDevice},
};
use embedded_hal_bus::spi::DeviceError;

pub struct Driver<'a, BUS, CS, D>
where
    BUS: SpiBus<u8>,
    CS: OutputPin,
    D: DelayNs,
{
    dev: Vn9e30f<DriverInterface<'a, BUS, CS, D>>,
    channels: usize,
}

impl<'a, BUS, CS, D> Driver<'a, BUS, CS, D>
where
    BUS: SpiBus<u8>,
    CS: OutputPin,
    D: DelayNs,
{
    pub fn new(driver: DriverInterface<'a, BUS, CS, D>, channels: usize) -> Self
    where
        BUS: SpiBus<u8>,
        CS: OutputPin,
        D: DelayNs,
    {
        assert!(channels <= 6, "Only up to 6 channel drivers are supported");

        Self {
            dev: Vn9e30f::new(driver),
            channels,
        }
    }

    /// Number of channels that this driver has.
    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Last global status received from the device.
    pub fn global_status(&self) -> GlobalStatus {
        self.dev.interface.last_global_status
    }

    /// Set all control registers to default.
    pub async fn software_reset(&mut self) -> Result<(), DeviceError<BUS, CS>> {
        self.dev.interface.bus.write(&[0xff, 0, 1]).await.unwrap();
        Ok(())
    }

    /// CLear all status registers.
    pub async fn clear_status(&mut self) -> Result<(), DeviceError<BUS, CS>> {
        self.dev.interface.bus.write(&[0xbf, 0, 0]).await.unwrap();
        Ok(())
    }

    /// Case temperature in Celsius.
    pub async fn tcase(&mut self) -> Result<f32, DeviceError<BUS, CS>> {
        let adc = self.dev.adc_9_sr().read_async().await?.tcase() as f32;
        Ok(401.8 - (1.009 * adc))
    }

    /// Enter normal mode.
    pub async fn enter_normal(&mut self) -> Result<(), DeviceError<BUS, CS>> {
        self.dev.ctrl().write_async(|w| w.set_unlock(true)).await?;
        self.dev
            .ctrl()
            .write_async(|w| {
                w.set_en(true);
                w.set_gostby(false);
            })
            .await?;
        Ok(())
    }

    /// Enter standby mode.
    pub async fn enter_standby(&mut self) -> Result<(), DeviceError<BUS, CS>> {
        self.dev.ctrl().write_async(|w| w.set_unlock(true)).await?;
        self.dev
            .ctrl()
            .write_async(|w| {
                w.set_en(false);
                w.set_gostby(true);
            })
            .await?;
        Ok(())
    }

    /// Toggle watchdog bit.
    pub async fn toggle_watchdog(&mut self) -> Result<(), DeviceError<BUS, CS>> {
        self.dev
            .socr()
            .modify_async(|w| {
                w.set_wdtb(true);
            })
            .await?;
        self.dev
            .socr()
            .modify_async(|w| {
                w.set_wdtb(false);
            })
            .await
    }

    /// Control the internal pull-up current generator to detect open-load in the off state.
    pub async fn output_pull_up(
        &mut self,
        num: usize,
        on: bool,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        self.dev
            .out_ctr_cr(num)
            .modify_async(|w| {
                w.set_oloffcr(on);
            })
            .await
    }

    /// Control an output state and duty cycle.
    pub async fn output(
        &mut self,
        num: usize,
        on: bool,
        duty: u16,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");
        assert!(duty < 1024, "Duty maximum is 1023");

        self.dev
            .socr()
            .modify_async(|w| match num {
                0 => w.set_socr_0(on),
                1 => w.set_socr_1(on),
                2 => w.set_socr_2(on),
                3 => w.set_socr_3(on),
                4 => w.set_socr_4(on),
                5 => w.set_socr_5(on),
                _ => unreachable!(),
            })
            .await?;

        self.dev
            .out_ctr_cr(num)
            .write_async(|w| w.set_duty(duty))
            .await
    }

    pub async fn output_enabled(&mut self, num: usize) -> Result<bool, DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        let result = self.dev.socr().read_async().await?;

        Ok(match num {
            0 => result.socr_0(),
            1 => result.socr_1(),
            2 => result.socr_2(),
            3 => result.socr_3(),
            4 => result.socr_4(),
            5 => result.socr_5(),
            _ => unreachable!(),
        })
    }

    /// Configure the channel kind.
    pub async fn channel_kind(
        &mut self,
        num: usize,
        kind: ChannelKind,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        self.dev
            .out_cfg_r(num)
            .modify_async(|w| w.set_ccr(kind))
            .await
    }

    /// Configure the frequency divisor.
    pub async fn pwm_divisor(
        &mut self,
        num: usize,
        divisor: PwmFreq,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        self.dev
            .out_cfg_r(num)
            .modify_async(|w| w.set_pwmfcy(divisor))
            .await
    }

    /// Current sample point selection.
    pub async fn current_sample_point(
        &mut self,
        num: usize,
        sp: CurrentSamplePoint,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        self.dev
            .out_cfg_r(num)
            .modify_async(|w| w.set_spcr(sp))
            .await
    }

    /// Channel phase.
    pub async fn channel_phase(
        &mut self,
        num: usize,
        phase: u8,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");
        assert!(phase < 32, "Maximum phase value is 32");

        self.dev
            .out_cfg_r(num)
            .modify_async(|w| w.set_chpha(phase))
            .await
    }

    /// Slope control.
    pub async fn slope_control(
        &mut self,
        num: usize,
        slope: SlopeControl,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        self.dev
            .out_cfg_r(num)
            .modify_async(|w| w.set_slopecr(slope))
            .await
    }

    /// Output power limitation behaviour.
    pub async fn off_time(
        &mut self,
        num: usize,
        latch_time: LatchOffTime,
    ) -> Result<(), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        let time = latch_time as u8;

        match num {
            0 => {
                self.dev
                    .chl_off_tcr_0()
                    .modify_async(|w| w.set_chlofftcr_0(time))
                    .await?
            }
            1 => {
                self.dev
                    .chl_off_tcr_0()
                    .modify_async(|w| w.set_chlofftcr_1(time))
                    .await?
            }
            2 => {
                self.dev
                    .chl_off_tcr_0()
                    .modify_async(|w| w.set_chlofftcr_2(time))
                    .await?
            }
            3 => {
                self.dev
                    .chl_off_tcr_1()
                    .modify_async(|w| w.set_chlofftcr_3(time))
                    .await?
            }
            4 => {
                self.dev
                    .chl_off_tcr_1()
                    .modify_async(|w| w.set_chlofftcr_4(time))
                    .await?
            }
            5 => {
                self.dev
                    .chl_off_tcr_1()
                    .modify_async(|w| w.set_chlofftcr_5(time))
                    .await?
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Current sense value.
    pub async fn current_sense(&mut self, num: usize) -> Result<(u16, bool), DeviceError<BUS, CS>> {
        assert!(num < self.channels, "Channel number outside of bounds");

        let read = self.dev.adc_sr(num).read_async().await?;

        Ok((read.adcsr(), read.updtsr()))
    }
}

/// Latch-off time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LatchOffTime {
    LatchOff = 0x0,
    Time16ms = 0x1,
    Time32ms = 0x2,
    Time48ms = 0x3,
    Time64ms = 0x4,
    Time80ms = 0x5,
    Time96ms = 0x6,
    Time112ms = 0x7,
    Time128ms = 0x8,
    Time144ms = 0x9,
    Time160ms = 0xA,
    Time176ms = 0xB,
    Time192ms = 0xC,
    Time208ms = 0xD,
    Time224ms = 0xE,
    Time240ms = 0xF,
}
