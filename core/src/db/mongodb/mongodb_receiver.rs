use async_trait::async_trait;
use log::info;

use crate::{migrator::migrator_steps::DataReceiver, parsers::parse_definitions, utils::file_utils::load_sorted_containers_from_file, DMResult};

use super::mongodb::Mongodb;

#[async_trait]
impl DataReceiver for Mongodb {
    async fn receive_data(&self) -> DMResult<()> {
        let migration_containers = load_sorted_containers_from_file()?;

        for container in migration_containers {
            let definition = parse_definitions(container);
            info!("{:?}", definition);
        }

        Ok(())
    }
}