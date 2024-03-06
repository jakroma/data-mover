use crate::{db::mongodb::mongodb::Mongodb, DMResult};

pub trait DataReceiver {
    async fn execute(&self) -> DMResult<()>;
}

impl DataReceiver for Mongodb {
    async fn execute(&self) -> DMResult<()> {
        Ok(())
    }
}