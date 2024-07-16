use std::error::Error;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use std::{env, fs, io};

use native_tls::TlsConnector;
use prost_build::compile_protos;
use ureq::AgentBuilder;
use which::which;
use zip::read::ZipArchive;

fn check_protoc_version(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let output = Command::new(path).arg("--version").output()?;

    let version = String::from_utf8(output.stdout)?;
    let version = version.trim().trim_start_matches("libprotoc").trim();
    let mut parts = version.split('.');
    let major = parts.next().ok_or("missing major version".to_string())?.parse::<u32>()?;
    let minor = parts.next().ok_or("missing minor version".to_string())?.parse::<u32>()?;

    if major < 3 || (major == 3 && minor < 12) {
        return Err(format!("protoc version 3.12 or later is required (found: {})", version).into());
    }
    Ok(path.to_owned())
}

fn get_protoc_asset(version: &str) -> Result<String, Box<dyn Error>> {
    let os = match env::consts::OS {
        "linux" => "linux",
        "macos" => "osx",
        "windows" => "win",
        _ => return Err("unsupported OS".into()),
    };
    let arch = match env::consts::OS {
        "linux" => match env::consts::ARCH {
            "aarch64" => "aarch_64",
            "powerpc64" => "ppcle_64",
            "s390x" => match version.get(0..4) {
                Some("3.10" | "3.11") => "s390x_64",
                Some("3.12" | "3.13" | "3.14" | "3.15") => "s390x",
                _ => "s390_64",
            },
            "x86" => "x86_32",
            "x86_64" => "x86_64",
            _ => return Err("unsupported architecture".into()),
        },
        "macos" => match env::consts::ARCH {
            "aarch64" => "aarch_64",
            "x86" => "x86_32",
            "x86_64" => "x86_64",
            _ => return Err("unsupported architecture".into()),
        },
        "windows" => match env::consts::ARCH {
            "aarch64" => "-aarch_64",
            "x86" => "32",
            "x86_64" => "64",
            _ => return Err("unsupported architecture".into()),
        },
        _ => unreachable!(),
    };
    let delim = match env::consts::OS {
        "windows" => "",
        _ => "-",
    };
    let asset = format!("protoc-{version}-{os}{delim}{arch}.zip");
    Ok(asset)
}

fn download_protoc(version: &str, force: bool) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
    let asset = get_protoc_asset(version)?;
    let download_path = PathBuf::from(env::var("OUT_DIR")?).join(&asset);
    let extract_path = download_path.with_extension("");

    let protoc = extract_path.join("bin").join("protoc").with_extension(match env::consts::OS {
        "windows" => "exe",
        _ => "",
    });
    let include = extract_path.join("include");

    if protoc.is_file() && include.is_dir() && !force && check_protoc_version(&protoc).is_ok() {
        eprintln!("found existing protoc binary in {:?}", extract_path);
        return Ok((protoc, include));
    }

    let download_file = if !download_path.is_file() || force {
        if download_path.is_file() {
            eprintln!("removing existing protoc download at {:?}", download_path);
            fs::remove_file(&download_path)?;
        }
        let mut download_file = File::options()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&download_path)?;
        let github_token = env::var("GITHUB_TOKEN").ok().map(|t| t.trim().to_string());
        let agent = AgentBuilder::new()
            .https_only(true)
            .timeout(Duration::from_secs(30))
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .try_proxy_from_env(true)
            .tls_connector(Arc::new(TlsConnector::new()?))
            .build();

        let url = format!(
            "https://github.com/protocolbuffers/protobuf/releases/download/v{version}/{asset}",
        );
        eprintln!("downloading protoc from {:?}", url);
        let mut request = agent.get(&url).set("Accept", "application/octet-stream");
        if let Some(token) = &github_token {
            request = request.set("Authorization", &format!("Bearer {}", token));
        }
        let download = request.call().map_err(|e| format!("failed to fetch protobuf: {}", e))?;
        io::copy(&mut download.into_reader(), &mut download_file)?;
        download_file.flush()?;
        download_file.seek(io::SeekFrom::Start(0))?;
        download_file
    } else {
        eprintln!("found existing protoc download at {:?}", download_path);
        File::open(&download_path)?
    };

    let mut archive = ZipArchive::new(download_file)?;
    if extract_path.is_dir() {
        eprintln!("removing existing extract directory {:?}", extract_path);
        fs::remove_dir_all(&extract_path)?;
    }
    fs::create_dir_all(&extract_path)?;
    eprintln!("extracting protoc to {:?}", extract_path);
    archive.extract(&extract_path)?;

    if !protoc.is_file() {
        return Err(format!("failed to find extracted protoc binary in {:?}", extract_path).into());
    }
    check_protoc_version(&protoc)?;

    Ok((protoc, include))
}

pub fn main() {
    let protoc = which("protoc")
        .map_err(|e| e.into())
        .and_then(|path| check_protoc_version(&path));
    let (protoc, include) = match protoc {
        Err(e) => {
            eprintln!("couldn't find local protoc: {} (downloading)", e);
            download_protoc("27.2", false)
                .or_else(|e| {
                    eprintln!("couldn't download protoc: {} (retrying)", e);
                    download_protoc("27.2", true)
                })
                .unwrap()
        }
        Ok(protoc) => (protoc, "".into()),
    };

    eprintln!("using protoc: {:?}", protoc);
    env::set_var("PROTOC", protoc);

    let result = if include.to_str().unwrap().is_empty() {
        compile_protos(&["src/sentencepiece.proto", "src/sentencepiece_model.proto"], &["src/"])
    } else {
        compile_protos(&["src/sentencepiece.proto", "src/sentencepiece_model.proto"], &[
            "src/",
            include.to_str().unwrap(),
        ])
    };
    result.expect("failed to compile protos");

    println!("cargo:rerun-if-changed=src/sentencepiece.proto");
    println!("cargo:rerun-if-changed=src/sentencepiece_model.proto");
    println!("cargo:rerun-if-changed=build.rs");
}
