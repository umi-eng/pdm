use pdm::pdm20::Pdm20;
use std::time::Duration;

#[derive(clap::Parser)]
pub struct Cmd {
    /// Delay in seconds.
    #[clap(long)]
    delay: f64,
    /// PWM duty (1.0 = 100%).
    #[clap(long)]
    pwm: f32,
    /// Output state map (4=true 10=false etc.).
    #[clap(last = false)]
    outputs: Vec<String>,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm20) -> anyhow::Result<()> {
        for item in self.outputs.iter() {
            let components: Vec<_> = item.split("=").collect();
            let ch: usize = components[0].parse()?;
            let enabled: bool = components[1].parse()?;
            if enabled {
                pdm.output_econ(ch, Duration::from_secs_f64(self.delay), self.pwm)
                    .await?;
            } else {
                pdm.output_econ_disable(ch).await?;
            }
        }

        Ok(())
    }
}
