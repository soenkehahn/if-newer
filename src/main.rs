use std::io;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if let Err(error) = run() {
        eprintln!("ERROR: {}", error.0)
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
    if !args.output.exists() {
        let mut command = Command::new(args.command);
        command.args(args.args);
        command.spawn()?.wait()?;
    }
    Ok(())
}

#[derive(Debug)]
struct Args {
    output: PathBuf,
    command: String,
    args: Vec<String>,
}

fn parse_args() -> Result<Args, AppError> {
    let args = std::env::args();
    let mut args = args.skip(2);
    Ok(Args {
        output: PathBuf::from(args.next().unwrap()),
        command: args
            .next()
            .ok_or(AppError("not enough arguments provided".to_owned()))?,
        args: args.collect(),
    })
}
