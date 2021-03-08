use std::fmt;

#[derive(Debug)]
pub enum MyCustomError {
    NoFilesError,
    IoError(std::io::Error),
    PathError,
    Default(String),
    ParseError(String),
}

impl std::error::Error for MyCustomError {}

impl From<std::io::Error> for MyCustomError {
    fn from(e: std::io::Error) -> Self {
        MyCustomError::IoError(e)
    }
}

impl From<&str> for MyCustomError {
    fn from(e: &str) -> Self {
        MyCustomError::Default(e.to_string())
    }
}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::NoFilesError => write!(f, "No files in dir"),
            MyCustomError::IoError(x) => write!(f, "IO error :{:?}", x),
            MyCustomError::PathError => write!(f, "path error"),
            MyCustomError::Default(x) => write!(f, "{}", x),
            MyCustomError::ParseError(x) => write!(f, "{}",x),
        }
    }
}
