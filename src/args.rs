use crate::error::AppError;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub output: PathBuf,
    pub command: String,
    pub args: Vec<String>,
}

pub fn parse_args() -> Result<Args, AppError> {
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
