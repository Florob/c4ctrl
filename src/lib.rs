use rumqttc::{AsyncClient, MqttOptions, QoS};

mod error;
mod events;
mod light;
mod rooms;

pub use error::Error;
pub use events::*;
pub use light::Light;
pub use rooms::*;

pub struct C4Ctrl {
    client: AsyncClient,
}

impl C4Ctrl {
    pub fn new(domain: impl Into<String>, port: u16) -> (C4Ctrl, EventLoop) {
        let options = MqttOptions::new("c4ctrl", domain, port);

        let (client, eventloop) = AsyncClient::new(options, 32);

        (C4Ctrl { client }, EventLoop { eventloop })
    }

    pub async fn subscribe(&mut self) -> Result<(), Error> {
        self.client.subscribe("licht/+/+", QoS::AtMostOnce).await?;
        Ok(())
    }

    pub fn fnordcenter(&mut self) -> Fnordcenter {
        Fnordcenter::new(&mut self.client)
    }

    pub fn plenarsaal(&mut self) -> Plenarsaal {
        Plenarsaal::new(&mut self.client)
    }

    pub fn wohnzimmer(&mut self) -> Wohnzimmer {
        Wohnzimmer::new(&mut self.client)
    }

    pub fn keller(&mut self) -> Keller {
        Keller::new(&mut self.client)
    }
}

pub struct EventLoop {
    eventloop: rumqttc::EventLoop,
}

impl EventLoop {
    pub async fn poll(&mut self) -> Result<Event, Error> {
        use rumqttc::{Event, Packet};

        loop {
            let event = self.eventloop.poll().await?;
            if let Event::Incoming(Packet::Publish(pu)) = event {
                let state = pu.payload == [0x01u8][..];
                return Ok(match pu.topic.as_str() {
                    // Fnordcenter
                    "licht/fnord/links" => FnordcenterEvent::LightLeft(state).into(),
                    "licht/fnord/rechts" => FnordcenterEvent::LightRight(state).into(),

                    // Plenarsaal
                    "licht/plenar/vornewand" => PlenarsaalEvent::LightFrontWall(state).into(),
                    "licht/plenar/vornefenster" => PlenarsaalEvent::LightFrontWindow(state).into(),
                    "licht/plenar/hintenwand" => PlenarsaalEvent::LightBackWall(state).into(),
                    "licht/plenar/hintenfenster" => PlenarsaalEvent::LightBackWindow(state).into(),

                    // Wohnzimmer
                    "licht/wohnzimmer/tuer" => WohnzimmerEvent::LightDoor(state).into(),
                    "licht/wohnzimmer/mitte" => WohnzimmerEvent::LightMiddle(state).into(),
                    "licht/wohnzimmer/kueche" => WohnzimmerEvent::LightKitchen(state).into(),
                    "licht/wohnzimmer/gang" => WohnzimmerEvent::LightHall(state).into(),

                    // Keller
                    "licht/keller/vorne" => KellerEvent::LightFront(state).into(),
                    "licht/keller/mitte" => KellerEvent::LightMiddle(state).into(),
                    "licht/keller/loet" => KellerEvent::LightSolder(state).into(),

                    _ => continue,
                });
            }
        }
    }
}
