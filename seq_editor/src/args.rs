use clap::Parser;
use gart_common::args::parse_file_path;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(value_parser = parse_file_path)]
    pub file_path: PathBuf,
}
