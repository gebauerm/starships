pub mod ship_components;
pub mod starshiphealth;
use crate::starship::ship_components::{StarshipEngine, RotationDirection, StarshipPosition};
use crate::starship::starshiphealth::StarshipHealth;



/// Starship provides interaction with the Space Struct. Its components define how the interaction with the
/// Space is been done.
///     E.g. The engine determines movement_speed and rotiation_speed and thus implicitly defines how
///         positions can be manipulated by a ship
#[derive(Debug)]
pub struct StarShip {
    position: StarshipPosition,
    engine: StarshipEngine,
    starshiphealth: StarshipHealth
}

impl StarShip {

    pub fn new(position: StarshipPosition, rotation_speed: f64, movement_speed: f32) -> Self {
        let starshipengine = StarshipEngine::new(rotation_speed, movement_speed);
        Self {position: position,  engine: starshipengine, starshiphealth: StarshipHealth::default() }
    }

    pub fn move_starship(&mut self, rotation_direction: RotationDirection) {
        self.position.rotate(self.engine.get_rotation_speed(), rotation_direction);
        self.position.change(self.engine.get_movement_speed());
    }

    pub fn take_hit(&mut self, damage: u8){
        self.starshiphealth.take_hit(damage);
    }

    pub fn get_health(&self) -> u8{
        self.starshiphealth.get()
}
}

impl Default for StarShip {
    fn default() -> Self {
        Self {position: StarshipPosition::default(), engine: StarshipEngine::default(), starshiphealth: StarshipHealth::default() }
    }
}

impl Drop for StarShip {
    fn drop(&mut self) {
        match self.starshiphealth {
            StarshipHealth::Alive(_) => println!("Starship is still alive."),
            StarshipHealth::Destroyed =>
        println!("Starship died.")
        }
    }
}



#[cfg(test)]
mod tests {
    use super::{StarShip, RotationDirection};

    #[test]
    fn test_ship_valid_movement() {
        // prepare
        let mut starship = StarShip::default();
        let initial_position = starship.position.clone();
        let movement_direction = RotationDirection::Left;

        // perform
        starship.move_starship(movement_direction);

        // assert
        assert_ne!(initial_position, starship.position);
    }

    #[test]
    fn test_ship_take_hit_alive() {
        let mut starship = StarShip::default();
        let damage = 100;

        starship.take_hit(damage);

        assert_eq!(starship.get_health(), 0);
    }

}
