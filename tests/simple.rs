use anyhow::Result;
use cradle::prelude::*;
use tempfile::TempDir;

struct Context {
    temp_dir: TempDir,
}

impl Context {
    fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        Ok(Context { temp_dir })
    }

    fn run(&self) -> String {
        let StdoutTrimmed(output) = (
            executable_path::executable_path("if-newer"),
            self.temp_dir.path().join("input"),
            self.temp_dir.path().join("output"),
            "echo",
            "command ran",
        )
            .run_output();
        output
    }
}

#[test]
fn simple() -> Result<()> {
    let context = Context::new()?;
    assert_eq!(context.run(), "command ran");
    ("touch", context.temp_dir.path().join("output")).run_result()?;
    assert_eq!(context.run(), "");
    Ok(())
}
