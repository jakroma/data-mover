use log::info;

use crate::{db::database::{sort_tables_by_dependencies, DatabaseConnectionType}, migrator::migrator_steps::MigrationProviderStats, utils::file_utils::save_sorted_containers_to_file, DMResult};

use super::migrator_steps::{DataProvider, DataReceiver};

pub struct Migrator {
    data_provider: DatabaseConnectionType,
    data_receiver: DatabaseConnectionType,
}

impl Migrator {
    pub async fn run(&self) -> DMResult<()> {
        info!("[Migration] Started!");

        self.run_provider().await?;

        self.run_receiver().await?;

        info!("[Migration] Ended!");

        Ok(())
    }

    async fn run_receiver(&self) -> Result<(), crate::DMError> {
        info!("[Migration][Receiver] Start.");
        self.data_receiver.receive_data().await?;
        info!("[Migration][Receiver] End.");
        Ok(())
    }
    
    async fn run_provider(&self) -> Result<(), crate::DMError> {
        info!("[Migration][Provider] Start.");
        let result = self.data_provider.get_data(MigrationProviderStats::new()).await?;

        let ordered_migration = sort_tables_by_dependencies(&result.migration_order)?;
        save_sorted_containers_to_file(&ordered_migration)?;

        info!("[Migration][Provider] End.");
        Ok(())
    }
    
    pub fn new(data_provider: DatabaseConnectionType, data_receiver: DatabaseConnectionType) -> Self {
        Migrator { data_provider, data_receiver }
    }
}
