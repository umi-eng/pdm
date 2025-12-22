use pdm::pdm36::Pdm36;
use std::{io::Read, path::PathBuf};

#[derive(clap::Parser)]
pub struct Cmd {
    /// Firmware file.
    #[clap(long)]
    firmware: PathBuf,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm36) -> anyhow::Result<()> {
        let mut file = std::fs::File::open(&self.firmware)?;
        let mut firmware = vec![];
        file.read_to_end(&mut firmware)?;

        println!("Starting firmware update...");
        pdm.update_firmware(&firmware).await?;
        println!("Done.");

        Ok(())
    }
}
