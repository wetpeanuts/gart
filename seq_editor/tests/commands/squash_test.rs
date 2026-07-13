use anyhow::Result;
use gart_seq_editor::commands::{command::Command, command::CommandTrait, squash::Squash};

#[test]
fn check_type() {
    assert!(Squash::check_type("sq "));
    assert!(Squash::check_type(" sq "));
    assert!(Squash::check_type("  sq  "));
    assert!(Squash::check_type("\tsq\t"));
    assert!(Squash::check_type("\nsq\n"));
    assert!(Squash::check_type(" \t\nsq\n\t "));
    assert!(Squash::check_type("sq any garbage"));

    assert!(!Squash::check_type(""));
    assert!(!Squash::check_type("other"));
    assert!(!Squash::check_type("sq0"));
}

#[test]
fn parse_success() -> Result<()> {
    let cases = [
        (0, "sq", Squash::new(0)),
        (0, "  sq  ", Squash::new(0)),
        (42, "sq", Squash::new(42)),
    ];

    for (line_no, line, expected) in cases {
        assert_eq!(Squash::parse(line_no, line)?, Command::Squash(expected));
    }

    Ok(())
}

#[test]
fn parse_failure() -> Result<()> {
    let cases = [(0, "sq0"), (0, "other"), (0, "sq 0")];

    for (line_no, line) in cases {
        assert!(Squash::parse(line_no, line).is_err());
    }

    Ok(())
}

#[test]
fn apply_success() -> Result<()> {
    Ok(())
}

#[test]
fn apply_failure() -> Result<()> {
    Ok(())
}
