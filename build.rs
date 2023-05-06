use std::io::Result;

fn main() -> Result<()> {
    // println!("cargo:rerun-if-changed=protobuf/database.proto");

    prost_build::Config::new()
        .out_dir("src/storage")
        .default_package_filename("mod")
        .compile_protos(&["database.proto"], &["protobuf"])?;
    Ok(())
}