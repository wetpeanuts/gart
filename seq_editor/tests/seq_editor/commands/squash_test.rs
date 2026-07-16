use anyhow::Result;
use gart_seq_editor::commands::{command::Command, command::CommandTrait, squash::Squash};

#[test]
fn check_type() {
    assert!(Squash::check_type("sq"));
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
    let lines = vec![
        "pick 0000 line1".to_string(),
        "any   1111 line2".to_string(),
    ];

    let cases = [
        (Squash::new(0), 0, "fixup 0000 line1".to_string()),
        (Squash::new(1), 1, "fixup 1111 line2".to_string()),
    ];

    for (sq_cmd, line_no, expeceted_line) in cases {
        let mut lines_copy = lines.clone();
        let mut lines_expected = lines.clone();
        lines_expected[line_no] = expeceted_line;

        sq_cmd.apply(&mut lines_copy)?;
        assert_eq!(lines_copy, lines_expected);
    }

    Ok(())
}

#[test]
fn apply_failure() -> Result<()> {
    let lines = vec!["pick 0000 line1".to_string(), "any".to_string()];

    let cases = [Squash::new(2)];

    for sq_cmd in cases {
        let mut lines_copy = lines.clone();
        assert!(sq_cmd.apply(&mut lines_copy).is_err());
    }

    Ok(())
}
