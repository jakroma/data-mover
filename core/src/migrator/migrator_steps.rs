use async_trait::async_trait;

use crate::DMResult;

#[async_trait]
pub trait DataProvider {
    async fn get_data(&self) -> DMResult<()>;
}

#[async_trait]
pub trait DataReceiver {
    async fn receive_data(&self) -> DMResult<()>;
}