use std::ops::Sub;



#[derive(Debug, Copy, Clone)]
pub struct Hitpoints {
    value: u32
}

impl Hitpoints {
    pub fn is_zero(&self) -> bool {
        self.value > 0
    }
}

impl Sub for Hitpoints {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: self.value - other.value
        }
    }
}


#[derive(Debug)]
pub enum StarshipHealth {
    Alive(Hitpoints),
    Dead
}

impl StarshipHealth {
    pub fn new() -> Self {
        StarshipHealth::Alive(Hitpoints { value: 100 })
    }

    fn build(hitpoints: Hitpoints) -> Self {
        if hitpoints.is_zero() {
            Self::Dead
        }
        else {
            Self::Alive(hitpoints)
        }
    }

    pub fn take_hit(&self, damage: Hitpoints) -> Self {
        if let Self::Alive(hitpoints) = self {
            let hitpoints = *hitpoints - damage;
            Self::build(hitpoints)
        }
        else {
            Self::Dead
        }
    }
}
