use crate::{db::{mongodb::mongodb::Mongodb, postgresql::postgresql::Postgresql}, DMResult};

pub trait DataReceiver {
    async fn execute(&self) -> DMResult<()>;
}

impl DataReceiver for Postgresql {
    async fn execute(&self) -> DMResult<()> {
        Ok(())
    }
}

impl DataReceiver for Mongodb {
    async fn execute(&self) -> DMResult<()> {
        Ok(())
    }
}