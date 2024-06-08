use async_trait::async_trait;
use log::{error, info};

use crate::{migrator::migrator_steps::DataReceiver, utils::file_utils::load_definitions, DMResult};

use super::mongodb::Mongodb;

#[async_trait]
impl DataReceiver for Mongodb {
    async fn receive_data(&self) -> DMResult<()> {
        for definition_result in load_definitions() {
            match definition_result {
                Ok(definition) => info!("{:?}", definition),
                Err(e) => error!("Error : {:?}", e),
            }
        }

        Ok(())
    }
}