use url::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum DMError {
    #[error("IO error")]
    IoError(std::io::Error),
    #[error("PostgreSQL error")]
    PostgreSQLError(tokio_postgres::Error),
    #[error("MongoDB error")]
    MongoDbError(mongodb::error::Error),
    #[error("Not supported type")]
    NotSupportedType(),
    #[error("Not supported db")]
    NotSupportedDb(),
    #[error("Invalid in connection string: {0}")]
    InvalidConnectionString(String),
    #[error("url parser error")]
    UrlParseError(ParseError),
    #[error("Directory does not exist: {0}")]
    NotExistsDirectoryError(String),
    #[error("Json error")]
    JsonError(),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Error: {0}")]
    Error(String),
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

impl From<tokio_postgres::Error> for DMError {
    fn from(err: tokio_postgres::Error) -> DMError {
        DMError::PostgreSQLError(err)
    }
}

impl From<mongodb::error::Error> for DMError {
    fn from(err: mongodb::error::Error) -> DMError {
        DMError::MongoDbError(err)
    }
}