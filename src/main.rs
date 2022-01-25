use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

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
    let input_modified = get_modified(&args.input)?.ok_or(AppError(format!(
        "file not found: {}",
        args.input.to_string_lossy()
    )))?;
    let should_run = match get_modified(&args.output)? {
        None => true,
        Some(output_modified) => output_modified < input_modified,
    };
    if should_run {
        let mut command = Command::new(args.command);
        command.args(args.args);
        command.spawn()?.wait()?;
    }
    Ok(())
}

fn get_modified(path: &Path) -> Result<Option<SystemTime>, AppError> {
    Ok(match fs::metadata(path) {
        Ok(metadata) => Some(metadata.modified()?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => Err(error)?,
    })
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
