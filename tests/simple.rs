use anyhow::Result;
use cradle::prelude::*;
use pretty_assertions::assert_eq;
use std::path::PathBuf;
use tempfile::TempDir;

struct Context {
    temp_dir: TempDir,
}

#[derive(PartialEq, Debug)]
struct Output {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

fn def() -> Output {
    Output {
        stdout: "".to_owned(),
        stderr: "".to_owned(),
        exit_code: 0,
    }
}

impl Context {
    fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        Ok(Context { temp_dir })
    }

    fn input(&self) -> PathBuf {
        self.temp_dir.path().join("input")
    }

    fn output(&self) -> PathBuf {
        self.temp_dir.path().join("output")
    }

    fn run(&self) -> Output {
        let (StdoutTrimmed(stdout), Stderr(stderr), Status(status)) = (
            CurrentDir(self.temp_dir.path()),
            executable_path::executable_path("if-newer"),
            "input",
            "output",
            "echo",
            "command ran",
        )
            .run_output();
        Output {
            stdout,
            stderr,
            exit_code: status.code().unwrap(),
        }
    }
}

fn it(message: &str) {
    eprintln!("it {}", message);
}

#[test]
fn simple() -> Result<()> {
    let context = Context::new()?;
    it("errors when the input file doesn't exist");
    assert_eq!(
        context.run(),
        Output {
            stderr: "ERROR: file not found: input\n".to_owned(),
            exit_code: 1,
            ..def()
        }
    );
    ("touch", context.input()).run_result()?;
    it("runs the command when the output file doesn't exist");
    assert_eq!(
        context.run(),
        Output {
            stdout: "command ran".to_owned(),
            ..def()
        }
    );
    ("touch", context.output()).run_result()?;
    it("doesn't run the command when the output exists");
    assert_eq!(
        context.run(),
        Output {
            stdout: "".to_owned(),
            ..def()
        }
    );
    Ok(())
}
