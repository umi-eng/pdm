use std::time::Duration;

pub struct Pdm36 {
    source_address: u8,
}

pub type Channels = crate::Channels<36>;

impl Pdm36 {
    /// Connect to a PDM36.
    pub fn new(interface: impl Into<String>, source_address: u8) -> Result<Self, ()> {
        Ok(Self { source_address })
    }

    /// Configure one or more output channels.
    pub async fn configure_outputs(
        &self,
        channels: impl Into<Channels>,
        config: OutputConfig,
    ) -> Result<(), ()> {
        let channels: Channels = channels.into();

        let id = j1939::Id::builder()
            .pgn(messages::OUTPUT_CONFIGURE)
            .da(self.source_address)
            .sa(0x00)
            .build();

        let mask1 = channels.0 & 0xFFF;
        let mask2 = (channels.0 >> 12) & 0xFFF;
        let mask3 = (channels.0 >> 24) & 0xFFF;
        let masks = [mask1 as u16, mask2 as u16, mask3 as u16];

        let current_limit = (config.current_limit.clamp(0.0, 25.0) * 10.0) as u8;

        let (econ_duty, econ_delay) = if let Some(econ) = config.economization {
            let duty = (econ.duty.clamp(0.0, 1.0) * 255.0) as u8;
            let delay = (econ.delay.as_secs_f32().clamp(0.05, 0.750) * 20.0) as u8;
            (duty, delay)
        } else {
            (255, 0)
        };

        for (n, mask) in masks.iter().enumerate() {
            if *mask != 0 {
                messages::OutputConfigure::new(
                    n as u8,
                    *mask,
                    config.open_load_detection,
                    config.blanking_window as u8,
                    current_limit,
                    econ_duty,
                    econ_delay,
                )
                .unwrap();

                todo!("actually send frames")
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OutputConfig {
    /// Open load detection feature.
    pub open_load_detection: bool,
    /// Blanking window.
    pub blanking_window: BlankingWindow,
    /// Current limit in amps.
    pub current_limit: f32,
    /// Coil economization setting.
    pub economization: Option<CoilEconomization>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            open_load_detection: false,
            blanking_window: BlankingWindow::LatchOff,
            current_limit: 0.0,
            economization: None,
        }
    }
}

/// Programmable blanking window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlankingWindow {
    LatchOff = 0,
    For32ms = 1,
    For96ms = 2,
    For240ms = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoilEconomization {
    /// Delay before duty cycle is ramped back.
    pub delay: Duration,
    /// PWM duty when economizing.
    pub duty: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn interface() {
        let pdm = Pdm36::new("can0", 0x55).unwrap();

        pdm.configure_outputs(
            Channels::new().range(1..=10).ch(20),
            OutputConfig {
                ..Default::default()
            },
        )
        .await
        .unwrap();
    }
}
