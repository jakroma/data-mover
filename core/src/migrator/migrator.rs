use crate::DMResult;

use super::{data_provider::DataProvider, data_receiver::DataReceiver};

pub struct Migrator<P: DataProvider> {
    data_provider: P,
    // data_receiver: R,
}

impl<P: DataProvider> Migrator<P> {
    pub async fn run(&self) -> DMResult<()> {
        self.data_provider.execute().await?;

        Ok(())
    }

    pub fn new(data_provider: P) -> Self {
        Migrator { data_provider }
    }
}
