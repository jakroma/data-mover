use log::info;

use crate::{db::db_type::DatabaseConnectionType, DMResult};

use super::migrator_steps::{DataProvider, DataReceiver};

pub struct Migrator {
    data_provider: DatabaseConnectionType,
    data_receiver: DatabaseConnectionType,
}

impl Migrator {
    pub async fn run(&self) -> DMResult<()> {
        info!("[Migration] Started!");

        info!("[Migration][Start] Getting data from data provider.");
        self.data_provider.get_data().await?;
        info!("[Migration][End] Getting data from data provider.");

        info!("[Migration][Start] Received data from provider.");
        self.data_receiver.receive_data().await?;
        info!("[Migration][End] Getting data from data provider.");

        info!("[Migration] Ended!");

        Ok(())
    }

    pub fn new(data_provider: DatabaseConnectionType, data_receiver: DatabaseConnectionType) -> Self {
        Migrator { data_provider, data_receiver }
    }
}
