use std::collections::HashMap;

use async_trait::async_trait;

use crate::{migrator::migrator_steps::{DataProvider, MigrationProviderResult, MigrationProviderStats}, DMResult};

use super::mongodb::Mongodb;

#[async_trait]
impl DataProvider for Mongodb {
    async fn get_data(&self, stats: MigrationProviderStats) -> DMResult<MigrationProviderResult> {
        Ok(MigrationProviderResult::new(stats, HashMap::new()))
    }
}