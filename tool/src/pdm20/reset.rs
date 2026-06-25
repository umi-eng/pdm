use pdm::pdm20::Pdm20;

#[derive(clap::Parser)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, pdm: Pdm20) -> anyhow::Result<()> {
        pdm.reset().await?;
        pdm.restart().await?;
        Ok(())
    }
}
