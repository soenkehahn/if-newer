use std::fs;
use std::io;
use std::io::Read;
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
    let input_modified = get_input_modified(&args)?;
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

fn get_input_modified(args: &Args) -> Result<SystemTime, AppError> {
    let mut stdin = String::new();
    let input_files: Vec<&Path> = match args.input.as_str() {
        "-" => {
            std::io::stdin().read_to_string(&mut stdin)?;
            stdin.split_whitespace().map(Path::new).collect()
        }
        _ => vec![Path::new(&args.input)],
    };
    let mut max_modified = None;
    for input_file in input_files {
        let modified = get_modified(input_file)?
            .ok_or_else(|| AppError(format!("file not found: {}", input_file.to_string_lossy())))?;
        match max_modified {
            None => {
                max_modified = Some(modified);
            }
            Some(max) => {
                if modified > max {
                    max_modified = Some(modified);
                }
            }
        }
    }
    max_modified.ok_or(AppError("no input files given on stdin".to_owned()))
}

fn get_modified(path: &Path) -> Result<Option<SystemTime>, AppError> {
    Ok(match fs::metadata(path) {
        Ok(metadata) => Some(metadata.modified()?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    })
}

#[derive(Debug)]
struct Args {
    input: String,
    output: PathBuf,
    command: String,
    args: Vec<String>,
}

fn parse_args() -> Result<Args, AppError> {
    let args = std::env::args();
    let mut args = args.skip(1);
    Ok(Args {
        input: args.next().unwrap(),
        output: PathBuf::from(args.next().unwrap()),
        command: args
            .next()
            .ok_or_else(|| AppError("not enough arguments provided".to_owned()))?,
        args: args.collect(),
    })
}
