use rumqttc::{AsyncClient, QoS};

use crate::error::Error;

pub struct Light<'a> {
    client: &'a mut AsyncClient,
    topic: &'a str,
}

impl<'a> Light<'a> {
    pub fn new(client: &'a mut AsyncClient, topic: &'a str) -> Light<'a> {
        Light { client, topic }
    }

    pub async fn set(&mut self, on: bool) -> Result<(), Error> {
        let value = if on { 0x01 } else { 0x00 };
        self.client
            .publish(self.topic, QoS::AtLeastOnce, true, [value])
            .await?;
        Ok(())
    }

    pub async fn on(&mut self) -> Result<(), Error> {
        self.set(true).await
    }

    pub async fn off(&mut self) -> Result<(), Error> {
        self.set(false).await
    }
}
