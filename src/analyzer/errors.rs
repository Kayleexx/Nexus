

#[derive(Debug)]

pub enum ParseError {
    InvalidSyntax(String),
    IoError(std::io::Error),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err)
    }
}