use std::io;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if let Err(error) = run() {
        eprintln!("ERROR: {}", error.0);
        std::process::exit(1);
    }
}

#[derive(Debug)]
struct AppError(String);

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> AppError {
        AppError(format!("{}", error))
    }
}

fn run() -> Result<(), AppError> {
    let args = parse_args()?;
    if !args.input.exists() {
        return Err(AppError(format!(
            "file not found: {}",
            args.input.to_string_lossy()
        )));
    }
    if !args.output.exists() {
        let mut command = Command::new(args.command);
        command.args(args.args);
        command.spawn()?.wait()?;
    }
    Ok(())
}

#[derive(Debug)]
struct Args {
    input: PathBuf,
    output: PathBuf,
    command: String,
    args: Vec<String>,
}

fn parse_args() -> Result<Args, AppError> {
    let args = std::env::args();
    let mut args = args.skip(1);
    Ok(Args {
        input: PathBuf::from(args.next().unwrap()),
        output: PathBuf::from(args.next().unwrap()),
        command: args
            .next()
            .ok_or(AppError("not enough arguments provided".to_owned()))?,
        args: args.collect(),
    })
}
