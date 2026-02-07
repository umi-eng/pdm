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
        // do this dance to avoid cargo_toml trying to resolve the whole workspace
        let toml_content =
            std::fs::read_to_string("./Cargo.toml").context("Could not read Cargo.toml")?;
        let manifest = cargo_toml::Manifest::from_slice(toml_content.as_bytes())
            .context("Could not read cargo manifest")?;

        let config = std::fs::read_to_string(&self.board).context(format!(
            "Can't access board config at: {}",
            self.board.display()
        ))?;
        let config: Board = toml::from_str(&config).context(format!(
            "Can't parse board config at: {}",
            self.board.display()
        ))?;

        println!("Patching .header section");
        PatchHeader {
            firmware: self.path.clone(),
            version: manifest.package().version().to_owned(),
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
