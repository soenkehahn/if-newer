use std::io;
use std::process::Command;

fn main() {
    if let Err(error) = run() {
        eprintln!("ERROR: {}", error.0)
    }
}

struct AppError(String);

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> AppError {
        AppError(format!("{}", error))
    }
}

fn run() -> Result<(), AppError> {
    let args = parse_args()?;
    let mut command = Command::new(args.command);
    command.args(args.args);
    command.spawn()?.wait()?;
    Ok(())
}

#[derive(Debug)]
struct Args {
    command: String,
    args: Vec<String>,
}

fn parse_args() -> Result<Args, AppError> {
    let args = std::env::args();
    let mut args = args.skip(3);
    Ok(Args {
        command: args
            .next()
            .ok_or(AppError("not enough arguments provided".to_owned()))?,
        args: args.collect(),
    })
}
