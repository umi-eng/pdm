mod cmd;

use anyhow::Result;
use clap::Parser;

#[derive(clap::Parser)]
struct Cli {
    /// CAN bus interface
    #[clap(long, short)]
    interface: String,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Find(cmd) => cmd.run(&self.interface).await,
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Find PDMs
    Find(cmd::find::Cmd),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.run().await
}
