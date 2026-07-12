use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn run(path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(&path)?;

    let mut filtered_content = content.lines().collect::<Vec<_>>();

    Ok(())
}
