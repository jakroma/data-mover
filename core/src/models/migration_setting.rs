use clap::Parser;

use crate::DMResult;

use super::connection::{MigrationConnection, MigrationConnections};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct MigrationSetting {
    /// Data provider type
    #[arg(long, value_enum, short = 'f', long)]
    pub migrate_from: String,
    /// Data receiver type
    #[arg(long, value_enum, short = 't')]
    pub migrate_to: String,
    // thread limit
    #[arg(long = "tl", default_value = "4")]
    pub threads_limit: u32,
    // pagination limit
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
