use url::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum DMError {
    #[error("IO error")]
    IoError(std::io::Error),
    #[error("Not supported type")]
    NotSupportedType(),
    #[error("Not supported db")]
    NotSupportedDb(),
    #[error("Invalid in connection string: {0}")]
    InvalidConnectionString(String),
    #[error("url parser error")]
    UrlParseError(ParseError),
}

impl From<ParseError> for DMError {
    fn from(err: ParseError) -> DMError {
        DMError::UrlParseError(err)
    }
}

impl From<std::io::Error> for DMError {
    fn from(err: std::io::Error) -> DMError {
        DMError::IoError(err)
    }
}