#[derive(thiserror::Error, Debug)]
pub enum DMError {
    #[error("IO error")]
    IoError(std::io::Error),
    #[error("Not supported db")]
    NotSupportedDb(),
    #[error("Invalid in connection string {0}")]
    InvalidConnectionString(String),
}

impl From<std::io::Error> for DMError {
    fn from(err: std::io::Error) -> DMError {
        DMError::IoError(err)
    }
}