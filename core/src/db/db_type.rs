use crate::{DMError, DMResult};

#[derive(Debug, Clone)]
pub enum Database {
    Mongodb,
    Postgressql
}

impl Database {
    pub fn new(s: &String) -> DMResult<Database> {
        if s.starts_with("postgresql") {
            Ok(Database::Postgressql)
        }
        else if s.starts_with("mongodb") {
            Ok(Database::Mongodb)
        }
        else {
            Err(DMError::NotSupportedDb())
        }
    }
}