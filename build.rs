use std::io::Result;

fn main() -> Result<()> {
    // println!("cargo:rerun-if-changed=protobuf/database.proto");

    prost_build::Config::new()
        .out_dir("src/database")
        .default_package_filename("database")
        .compile_protos(&["database.proto"], &["protobuf"])?;
    Ok(())
}