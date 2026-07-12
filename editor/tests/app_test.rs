use anyhow::Result;
use gart_editor::app::run;
use std::fs::read_to_string;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn run_success() -> Result<()> {
    let mut tmp = NamedTempFile::new()?;
    let init_content = r"dummy line 1
\gart
dummy line 2
\gart command
dummy line 3
   \gart command
dummy line 4
\gartcommand
\command

dummy line 5

";

    let expected_filtered_content = r"dummy line 1
dummy line 2
dummy line 3
dummy line 4
\command

dummy line 5

";

    tmp.write_all(init_content.as_bytes())?;
    run(tmp.path().to_path_buf())?;

    let filtered_content = read_to_string(tmp.path())?;

    assert_eq!(filtered_content, expected_filtered_content);

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
