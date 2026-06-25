use pdm::{Outputs, pdm36::Pdm36};

#[derive(clap::Parser)]
pub struct Cmd {
    /// PWM duty (1.0 = 100%).
    #[clap(long)]
    pwm: Option<f32>,
    /// Output state map (4=true 10=false etc.).
    #[clap(last = false)]
    outputs: Vec<String>,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm36) -> anyhow::Result<()> {
        let mut outputs = Outputs::new();

        for item in self.outputs.iter() {
            let components: Vec<_> = item.split("=").collect();
            let ch: usize = components[0].parse()?;
            let state: bool = components[1].parse()?;
            outputs.set_ch(ch, state.into());
        }

        if let Some(duty) = self.pwm {
            pdm.set_outputs_pwm(outputs, duty).await?;
        } else {
            pdm.set_outputs(outputs).await?;
        }

        Ok(())
    }
}
