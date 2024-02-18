pub mod ship_components;
pub mod starshiphealth;
use crate::starship::ship_components::{MovementDirection, StarshipEngine};
use crate::space::QuadraticSpace;
use crate::starship::starshiphealth::StarshipHealth;


/// Starship provides interaction with the Space Struct. Its components define how the interaction with the
/// Space is been done.
///     E.g. The engine determines movement_speed and rotiation_speed and thus implicitly defines how
///         positions can be manipulated by a ship
#[derive(Debug)]
pub struct StarShip {

    engine: StarshipEngine,
    starshiphealth: StarshipHealth
}

impl StarShip{

    pub fn new(starshippointer: StarshipEngine) -> Self {
        Self { engine: starshippointer, starshiphealth: StarshipHealth::new() }
    }

    pub fn build(space: &mut QuadraticSpace) -> Self {
        let angle = 0.0;
        let rotation_speed = 90.0;
        let movement_speed = 10.0;
        let starshippointer = StarshipEngine::new(
            x, y, angle, rotation_speed, movement_speed, space.widt);
        StarShip::new(starshippointer)
    }

    pub fn move_starship(&mut self, movement_direction: MovementDirection) -> bool {
        let angle = match movement_direction {
            MovementDirection::Left => self.engine.rotate_pointer_left(),
            MovementDirection::Right => self.engine.rotate_pointer_right()
        };
        let (x, y) = self.engine.move_pointer(&angle);
        if self.engine.validate_pointer_position(x, y) {
            self.engine.commit_move(x, y, angle);
            true
        }
        else {
            false
        }
    }

    pub fn take_hit(&mut self, damage: u32) -> &StarshipHealth {
        self.starshiphealth = self.starshiphealth.take_hit(damage);
        &self.starshiphealth
    }

}

impl Default for StarShip {
    fn default() -> Self {
        let starshippointer = StarshipEngine::default();
        Self::new(starshippointer)
    }
}

impl Drop for StarShip {
    fn drop(&mut self) {
        println!("Starship died.")
    }
}


#[cfg(test)]
mod tests {
    use crate::starship::starshiphealth::StarshipHealth;

    use super::{StarShip, MovementDirection};

    #[test]
    fn test_ship_valid_movement() {
        // prepare
        let mut starship = StarShip::default();
        let movement_direction = MovementDirection::Left;

        // perform
        let moved = starship.move_starship(movement_direction);

        // assert
        assert!(moved);
    }

    #[test]
    fn test_ship_invalid_movement() {
        // prepare
        let mut starship = StarShip::default();
        let movement_direction = MovementDirection::Right;

        // perform
        let moved = starship.move_starship(movement_direction);

        // assert
        assert!(!moved);
    }

    #[test]
    fn test_ship_take_hit_alive() {
        let mut starship = StarShip::default();
        let damage = 5;
        let leftover_hitpoints = 95;

        let starshiphealth = starship.take_hit(damage);

        assert_eq!(starshiphealth.get_hitpoints(), &leftover_hitpoints)
    }

    #[test]
    fn test_ship_take_hit_dead() {
        let mut starship = StarShip::default();
        let damage = 100;

        let starshiphealth = starship.take_hit(damage);

        match starshiphealth {
            StarshipHealth::Alive(hitpoints) => assert!(false),
            StarshipHealth::Dead => assert!(true)
        }
    }

}
