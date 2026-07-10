use anyhow::Result;
use clap::Parser;
use gart_editor::args::Args;
use gart_editor::formatting::remove_gart_commands;

fn main() -> Result<()> {
    let args = Args::parse();
    remove_gart_commands(args.file_path)?;

    Ok(())
}
