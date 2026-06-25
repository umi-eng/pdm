use pdm::pdm36::Pdm36;

#[derive(clap::Parser)]
pub struct Cmd {
    /// Output selection.
    output: usize,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm36) -> anyhow::Result<()> {
        loop {
            let reading = pdm.current_sense(self.output).await?;
            println!("{:.03} A", reading);
        }
    }
}
