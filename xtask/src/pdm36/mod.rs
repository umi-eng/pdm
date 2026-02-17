mod program_vpd;

#[derive(Debug, clap::Subcommand)]
pub enum Pdm36 {
    /// Write vital product data to OTP memory.
    ProgramVpd(program_vpd::ProgramVpd),
}

impl Pdm36 {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Pdm36::ProgramVpd(cmd) => cmd.run(),
        }
    }
}
