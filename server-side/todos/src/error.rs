use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Internal,
    AlreadyExists,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal => {
                write!(f, "Internal error.")
            }
            Self::AlreadyExists => {
                write!(f, "Already exists.")
            }
        }
    }
}

impl std::error::Error for Error {}
