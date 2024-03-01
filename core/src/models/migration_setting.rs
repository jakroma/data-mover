use clap::Parser;

use crate::DMResult;

use super::connection::{MigrationConnection, MigrationConnections};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct MigrationSetting {
    /// Data provider type
    #[arg(value_enum, short = 'f', long)]
    pub migrate_from: String,
    /// Data receiver type
    #[arg(value_enum, short = 't', long)]
    pub migrate_to: String,

    // concurrency limit
    #[arg(long, short = 'c', default_value = "4")]
    pub concurrent_limit: u32,

    // pagination limit for query concurrency limit
    #[arg(long, short = 'p', default_value = "50")]
    pub pagination_limit: u32,
}

impl MigrationSetting {
    pub fn create_connections(&self) -> DMResult<MigrationConnections> {
        Ok(MigrationConnections::new(
            MigrationConnection::new(&self.migrate_from)?,
            MigrationConnection::new(&self.migrate_to)?,
        ))
    }
}
