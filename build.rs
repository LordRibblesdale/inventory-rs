fn main() -> anyhow::Result<()> {
    // println!("cargo:rerun-if-changed=protobuf/database.proto");

    protobuf_codegen::Codegen::new()
        .pure()
        .include("protobuf")
        .input("protobuf/database.proto")
        .out_dir("src/storage")
        .run()
}