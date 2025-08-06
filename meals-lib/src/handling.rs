use std::fmt;

#[derive(Debug, Clone)]
pub struct HandlingError;

impl fmt::Display for HandlingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not handle command")
    }
}

pub trait CommandHandler<T> {
    fn handle(&self) -> Result<T>;
}

pub type Result<T> = std::result::Result<T, HandlingError>;