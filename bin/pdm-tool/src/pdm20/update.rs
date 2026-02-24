use pdm::pdm20::Pdm20;
use std::{io::Read, path::PathBuf};

#[derive(clap::Parser)]
pub struct Cmd {
    /// Firmware file.
    #[clap(long)]
    firmware: PathBuf,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm20) -> anyhow::Result<()> {
        let mut file = std::fs::File::open(&self.firmware)?;
        let mut firmware = vec![];
        file.read_to_end(&mut firmware)?;

        println!("Starting firmware update...");
        pdm.update_firmware(&firmware).await?;
        println!("Done.");

        Ok(())
    }
}
