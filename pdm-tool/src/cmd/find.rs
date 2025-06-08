use socketcan::Socket;

#[derive(clap::Parser)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, interface: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
