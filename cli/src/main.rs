use clap::Parser;
use data_mover_core::{models::migration_setting::MigrationSetting, DMResult};

#[tokio::main]
async fn main() -> DMResult<()> {
    let settings: MigrationSetting = MigrationSetting::parse();

    let _connections = settings.create_connections()?;

    Ok(())
}
