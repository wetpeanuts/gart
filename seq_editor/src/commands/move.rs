use crate::commands::command::{Command, CommandTrait};
use anyhow::{Result, anyhow, bail};
use once_cell::sync::Lazy;
use regex::Regex;

static RE_MV_CMD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*mv\s+").unwrap());

#[derive(Debug, PartialEq)]
pub struct Move {
    line_no: usize,
    pub line_diff: isize,
}

impl Move {
    pub fn new(line_no: usize, line_diff: isize) -> Self {
        Self { line_no, line_diff }
    }
}

impl CommandTrait for Move {
    fn apply(&self, lines: &mut Vec<String>) -> Result<()> {
        if self.line_diff == 0 {
            return Ok(());
        }

        let target_pos = self
            .line_no
            .checked_sub_signed(self.line_diff)
            .ok_or(anyhow!(
                "Failed validate 'mv' command: target line no is out of valid range ({} - {})",
                self.line_no,
                self.line_diff
            ))?;

        if target_pos >= lines.len() {
            bail!(
                "Invalid 'mv' command args: target pos ({}) is higher than total line count ({})",
                target_pos,
                lines.len()
            )
        }

        let line = lines.remove(self.line_no);
        lines.insert(target_pos, line);

        Ok(())
    }

    fn check_type(cmd_str: &str) -> bool {
        return RE_MV_CMD.is_match(cmd_str);
    }

    fn parse(line_no: usize, line: &str) -> Result<Command> {
        let cmd_args: Vec<&str> = line.split_whitespace().collect();

        if cmd_args.len() != 2 {
            bail!("Incorrect number of arguments for 'mv' command. Correct usage: 'mv <int>'")
        }
        if cmd_args[0] != "mv" {
            bail!("Failed to parse 'mv' command")
        }

        let line_diff: isize = cmd_args[1].parse().map_err(|e| {
            anyhow!("Failed to parse argument for 'mv' command: {e}. Correct usage: 'mv <int>'")
        })?;

        if line_no.checked_sub_signed(line_diff).is_none() {
            bail!(
                "Failed validate 'mv' command: target line no is out of valid range ({} - {})",
                line_no,
                line_diff
            )
        }

        Ok(Command::Move(Move { line_no, line_diff }))
    }
}
