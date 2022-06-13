use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    SerializationError(serde_yaml::Error),
    IoError(std::io::Error),
    WordnikError(wordnik::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerializationError(err) => write!(f, "{}", err),
            Self::IoError(err) => write!(f, "{}", err),
            Self::WordnikError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerializationError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<wordnik::error::Error> for Error {
    fn from(err: wordnik::error::Error) -> Self {
        Self::WordnikError(err)
    }
}
