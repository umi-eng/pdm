mod program_vpd;

use clap::Parser;

#[derive(clap::Parser)]
enum Cli {
    /// Write vital product data to OTP memory.
    ProgramVpd(program_vpd::Cmd),
}

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::ProgramVpd(cmd) => cmd.run()?,
    }

    Ok(())
}
