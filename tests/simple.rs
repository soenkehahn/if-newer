use anyhow::Result;
use colored::Colorize;
use cradle::prelude::*;
use pretty_assertions::assert_eq;
use std::thread;
use std::time::Duration;
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

    fn touch(&self, path: &str) -> Result<()> {
        ("touch", self.temp_dir.path().join(path)).run_result()?;
        Ok(())
    }

    fn run(&self, input: &str, stdin: Option<&str>) -> Output {
        let stdin = match stdin {
            None => vec![],
            Some(stdin) => vec![Stdin(stdin)],
        };
        let (StdoutTrimmed(stdout), Stderr(stderr), Status(status)) = (
            CurrentDir(self.temp_dir.path()),
            stdin,
            executable_path::executable_path("if-newer"),
            input,
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
    colored::control::set_override(true);
    eprintln!("{}", format!("it {}", message).yellow());
}

#[test]
fn simple() -> Result<()> {
    let context = Context::new()?;

    it("errors when the input file doesn't exist");
    assert_eq!(
        context.run("input", None),
        Output {
            stderr: "ERROR: file not found: input\n".to_owned(),
            exit_code: 1,
            ..def()
        }
    );

    it("runs the command when the output file doesn't exist");
    context.touch("input")?;
    assert_eq!(
        context.run("input", None),
        Output {
            stdout: "command ran".to_owned(),
            ..def()
        }
    );

    it("doesn't run the command when the output exists");
    context.touch("output")?;
    assert_eq!(
        context.run("input", None),
        Output {
            stdout: "".to_owned(),
            ..def()
        }
    );

    it("runs the command when the input is newer than the output");
    thread::sleep(Duration::from_millis(30));
    context.touch("input")?;
    assert_eq!(
        context.run("input", None),
        Output {
            stdout: "command ran".to_owned(),
            ..def()
        }
    );

    Ok(())
}

#[test]
fn multiple_input_files() -> Result<()> {
    let context = Context::new()?;

    it("reads input files from stdin when '-' is given");
    assert_eq!(
        context.run("-", Some("input1 input2")),
        Output {
            stderr: "ERROR: file not found: input1\n".to_owned(),
            exit_code: 1,
            ..def()
        }
    );
    context.touch("input1")?;
    assert_eq!(
        context.run("-", Some("input1 input2")),
        Output {
            stderr: "ERROR: file not found: input2\n".to_owned(),
            exit_code: 1,
            ..def()
        }
    );

    it("runs the command when output doesn't exist");
    context.touch("input2")?;
    assert_eq!(
        context.run("-", Some("input1 input2")),
        Output {
            stdout: "command ran".to_owned(),
            exit_code: 0,
            ..def()
        }
    );

    it("doesn't run the command when output exists");
    context.touch("output")?;
    assert_eq!(
        context.run("-", Some("input1 input2")),
        Output {
            stdout: "".to_owned(),
            exit_code: 0,
            ..def()
        }
    );

    it("runs the command when the first output is newer");
    context.touch("input1")?;
    assert_eq!(
        context.run("-", Some("input1 input2")),
        Output {
            stdout: "command ran".to_owned(),
            exit_code: 0,
            ..def()
        }
    );

    it("runs the command when the second output is newer");
    context.touch("output")?;
    thread::sleep(Duration::from_millis(30));
    context.touch("input2")?;
    assert_eq!(
        context.run("-", Some("input1 input2")),
        Output {
            stdout: "command ran".to_owned(),
            exit_code: 0,
            ..def()
        }
    );

    Ok(())
}
