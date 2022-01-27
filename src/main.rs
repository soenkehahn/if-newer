mod args;
mod error;

use crate::args::{parse_args, Args};
use crate::error::AppError;
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

fn run() -> Result<(), AppError> {
    let args = parse_args()?;
    if should_run(&args)? {
        let mut command = Command::new(args.command);
        command.args(args.args);
        command.spawn()?.wait()?;
    } else {
        log(
            &format!(
                "{} is newer than all input files",
                args.output.to_string_lossy(),
            ),
            false,
            &args,
        );
    }
    Ok(())
}

fn should_run(args: &Args) -> Result<bool, AppError> {
    let input_times = get_input_times(args)?;
    match get_time(&args.output)? {
        None => {
            log(
                &format!("{} not found", args.output.to_string_lossy(),),
                true,
                args,
            );
            Ok(true)
        }
        Some(output_time) => {
            for (input, input_time) in input_times {
                if output_time < input_time {
                    log(
                        &format!(
                            "{} is newer than {}",
                            input.to_string_lossy(),
                            args.output.to_string_lossy(),
                        ),
                        true,
                        args,
                    );
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}

fn get_input_times(args: &Args) -> Result<Vec<(PathBuf, SystemTime)>, AppError> {
    let mut stdin = String::new();
    let input_files: Vec<&Path> = match args.input.as_str() {
        "-" => {
            std::io::stdin().read_to_string(&mut stdin)?;
            stdin.split_whitespace().map(Path::new).collect()
        }
        _ => vec![Path::new(&args.input)],
    };
    let mut input_times: Vec<(PathBuf, SystemTime)> = Vec::new();
    for input_file in input_files {
        let modified = get_time(input_file)?
            .ok_or_else(|| AppError(format!("file not found: {}", input_file.to_string_lossy())))?;
        input_times.push((input_file.to_owned(), modified));
    }
    Ok(input_times)
}

fn get_time(path: &Path) -> Result<Option<SystemTime>, AppError> {
    Ok(match fs::metadata(path) {
        Ok(metadata) => Some(metadata.modified()?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    })
}

fn log(message: &str, should_run: bool, args: &Args) {
    eprintln!(
        "{},{}running: {} {}",
        message,
        if should_run { " " } else { " not " },
        args.command,
        args.args.join(" ")
    );
}
