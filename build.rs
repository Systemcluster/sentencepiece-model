use prost_build::compile_protos;
use protoc_bin_vendored::{include_path, protoc_bin_path};
use which::which;

pub fn main() {
    let protoc = which("protoc").unwrap_or_else(|_| protoc_bin_path().unwrap());
    eprintln!("Using protoc: {:?}", protoc);
    std::env::set_var("PROTOC", protoc);

    compile_protos(&["src/sentencepiece.proto", "src/sentencepiece_model.proto"], &[
        "src/",
        include_path().unwrap().to_str().unwrap(),
    ])
    .unwrap();

    println!("cargo:rerun-if-changed=src/sentencepiece.proto");
    println!("cargo:rerun-if-changed=src/sentencepiece_model.proto");
    println!("cargo:rerun-if-changed=build.rs");
}
