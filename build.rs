use miette::{IntoDiagnostic, Result};

pub fn main() -> Result<()> {
    let descriptors =
        protox::compile(["src/sentencepiece.proto", "src/sentencepiece_model.proto"], ["src/"])?;
    prost_build::compile_fds(descriptors).into_diagnostic()?;

    println!("cargo:rerun-if-changed=src/sentencepiece.proto");
    println!("cargo:rerun-if-changed=src/sentencepiece_model.proto");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
