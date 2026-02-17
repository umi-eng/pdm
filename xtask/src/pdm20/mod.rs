mod program_vpd;

#[derive(Debug, clap::Subcommand)]
pub enum Pdm20 {
    /// Write vital product data to OTP memory.
    ProgramVpd(program_vpd::ProgramVpd),
}

impl Pdm20 {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Pdm20::ProgramVpd(cmd) => cmd.run(),
        }
    }
}
