#[derive(Debug)]
pub enum Event {
    Fnordcenter(FnordcenterEvent),
    Plenarsaal(PlenarsaalEvent),
    Wohnzimmer(WohnzimmerEvent),
    Keller(KellerEvent),
}

impl From<FnordcenterEvent> for Event {
    fn from(v: FnordcenterEvent) -> Self {
        Self::Fnordcenter(v)
    }
}

impl From<PlenarsaalEvent> for Event {
    fn from(v: PlenarsaalEvent) -> Self {
        Self::Plenarsaal(v)
    }
}

impl From<WohnzimmerEvent> for Event {
    fn from(v: WohnzimmerEvent) -> Event {
        Event::Wohnzimmer(v)
    }
}

impl From<KellerEvent> for Event {
    fn from(v: KellerEvent) -> Self {
        Self::Keller(v)
    }
}

#[derive(Debug)]
pub enum FnordcenterEvent {
    LightLeft(bool),
    LightRight(bool),
}

#[derive(Debug)]
pub enum PlenarsaalEvent {
    LightFrontWall(bool),
    LightFrontWindow(bool),
    LightBackWall(bool),
    LightBackWindow(bool),
}

#[derive(Debug)]
pub enum WohnzimmerEvent {
    LightDoor(bool),
    LightMiddle(bool),
    LightKitchen(bool),
    LightHall(bool),
}

#[derive(Debug)]
pub enum KellerEvent {
    LightFront(bool),
    LightMiddle(bool),
    LightSolder(bool),
}
