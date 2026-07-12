use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn run(path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(&path)?;
    let ends_with_newline = content.ends_with('\n');

    let mut filtered_content = content
        .lines()
        .filter(|line| !line.trim().starts_with(r"\gart"))
        .collect::<Vec<_>>()
        .join("\n");

    if ends_with_newline {
        filtered_content.push('\n');
    }

    fs::write(path, filtered_content)?;

    Ok(())
}
