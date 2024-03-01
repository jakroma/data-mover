use crate::{DMError, DMResult};

#[derive(Debug, Clone)]
pub enum Database {
    Mongodb,
    Postgressql
}

impl Database {
    pub fn new(s: &String) -> DMResult<Database> {
        if s.eq_ignore_ascii_case("postgresql") {
            Ok(Database::Postgressql)
        }
        else if s.eq_ignore_ascii_case("mongodb") {
            Ok(Database::Mongodb)
        }
        else {
            Err(DMError::NotSupportedDb())
        }
    }
}