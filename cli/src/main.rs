use clap::Parser;
use data_mover_core::{db::postgresql::postgresql::Postgresql, migrator::migrator::Migrator, models::{migration_execution_settings::MigrationExecutionSettings, migration_setting::MigrationSetting}, DMResult};

#[tokio::main]
async fn main() -> DMResult<()> {
    let settings: MigrationSetting = MigrationSetting::parse();

    let connections = settings.create_connections()?;

    let plpsql: Postgresql = Postgresql::new(
        connections.from,
        MigrationExecutionSettings::new(
            settings.concurrent_limit,
            settings.pagination_limit,
        ),
    )
    .await?;

    let migrator: Migrator<Postgresql> = Migrator::new(plpsql);
    migrator.run().await?;

    Ok(())
}
