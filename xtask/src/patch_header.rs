use anyhow::{Context, ensure};
use header::Flags;
use object::{Object, ObjectSection, ReadCacheOps, SectionKind, elf::SHF_ALLOC};
use std::{fs::File, io::Write, path::PathBuf};
use zerocopy::IntoBytes;

#[derive(Debug, clap::Parser)]
pub struct PatchHeader {
    /// Firmware binary.
    pub firmware: PathBuf,
    /// Firmware version.
    #[clap(long)]
    pub version: String,
    /// Target board.
    #[clap(long)]
    pub target: String,
}

impl PatchHeader {
    pub fn run(self) -> anyhow::Result<()> {
        let version: Vec<u8> = self
            .version
            .split(".")
            .map(|s| s.parse().expect("Convert to u8"))
            .collect();
        ensure!(version.len() == 3, "Version number malformed");
        let version = header::Version::new(version[0], version[1], version[2]);
        println!(
            "Firmware version: v{}.{}.{}",
            version.major, version.minor, version.patch
        );

        let target = self.target.into_bytes();
        ensure!(target.len() == 4, "Target length incorrect");
        let target = [target[0], target[1], target[2], target[3]];
        println!("Target: {:?}", String::from_utf8_lossy(&target));

        println!("Patching firmware: {:?}", self.firmware);
        let binary_data = std::fs::read(&self.firmware)?;
        let obj_file = object::File::parse(&*binary_data)?;

        let header_section = obj_file
            .section_by_name(".header")
            .expect("No .header section found in binary");
        println!("Found .header section:");

        let header_offset = header_section
            .file_range()
            .expect("Header section has no file range")
            .0;

        let header_size = header_section.size();
        println!("  Header section size: {} bytes", header_size);

        let image_size = calculate_flash_image_size(&obj_file)?;
        println!("Image size: {} bytes", image_size);

        let header = header::ImageHeader::new(image_size, version, Flags::empty(), target);

        let mut file = File::options().write(true).open(self.firmware)?;

        file.seek(header_offset)
            .expect("Could not seek to position");
        file.write_all(header.as_bytes())?;
        println!("✓ ELF binary patched successfully");

        Ok(())
    }
}

/// Calculate what arm-none-eabi-objcopy -O binary would produce
fn calculate_flash_image_size(obj_file: &object::File) -> anyhow::Result<u32> {
    println!("Calculating binary size");

    let mut size = 0;
    let mut ignored_sections = 0;

    for section in obj_file.sections() {
        let section_name = section
            .name()
            .context(format!("Failed to read section name"))?;

        if section.kind() == SectionKind::UninitializedData {
            ignored_sections += 1;
            continue;
        }

        let flags = match section.flags() {
            object::SectionFlags::Elf { sh_flags } => sh_flags as u32,
            _ => unreachable!("section flag type invalid"),
        };
        if flags & SHF_ALLOC != 0 {
            let section_size = section.size();
            println!(
                "  Including section: {} @ 0x{:08X} ({} bytes)",
                section_name,
                section.address(),
                section_size
            );

            size += section_size;
        } else {
            ignored_sections += 1;
            continue;
        }
    }

    println!("  Ignored {ignored_sections} sections");

    Ok(size as u32)
}
