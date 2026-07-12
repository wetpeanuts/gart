use anyhow::Result;
use gart_seq_editor::commands::{command::Command, command::CommandTrait, r#move::Move};

fn permute<T: Clone>(v: &mut Vec<T>, perm: &[usize]) {
    assert_eq!(v.len(), perm.len());

    let old = v.clone();

    for (i, &j) in perm.iter().enumerate() {
        v[i] = old[j].clone();
    }
}

#[test]
fn check_type() {
    assert!(Move::check_type("mv "));
    assert!(Move::check_type(" mv "));
    assert!(Move::check_type("  mv  "));
    assert!(Move::check_type("\tmv\t"));
    assert!(Move::check_type("\nmv\n"));
    assert!(Move::check_type(" \t\nmv\n\t "));
    assert!(Move::check_type("mv 0"));
    assert!(Move::check_type("mv any garbage"));

    assert!(!Move::check_type(""));
    assert!(!Move::check_type("other"));
    assert!(!Move::check_type("mv0"));
}

#[test]
fn parse_success() -> Result<()> {
    let cases = [
        (0, "mv 0", Move::new(0, 0)),
        (0, "  mv  0  ", Move::new(0, 0)),
        (42, "mv 0", Move::new(42, 0)),
        (42, "mv 42", Move::new(42, 42)),
        (42, "mv 1", Move::new(42, 1)),
        (1, "mv 1", Move::new(1, 1)),
        (1, "mv 0", Move::new(1, 0)),
        (0, "mv -1", Move::new(0, -1)),
        (1, "mv -1", Move::new(1, -1)),
    ];

    for (line_no, line, expected) in cases {
        assert_eq!(Move::parse(line_no, line)?, Command::Move(expected));
    }

    Ok(())
}

#[test]
fn parse_failure() -> Result<()> {
    let cases = [
        (0, "mv"),
        (0, "  mv  "),
        (0, "mv0"),
        (0, "other"),
        (0, "mv 0 0"),
        (0, "mv 1"),
        (42, "mv 43"),
        (usize::MAX, "mv -1"),
    ];

    for (line_no, line) in cases {
        assert!(Move::parse(line_no, line).is_err());
    }

    Ok(())
}

#[test]
fn apply_success() -> Result<()> {
    let lines = vec![
        "line1".to_string(),
        "line2".to_string(),
        "line3".to_string(),
    ];

    let cases = [
        (Move::new(0, 0), [0, 1, 2]),
        (Move::new(1, 0), [0, 1, 2]),
        (Move::new(1, 1), [1, 0, 2]),
        (Move::new(2, 1), [0, 2, 1]),
        (Move::new(2, 2), [2, 0, 1]),
        (Move::new(0, -1), [1, 0, 2]),
        (Move::new(0, -2), [1, 2, 0]),
    ];

    for (mv_cmd, permutation) in cases {
        let mut lines_copy = lines.clone();
        let mut lines_expected = lines.clone();
        permute(&mut lines_expected, &permutation);

        mv_cmd.apply(&mut lines_copy)?;
        assert_eq!(lines_copy, lines_expected);
    }

    Ok(())
}

#[test]
fn apply_failure() -> Result<()> {
    let lines = vec![
        "line1".to_string(),
        "line2".to_string(),
        "line3".to_string(),
    ];

    let cases = [
        Move::new(0, 1),
        Move::new(1, 2),
        Move::new(1, -2),
        Move::new(2, -1),
    ];

    for mv_cmd in cases {
        let mut lines_copy = lines.clone();
        assert!(mv_cmd.apply(&mut lines_copy).is_err());
    }

    Ok(())
}
