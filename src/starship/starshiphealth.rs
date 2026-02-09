use std::u8;




#[derive(Debug)]
pub enum StarshipHealth {
    Alive(u8),
    Destroyed
}

impl StarshipHealth {
    pub fn new(hitpoints: u8) -> Self {
        Self::Alive(hitpoints)
    }

    pub fn take_hit(&mut self, damage: u8) {
        match self {
            StarshipHealth::Alive(hitpoints) => {
                if *hitpoints > damage {
                    *hitpoints -= damage;
                } else {
                    *self = StarshipHealth::Destroyed;
                }
            },
            StarshipHealth::Destroyed => {}
        }
    }

    pub fn get(&self) -> u8 {
        match self {
            StarshipHealth::Alive(hitpoints) => *hitpoints,
            StarshipHealth::Destroyed => 0
        }
    }

}

impl Default for StarshipHealth {
    fn default() -> Self {
        Self::Alive(100)
    }
}
