use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct ProgramVpd {
    /// VPD file in RON format.
    #[clap(long)]
    vpd: PathBuf,
    /// Make probe-rs do a dry run.
    #[clap(long)]
    dry_run: bool,
}

impl ProgramVpd {
    pub fn run(self) -> anyhow::Result<()> {
        todo!("PDM20 VPD loading")
    }
}
