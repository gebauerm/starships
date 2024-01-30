pub mod starshipspacepointer;
pub mod starshiphealth;
use crate::starship::starshipspacepointer::{MovementDirection, StarshipSpacePointer};
use crate::space::QuadraticSpace;
use crate::starship::starshiphealth::StarshipHealth;



#[derive(Debug)]
pub struct StarShip {
    starshipspacepointer: StarshipSpacePointer,
    starshiphealth: StarshipHealth
}

impl StarShip{

    pub fn new(starshippointer: StarshipSpacePointer) -> Self {
        Self { starshipspacepointer: starshippointer, starshiphealth: StarshipHealth::new() }
    }

    pub fn build(space: &mut QuadraticSpace) -> Self {
        let (x, y) = space.get_random_coordinates();
        let angle = 0.0;
        let rotation_speed = 90.0;
        let movement_speed = 10.0;
        let starshippointer = StarshipSpacePointer::new(
            x, y, angle, rotation_speed, movement_speed, space.width, space.height);
        StarShip::new(starshippointer)
    }

    pub fn move_starship(&mut self, movement_direction: MovementDirection) -> bool {
        let angle = match movement_direction {
            MovementDirection::Left => self.starshipspacepointer.rotate_pointer_left(),
            MovementDirection::Right => self.starshipspacepointer.rotate_pointer_right()
        };
        let (x, y) = self.starshipspacepointer.move_pointer(&angle);
        if self.starshipspacepointer.validate_pointer_position(x, y) {
            self.starshipspacepointer.commit_move(x, y, angle);
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
        let starshippointer = StarshipSpacePointer::default();
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

        match starshiphealth {
            StarshipHealth::Alive(hitpoints) => assert_eq!(&leftover_hitpoints, hitpoints),
            StarshipHealth::Dead => assert!(false)
        }
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
