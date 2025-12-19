mod pdm20;
mod pdm36;

use anyhow::Result;
use clap::Parser;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Pdm20(cmd) => cmd.run().await,
            Subcommand::Pdm36(cmd) => cmd.run().await,
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// PDM20 commands.
    Pdm20(pdm20::Cmd),
    /// PDM36 commands.
    Pdm36(pdm36::Cmd),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await
}

pub(crate) fn maybe_hex(input: &str) -> Result<u8, String> {
    let result = if input.starts_with("0x") {
        u8::from_str_radix(&input[2..], 16)
    } else {
        u8::from_str_radix(input, 10)
    };

    result.map_err(|e| format!("{e}"))
}
