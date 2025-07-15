mod cmd;

use anyhow::Result;
use clap::Parser;
use pdm::pdm36;
use socketcan::tokio::CanSocket;

#[derive(clap::Parser)]
struct Cli {
    /// CAN bus interface
    #[clap(long)]
    interface: String,
    /// Device address.
    #[clap(long, default_value_t = 0x55)]
    address: u8,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Update(cmd) => cmd.run(open_pdm(&self.interface, self.address)?).await,
            Subcommand::Output(cmd) => cmd.run(open_pdm(&self.interface, self.address)?).await,
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Update firmware.
    Update(cmd::update::Cmd),
    /// Control outputs.
    Output(cmd::output::Cmd),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.run().await
}

fn open_pdm(interface: &str, address: u8) -> Result<pdm36::Pdm36> {
    let socket = CanSocket::open(interface)?;
    Ok(pdm36::Pdm36::new(socket, address))
}
