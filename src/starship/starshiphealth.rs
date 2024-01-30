
#[derive(Debug)]
pub enum StarshipHealth {
    Alive(u32),
    Dead
}

impl StarshipHealth {
    pub fn new() -> Self {
        StarshipHealth::Alive(100)
    }

    fn build(hitpoints: u32) -> Self {
        if hitpoints <= 0 {
            Self::Dead
        }
        else {
            Self::Alive(hitpoints)
        }
    }

    pub fn take_hit(&self, damage: u32) -> Self {
        match self {
            Self::Alive(hitpoints) => Self::build(*hitpoints - damage),
            Self::Dead => Self::Dead
        }
    }

    pub fn get_hitpoints(&self) -> &u32 {
        match self {
            Self::Alive(hitpoints) => hitpoints,
            Self::Dead => panic!("called unwrap() on a StarshipHealth::Dead!")
        }
    }
}
