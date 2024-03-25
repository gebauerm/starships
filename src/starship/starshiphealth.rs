#[derive(Debug)]
pub struct StarshipHealth {
    hitpoints: u32
}

impl StarshipHealth {
    pub fn new(hitpoints: u32) -> Self {
        Self {hitpoints}
    }

    // what do we do when ship is dead? --> delete it automatically?
}

impl Default for StarshipHealth {
    fn default() -> Self {
        Self { hitpoints: 100 }
    }
}
