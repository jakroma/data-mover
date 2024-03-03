pub struct MigrationExecutionSettings {
    pub concurrent_limit: u32,
    pub pagination_limit: u32,
}

impl MigrationExecutionSettings {
    pub fn new(concurrent_limit: u32, pagination_limit: u32) -> Self {
        MigrationExecutionSettings {
            concurrent_limit: concurrent_limit,
            pagination_limit: pagination_limit,
        }
    }
}

