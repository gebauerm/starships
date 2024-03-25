pub mod ship_components;
pub mod starshiphealth;
use crate::space::positions::{SpacePosition, SpacePositionsStorage, RotationDirection};
use crate::starship::ship_components::{StarshipEngine};
use crate::space::{self, QuadraticSpace};
use crate::starship::starshiphealth::StarshipHealth;
use measurements::Angle;



/// Starship provides interaction with the Space Struct. Its components define how the interaction with the
/// Space is been done.
///     E.g. The engine determines movement_speed and rotiation_speed and thus implicitly defines how
///         positions can be manipulated by a ship
#[derive(Debug)]
pub struct StarShip <'a> {
    position_id: usize,
    engine: StarshipEngine,
    starshiphealth: StarshipHealth,
    spacepositionstorage: &'a SpacePositionsStorage
}

impl StarShip<'_>{

    pub fn new(rotation_speed: f64, movement_speed: f32) -> Self {
        let starshipengine = StarshipEngine::new(rotation_speed, movement_speed);
        Self {position_id: 0,  engine: starshipengine, starshiphealth: StarshipHealth::new() }
    }

    pub fn move_starship(&mut self, movement_direction: RotationDirection) {
        let mut space_position = self.spacepositionstorage.get_position(&self.position_id);
        space_position.rotate(self.engine.get_rotation_speed(), movement_direction);
        space_position.change(self.engine.get_movement_speed());
        self.spacepositionstorage.save_position(space_position);
    }

    pub fn take_hit(&mut self, damage: u32) -> &StarshipHealth {
        self.starshiphealth = self.starshiphealth.take_hit(damage);
        &self.starshiphealth
    }

    pub fn set_position_id(&mut self, space_position_id: usize) {
        self.position_id = space_position_id;
    }

    pub fn get_position_id(&self) -> usize {
        self.position_id
    }
}

impl Default for StarShip <'_> {
    fn default() -> Self {
        Self {position_id: 0, engine: StarshipEngine::default(), starshiphealth: StarshipHealth::default() }
    }
}

impl Drop for StarShip <'_> {
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
