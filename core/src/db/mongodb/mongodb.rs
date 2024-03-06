use mongodb::{bson::doc, options::ClientOptions, Client, Database};

use crate::{
    models::{
        connection::MigrationConnection, migration_execution_settings::MigrationExecutionSettings,
    },
    DMResult,
};

pub struct Mongodb {
    pub client: Client,
    pub exec_settings: MigrationExecutionSettings,
    pub db: Database,
}

impl Mongodb {
    pub async fn new(
        connection_model: MigrationConnection,
        exec_settings: MigrationExecutionSettings,
    ) -> DMResult<Mongodb> {
        let client_options = ClientOptions::parse(connection_model.full_url).await?;
        let client = Client::with_options(client_options)?;

        let db = client.database(&connection_model.database);

        Ok(Mongodb { client, db, exec_settings })
    }
}
