use async_trait::async_trait;
use log::error;

use crate::DMResult;

use std::{collections::HashMap, sync::{Arc, Mutex}};
pub struct MigrationProviderResult {
    pub stats: Box<MigrationProviderStats>,
    pub migration_order: HashMap<String, Vec<String>>
}

impl MigrationProviderResult {
    pub fn new(stats: MigrationProviderStats, migration_order: HashMap<String, Vec<String>>) -> Self {
        MigrationProviderResult {
            stats: Box::new(stats),
            migration_order
        }
    }
}

pub struct MigrationProviderStats {
    pub tables: Arc<Mutex<u128>>,
    pub entities: Arc<Mutex<u128>>,
}

pub enum MigrationProviderStatsType {
    Tables,
    Entities,
}

impl MigrationProviderStats {
    pub fn new() -> Self {
        MigrationProviderStats {
            tables: Arc::new(Mutex::new(0)),
            entities: Arc::new(Mutex::new(0)),
        }
    }

    pub fn interlocked_increment(&self, stat_type: MigrationProviderStatsType) {
        let (mutex, label) = match stat_type {
            MigrationProviderStatsType::Tables => (&self.tables, "tables"),
            MigrationProviderStatsType::Entities => (&self.entities, "entities"),
        };

        if let Ok(mut guard) = mutex.lock() {
            *guard += 1;
        } else {
            error!("Failed to acquire lock for {}", label);
        }
    }
}

#[async_trait]
pub trait DataProvider {
    async fn get_data(&self, stats: MigrationProviderStats) -> DMResult<MigrationProviderResult>;
}

#[async_trait]
pub trait DataReceiver {
    async fn receive_data(&self) -> DMResult<()>;
}

