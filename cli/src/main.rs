use clap::Parser;
use data_mover_core::{
    db::database::create_connection,
    migrator::migrator::Migrator,
    models::migration_setting::MigrationSetting,
    DMResult,
};
use env_logger::Env;
use log::{info};

#[tokio::main]
async fn main() -> DMResult<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Migration started.");
    let settings: MigrationSetting = MigrationSetting::parse();
    let connections = settings.create_connections()?;

    let migrator = Migrator::new(
        create_connection(connections.from, &settings).await?,
        create_connection(connections.to, &settings).await?,
    );
    migrator.run().await?;
    info!("Migration ended.");

    Ok(())
}
