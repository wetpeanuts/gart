use anyhow::{Result, bail};
use clap::Parser;
use std::path::PathBuf;

pub fn parse_file_path(str_file_path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(str_file_path);

    if !path.exists() {
        bail!("{:?} does not exist", path);
    }

    if !path.is_file() {
        bail!("{:?} is not a file", path);
    }

    Ok(path)
}

#[derive(Parser)]
pub struct Args {
    #[arg(value_parser = parse_file_path)]
    pub file_path: PathBuf,
}
