use rumqttc::AsyncClient;

use crate::Light;

pub struct Fnordcenter<'a> {
    client: &'a mut AsyncClient,
}

impl<'a> Fnordcenter<'a> {
    pub fn new(client: &'a mut AsyncClient) -> Fnordcenter {
        Fnordcenter { client }
    }

    pub fn light_left(&mut self) -> Light {
        Light::new(self.client, "licht/fnord/links")
    }

    pub fn light_right(&mut self) -> Light {
        Light::new(self.client, "licht/fnord/right")
    }
}

pub struct Plenarsaal<'a> {
    client: &'a mut AsyncClient,
}

impl Plenarsaal<'_> {
    pub fn new(client: &mut AsyncClient) -> Plenarsaal {
        Plenarsaal { client }
    }

    pub fn light_front_wall(&mut self) -> Light {
        Light::new(self.client, "licht/plenar/vornewand")
    }

    pub fn light_front_window(&mut self) -> Light {
        Light::new(self.client, "licht/plenar/vornefenster")
    }

    pub fn light_back_wall(&mut self) -> Light {
        Light::new(self.client, "licht/plenar/hintenwand")
    }

    pub fn light_back_window(&mut self) -> Light {
        Light::new(self.client, "licht/plenar/hintenfenster")
    }
}

pub struct Wohnzimmer<'a> {
    client: &'a mut AsyncClient,
}

impl<'a> Wohnzimmer<'a> {
    pub fn new(client: &'a mut AsyncClient) -> Wohnzimmer {
        Wohnzimmer { client }
    }

    pub fn light_door(&mut self) -> Light {
        Light::new(self.client, "licht/wohnzimmer/tuer")
    }

    pub fn light_middle(&mut self) -> Light {
        Light::new(self.client, "licht/wohnzimmer/mitte")
    }

    pub fn light_kitchen(&mut self) -> Light {
        Light::new(self.client, "licht/wohnzimmer/kueche")
    }

    pub fn light_hall(&mut self) -> Light {
        Light::new(self.client, "licht/wohnzimmer/gang")
    }
}

pub struct Keller<'a> {
    client: &'a mut AsyncClient,
}

impl<'a> Keller<'a> {
    pub fn new(client: &'a mut AsyncClient) -> Keller {
        Keller { client }
    }

    pub fn light_front(&mut self) -> Light {
        Light::new(self.client, "licht/wohnzimmer/vorne")
    }

    pub fn light_middle(&mut self) -> Light {
        Light::new(self.client, "licht/wohnzimmer/mitte")
    }

    pub fn light_solder(&mut self) -> Light {
        Light::new(self.client, "licht/wohnzimmer/loet")
    }
}
