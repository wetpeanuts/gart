use anyhow::Result;
use gart_common::args::parse_file_path;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn parse_file_path_success() -> Result<()> {
    let tmp = NamedTempFile::new()?;
    let path = parse_file_path(tmp.path().to_str().unwrap())?;

    assert_eq!(path, tmp.path());

    Ok(())
}

#[test]
fn parse_file_path_is_not_file() -> Result<()> {
    let tmp_dir = TempDir::new()?;

    assert!(parse_file_path(tmp_dir.path().to_str().unwrap()).is_err());

    Ok(())
}

#[test]
fn parse_file_path_file_does_not_exist() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let non_existing_path = tmp_dir.path().join("tmp");

    assert!(parse_file_path(non_existing_path.to_str().unwrap()).is_err());

    Ok(())
}
