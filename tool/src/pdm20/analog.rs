use pdm::pdm20::Pdm20;

#[derive(clap::Parser)]
pub struct Cmd {
    /// Analog input selection.
    input: usize,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm20) -> anyhow::Result<()> {
        loop {
            let reading = pdm.analog_input(self.input).await?;
            println!("{:.03} V", reading);
        }
    }
}
