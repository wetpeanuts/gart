use anyhow::Result;
use clap::Parser;
use gart_seq_editor::app;
use gart_seq_editor::args::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    app::run(args.file_path)?;

    Ok(())
}
