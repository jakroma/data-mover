use std::sync::{Arc, Mutex};

pub struct MigrationProviderStats {
    pub to_migrate: Arc<Mutex<u128>>,
    pub migrated: Arc<Mutex<u128>>,
    pub errors: Arc<Mutex<u128>>
}