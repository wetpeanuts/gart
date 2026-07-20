use crate::commands::{r#move::Move, squash::Squash};
use anyhow::{Result, anyhow, bail};
use once_cell::sync::Lazy;
use regex::Regex;
use std::vec;

static RE_GART_CMD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*\S+\s+\S+\s+\\gart\s+").unwrap());

pub trait CommandTrait {
    fn apply(&self, lines: &mut Vec<String>) -> Result<()>;
    fn check_type(cmd_str: &str) -> bool;
    fn parse(line_no: usize, cmd_str: &str) -> Result<Command>;
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Move(Move),
    Squash(Squash),
}

impl Command {
    fn apply(&self, lines: &mut Vec<String>) -> Result<()> {
        match self {
            Command::Move(cmd) => cmd.apply(lines),
            Command::Squash(cmd) => cmd.apply(lines),
        }
    }

    fn parse(line_no: usize, cmd_str: &str) -> Result<Command> {
        return if Move::check_type(cmd_str) {
            Move::parse(line_no, cmd_str)
        } else if Squash::check_type(cmd_str) {
            Squash::parse(line_no, cmd_str)
        } else {
            bail!("Unknown command")
        };
    }
}

fn parse_commands_from_line(line_no: usize, line: &str) -> Result<Vec<Command>> {
    if let Some(m) = RE_GART_CMD.find(line) {
        let cmds_str: &str = &line[m.end()..];
        let mut curr_line_no = line_no;
        let cmds: Vec<Command> = cmds_str.split('&').try_fold(
            Vec::new(),
            |mut acc, cmd_str| -> Result<Vec<Command>> {
                let cmd = Command::parse(curr_line_no, cmd_str)?;
                if let Command::Move(mv) = &cmd {
                    curr_line_no = curr_line_no
                        .checked_sub_signed(mv.line_diff)
                        .ok_or(anyhow!("Invalid move diff"))?
                }
                acc.push(cmd);
                Ok(acc)
            },
        )?;
        return Ok(cmds);
    }
    Ok(vec![])
}

pub fn parse_commands(lines: &Vec<String>) -> Result<Vec<Command>> {
    let cmds: Vec<Command> = lines.iter().enumerate().try_fold(
        Vec::new(),
        |mut acc, (line_no, line)| -> Result<Vec<Command>> {
            acc.extend(parse_commands_from_line(line_no, line)?);
            Ok(acc)
        },
    )?;
    Ok(cmds)
}

pub fn apply_commands(lines: &mut Vec<String>, commands: &Vec<Command>) -> Result<()> {
    for cmd in commands {
        cmd.apply(lines)?
    }
    Ok(())
}
