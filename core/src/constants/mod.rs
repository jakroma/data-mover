pub mod postgresql;

pub const TEMP_DIRECTORY: [&str; 2] = [".", "dbcTemp"];
pub const MIGRATION_ORDER_FILE_NAME: &str = "migration_order.json";