use async_trait::async_trait;

use crate::{migrator::migrator_steps::DataProvider, DMResult};

use super::mongodb::Mongodb;

#[async_trait]
impl DataProvider for Mongodb {
    async fn get_data(&self) -> DMResult<()> {
        Ok(())
    }
}