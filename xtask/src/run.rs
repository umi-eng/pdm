use crate::config::Board;
use crate::patch_header::PatchHeader;
use anyhow::Context as _;
use std::path::PathBuf;
use xshell::Shell;

#[derive(Debug, clap::Parser)]
pub struct Run {
    /// The path to the ELF file to flash and run.
    path: PathBuf,
    #[clap(long, default_value = "board.toml")]
    board: PathBuf,
}

impl Run {
    pub fn run(self) -> anyhow::Result<()> {
        let config = std::fs::read_to_string(&self.board).context(format!(
            "Can't access board config at: {}",
            self.board.display()
        ))?;
        let config: Board = toml::from_str(&config).context(format!(
            "Can't parse board config at: {}",
            self.board.display()
        ))?;

        PatchHeader {
            firmware: self.path.clone(),
            version: "0.1.0".to_owned(),
            target: config.target,
        }
        .run()?;

        let sh = Shell::new()?;
        let chip = config.chip;
        let path = self.path;
        xshell::cmd!(sh, "probe-rs run --chip {chip} {path}").run()?;

        Ok(())
    }
}
