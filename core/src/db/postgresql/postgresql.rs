use tokio_postgres::{Client, NoTls};

use crate::{
    error::DMError,
    models::{connection_model::ConnectionModel, migration_setting::MigrationSetting, migration_execution_settings::MigrationExecutionSettings},
};

pub struct Postgresql {
    pub client: Client,
    pub exec_settings: MigrationExecutionSettings
}

#[derive(Debug)]
pub struct TableInfo {
    pub table_schema: String,
    pub table_name: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug)]
pub struct ColumnInfo {
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool
}

impl Postgresql {
    pub async fn new(connection_model: ConnectionModel, exec_settings: MigrationExecutionSettings) -> Result<Postgresql, DMError> {
        let (client, connection) = tokio_postgres::connect(
            &format!(
                "host={0} user={1} password={2} dbname={3} port={4}",
                connection_model.host,
                connection_model.user,
                connection_model.password,
                connection_model.database,
                connection_model.port
            ),
            NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Postgresql { client, exec_settings })
    }
}