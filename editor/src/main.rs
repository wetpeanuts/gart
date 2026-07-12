use anyhow::Result;
use clap::Parser;
use gart_editor::app::run;
use gart_editor::args::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    run(args.file_path)?;

    Ok(())
}
