mod program_vpd;
mod sign;

use clap::Parser;

#[derive(clap::Parser)]
enum Cli {
    /// Write vital product data to OTP memory.
    ProgramVpd(program_vpd::Cmd),
    /// Sign a firmware binary.
    Sign(sign::Cmd),
}

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::ProgramVpd(cmd) => cmd.run()?,
        Cli::Sign(cmd) => cmd.run()?,
    }

    Ok(())
}
