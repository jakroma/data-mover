use async_trait::async_trait;

use crate::{migrator::migrator_steps::{DataProvider, DataReceiver}, models::{connection::MigrationConnection, migration_execution_settings::MigrationExecutionSettings, migration_setting::MigrationSetting}, DMError, DMResult};

use super::{mongodb::mongodb::Mongodb, postgresql::postgresql::Postgresql};

#[derive(Clone)]
pub enum DatabaseType {
    Postgresql,
    Mongodb
}

pub enum DatabaseConnectionType {
    Postgresql(Postgresql),
    Mongodb(Mongodb),
}

impl DatabaseConnectionType {
    pub fn new(s: &String) -> DMResult<DatabaseType> {
        if s.starts_with("postgresql") {
            Ok(DatabaseType::Postgresql)
        }
        else if s.starts_with("mongodb") {
            Ok(DatabaseType::Mongodb)
        }
        else {
            Err(DMError::NotSupportedDb())
        }
    }
}

pub async fn create_connection(connection_model: MigrationConnection, settings: &MigrationSetting) -> DMResult<DatabaseConnectionType> {
    match connection_model.db_type {
        DatabaseType::Postgresql => {
            let conn = Postgresql::new(connection_model, MigrationExecutionSettings::new(settings.threads_limit, settings.pagination_limit)).await?;
            Ok(DatabaseConnectionType::Postgresql(conn))
        }
        DatabaseType::Mongodb => {
            let conn = Mongodb::new(connection_model, MigrationExecutionSettings::new(settings.threads_limit, settings.pagination_limit)).await?;
            Ok(DatabaseConnectionType::Mongodb(conn))
        }
    }
}

#[async_trait]
impl DataProvider for DatabaseConnectionType {
    async fn get_data(&self) -> DMResult<()> {
        match self {
            DatabaseConnectionType::Postgresql(p) => p.get_data().await,
            DatabaseConnectionType::Mongodb(m) => m.get_data().await,
        }
    }
}

#[async_trait]
impl DataReceiver for DatabaseConnectionType {
    async fn receive_data(&self) -> DMResult<()> {
        match self {
            DatabaseConnectionType::Postgresql(p) => p.receive_data().await,
            DatabaseConnectionType::Mongodb(m) => m.receive_data().await,
        }
    }
}