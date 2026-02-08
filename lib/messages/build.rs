use dbc_codegen::FeatureConfig;

fn main() {
    generate("pdm36");
    generate("pdm20");
}

fn generate(name: &str) {
    let dbc_file = format!("{name}.dbc");
    let file = std::fs::read_to_string(&dbc_file).unwrap();
    println!("cargo:rerun-if-changed={dbc_file}");

    let out_file = format!("src/{name}/messages.rs");
    let out_file = std::path::Path::new(&out_file);
    std::fs::create_dir_all(out_file.parent().unwrap()).unwrap();
    let mut out = std::io::BufWriter::new(std::fs::File::create(out_file).unwrap());

    let config = dbc_codegen::Config::builder()
        .dbc_name(name)
        .dbc_content(&file)
        .impl_defmt(FeatureConfig::Gated("defmt-1"))
        .build();
    dbc_codegen::codegen(config, &mut out).unwrap();
}
