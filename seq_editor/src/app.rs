use crate::commands::{apply_commands, parse_commands};
use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn run(path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(&path)?;

    let mut lines = content
        .lines()
        .filter(|line| !line.trim().starts_with(r"#") && !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let cmds = parse_commands(&lines)?;
    apply_commands(&mut lines, &cmds)?;

    fs::write(path, lines.join("\n"))?;
    Ok(())
}
