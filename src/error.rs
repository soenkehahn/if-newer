use std::io;

#[derive(Debug)]
pub struct AppError(pub String);

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> AppError {
        AppError(format!("{}", error))
    }
}
