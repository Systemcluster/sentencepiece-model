use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

use prost_build::compile_protos;
use which::which;

fn check_protoc_version(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let output = Command::new(path).arg("--version").output()?;

    let version = String::from_utf8(output.stdout)?;
    let version = version.trim().trim_start_matches("libprotoc").trim();
    let mut version = version.split('.');
    let major = version.next().ok_or("missing major version".to_string())?.parse::<u32>()?;
    let minor = version.next().ok_or("missing minor version".to_string())?.parse::<u32>()?;

    if major < 3 || (major == 3 && minor < 20) {
        return Err("protoc version 3.20 or later is required".into());
    }
    Ok(path.to_owned())
}

pub fn main() {
    let protoc = which("protoc")
        .map_err(|e| e.into())
        .and_then(|path| check_protoc_version(&path));
    let (protoc, includes) = match protoc {
        Err(e) => {
            eprintln!("couldn't find local protoc: {}", e);
            protoc_prebuilt::init("27.1").unwrap()
        }
        Ok(protoc) => (protoc, "".into()),
    };

    eprintln!("using protoc: {:?}", protoc);
    std::env::set_var("PROTOC", protoc);

    let result = if includes.to_str().unwrap().is_empty() {
        compile_protos(&["src/sentencepiece.proto", "src/sentencepiece_model.proto"], &["src/"])
    } else {
        compile_protos(&["src/sentencepiece.proto", "src/sentencepiece_model.proto"], &[
            "src/",
            includes.to_str().unwrap(),
        ])
    };
    result.expect("failed to compile protos");

    println!("cargo:rerun-if-changed=src/sentencepiece.proto");
    println!("cargo:rerun-if-changed=src/sentencepiece_model.proto");
    println!("cargo:rerun-if-changed=build.rs");
}
