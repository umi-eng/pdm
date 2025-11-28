use dbc_codegen::FeatureConfig;

fn main() {
    let dbc_path = "pdm-36.dbc";
    let dbc_file = std::fs::read_to_string(dbc_path).unwrap();
    println!("cargo:rerun-if-changed={dbc_path}");

    let config = dbc_codegen::Config::builder()
        .dbc_name("pdm-36.dbc")
        .dbc_content(&dbc_file)
        .impl_defmt(FeatureConfig::Gated("defmt-1"))
        .build();

    let mut out = std::io::BufWriter::new(std::fs::File::create("src/messages.rs").unwrap());
    dbc_codegen::codegen(config, &mut out).unwrap();
}
