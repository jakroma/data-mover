pub struct MigrationExecutionSettings {
    pub thread_limit: u32,
    pub pagination_limit: u32,
}

impl MigrationExecutionSettings {
    pub fn new(thread_limit: u32, pagination_limit: u32) -> Self {
        MigrationExecutionSettings {
            thread_limit,
            pagination_limit,
        }
    }
}

