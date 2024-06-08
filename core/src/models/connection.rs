use url::Url;

use crate::{
    db::db_type::{DatabaseConnectionType, DatabaseType},
    DMError, DMResult,
};

#[derive(Clone)]
pub struct MigrationConnection {
    pub full_url: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub db_type: DatabaseType,
    pub schema: Option<String>,
}

pub struct MigrationConnections {
    pub from: MigrationConnection,
    pub to: MigrationConnection,
}

impl MigrationConnections {
    pub fn new(from: MigrationConnection, to: MigrationConnection) -> Self {
        MigrationConnections { from, to }
    }
}

impl MigrationConnection {
    pub fn new(full_url: &String) -> DMResult<MigrationConnection> {
        let db_type = DatabaseConnectionType::new(full_url)?;
        let url = Url::parse(&full_url)?;

        let user = url.username().to_string();
        if user.is_empty() {
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

        let query_params: url::form_urlencoded::Parse<'_> = url.query_pairs();

        let mut schema = Option::None;

        for query_param in query_params {
            if query_param.0.eq_ignore_ascii_case("schema") {
                schema = Option::Some(query_param.1.to_string());
            }
        }

        Ok(MigrationConnection {
            full_url: full_url.to_owned(),
            host,
            port,
            user,
            password,
            database,
            db_type,
            schema,
        })
    }
}