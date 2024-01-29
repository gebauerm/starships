use std::ops::Sub;



#[derive(Debug)]
struct Hitpoints {
    value: u32
}

impl Hitpoints {
    fn is_zero(&self) -> bool {
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
pub enum StarshipState {
    Alive(Hitpoints),
    Dead
}

impl StarshipState {
    pub fn new() -> Self {
        StarshipState::Alive(Hitpoints { value: 100 })
    }

    fn build(hitpoints: Hitpoints) -> Self {
        if hitpoints.is_zero() {
            Self::Dead
        }
        else {
            Self::Alive(hitpoints)
        }
    }

    pub fn take_hit(self, damage: Hitpoints) -> Self {
        if let Self::Alive(hitpoints) = self {
            let hitpoints = hitpoints - damage;
            Self::build(hitpoints - damage)
        }
        else {
            Self::Dead
        }
    }
}
