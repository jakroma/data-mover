use url::Url;

use crate::{DMError, DMResult, db::db_type::Database};

#[derive(Clone)]
pub struct MigrationConnection {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub db_type: Database,
}

pub struct MigrationConnections {
    pub from: MigrationConnection,
    pub to: MigrationConnection
}

impl MigrationConnections {
    pub fn new(from: MigrationConnection, to: MigrationConnection) -> Self {
        MigrationConnections {
            from,
            to
        }
    }
}

impl MigrationConnection {
    pub fn new(s: &String) -> DMResult<MigrationConnection> {
        let db_type = Database::new(s)?;
        let url = Url::parse(&s)?;

        let user = url.username().to_string();
        if user.is_empty()
        {
            return Err(DMError::InvalidConnectionString(String::from("username")));
        }

        let password = match url.password() {
            Some(host) => host.to_string(),
            None => return Err(DMError::InvalidConnectionString(String::from("host"))),
        };

        let host = match url.host_str() {
            Some(host) => host.to_string(),
            None => return Err(DMError::InvalidConnectionString(String::from("host"))),
        };

        let port = match url.port() {
            Some(port) => port,
            None => return Err(DMError::InvalidConnectionString(String::from("port"))),
        };

        let database = match url.path_segments() {
            Some(mut segments) => match segments.next() {
                Some(x) => x.to_string(),
                None => return Err(DMError::InvalidConnectionString(String::from("database"))),
            },
            None => return Err(DMError::InvalidConnectionString(String::from("database"))),
        };


        Ok(MigrationConnection {
            host,
            port,
            user,
            password,
            database,
            db_type,
        })
    }
}