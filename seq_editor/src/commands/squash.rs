use crate::commands::command::{Command, CommandTrait};
use anyhow::{Result, bail};
use once_cell::sync::Lazy;
use regex::Regex;

static RE_SQ_CMD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*sq(?:\s+|$)").unwrap());
static RE_FIRST_WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*\S+\s+").unwrap());

#[derive(Debug, PartialEq)]
pub struct Squash {
    line_no: usize,
}

impl Squash {
    pub fn new(line_no: usize) -> Self {
        Self { line_no }
    }
}

impl CommandTrait for Squash {
    fn apply(&self, lines: &mut Vec<String>) -> Result<()> {
        if self.line_no >= lines.len() {
            bail!(
                "Invalid 'sq' command: line {} does not exist (lines count: {})",
                self.line_no,
                lines.len()
            )
        }

        // Squash and clean second commit message
        let new_line = RE_FIRST_WORD
            .replacen(&lines[self.line_no], 1, "fixup ")
            .into_owned();
        lines[self.line_no] = new_line;

        Ok(())
    }

    fn check_type(cmd_str: &str) -> bool {
        return RE_SQ_CMD.is_match(cmd_str);
    }

    fn parse(line_no: usize, line: &str) -> Result<Command> {
        let cmd_args: Vec<&str> = line.split_whitespace().collect();

        if cmd_args.len() != 1 {
            bail!("Incorrect number of arguments for 'sq' command. Correct usage: 'sq'")
        }
        if cmd_args[0] != "sq" {
            bail!("Failed to parse 'sq' command")
        }

        Ok(Command::Squash(Squash { line_no }))
    }
}
