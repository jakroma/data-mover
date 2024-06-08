use async_trait::async_trait;

use crate::migrator::migrator_steps::DataReceiver;

use super::{postgresql::Postgresql};

#[async_trait]
impl DataReceiver for Postgresql {
    async fn receive_data(&self) -> crate::DMResult<()> {
        todo!()
    }
}