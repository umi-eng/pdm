use pdm::pdm36::Pdm36;

#[derive(clap::Parser)]
pub struct Cmd {
    /// Analog input selection.
    input: usize,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm36) -> anyhow::Result<()> {
        loop {
            let reading = pdm.analog_input(self.input).await?;
            println!("{:.03} V", reading);
        }
    }
}
