use anyhow::Result;
use cradle::prelude::*;
use tempfile::TempDir;

#[test]
fn runs_given_commands() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let StdoutTrimmed(output) = (
        executable_path::executable_path("if-newer"),
        temp_dir.path().join("file"),
        temp_dir.path().join("output"),
        "echo",
        "foo",
    )
        .run_output();
    assert_eq!(output, "foo");
    Ok(())
}
