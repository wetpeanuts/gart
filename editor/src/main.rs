use anyhow::Result;
use clap::Parser;
use gart_editor::args::Args;
use gart_editor::app::run;

fn main() -> Result<()> {
    let args = Args::parse();
    run(args.file_path)?;

    Ok(())
}
