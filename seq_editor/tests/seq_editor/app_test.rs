use anyhow::Result;
use gart_seq_editor::app::run;
use std::fs::read_to_string;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn run_success() -> Result<()> {
    let cases = vec![
        (
            // File unchanged
            r"pick 0000 line0
pick 1111 line1",
            r"pick 0000 line0
pick 1111 line1",
        ),
        // Remove comments
        (
            r"pick 0000 line0
pick 1111 line1
# pick 2222 line2
#pick 3333 line3
#",
            r"pick 0000 line0
pick 1111 line1",
        ),
        // Squash commit
        (
            r"pick 0000 line0
pick 1111 \gart sq",
            r"pick 0000 line0
fixup 1111 \gart sq",
        ),
        // Move commit up
        (
            r"pick 0000 line0
pick 1111 \gart mv 1",
            r"pick 1111 \gart mv 1
pick 0000 line0",
        ),
        // Move commit down
        (
            r"pick 0000 \gart mv -1
pick 1111 line1",
            r"pick 1111 line1
pick 0000 \gart mv -1",
        ),
        // Dummy move commit
        (
            r"pick 0000 \gart mv 0
pick 1111 line1",
            r"pick 0000 \gart mv 0
pick 1111 line1",
        ),
        // Move up and squash commit
        (
            r"pick 0000 line0
pick 1111 line1
pick 2222 \gart mv 1 & sq",
            r"pick 0000 line0
fixup 2222 \gart mv 1 & sq
pick 1111 line1",
        ),
        // Move down and squash commit
        (
            r"pick 0000 \gart mv -1 & sq
pick 1111 line1
pick 2222 line2",
            r"pick 1111 line1
fixup 0000 \gart mv -1 & sq
pick 2222 line2",
        ),
    ];

    for (init_content, expected_content) in cases {
        let mut tmp = NamedTempFile::new()?;

        tmp.write_all(init_content.as_bytes())?;
        run(tmp.path().to_path_buf())?;

        let processed_content = read_to_string(tmp.path())?;

        assert_eq!(processed_content, expected_content);
    }

    Ok(())
}

#[test]
fn run_failure() -> Result<()> {
    let cases = vec![
        // Invalid gart command
        r"pick 0000 \gart some",
        // Invalid move command
        r"pick 0000 \gart mv",
        // Invalid command separator
        r"pick 0000 \gart mv 1 | sq",
        // Invalid move up
        r"pick 0000 \gart mv 1
pick 1111 line1",
        // Invalid move down
        r"pick 0000 line0
pick 1111 \gart mv -1",
    ];

    for init_content in cases {
        let mut tmp = NamedTempFile::new()?;

        tmp.write_all(init_content.as_bytes())?;
        assert!(run(tmp.path().to_path_buf()).is_err());
    }

    Ok(())
}

#[test]
fn run_is_not_file() -> Result<()> {
    let tmp_dir = TempDir::new()?;

    assert!(run(tmp_dir.path().to_path_buf()).is_err());

    Ok(())
}

#[test]
fn run_file_does_not_exist() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let non_existing_path = tmp_dir.path().join("tmp");

    assert!(run(non_existing_path.to_path_buf()).is_err());

    Ok(())
}
