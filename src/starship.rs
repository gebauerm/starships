pub mod starshipspacepointer;
pub mod starshiphealth;
use crate::starship::starshipspacepointer::{MovementDirection, StarshipSpacePointer};
use crate::space::QuadraticSpace;
use crate::starship::starshiphealth::{StarshipState, StarshipState, Hitpoints};



#[derive(Debug)]
pub struct StarShip {
    starshipspacepointer: StarshipSpacePointer,
    starshipstate: StarshipState
}

impl StarShip{

    pub fn new(starshippointer: StarshipSpacePointer) -> Self {
        Self { starshipspacepointer: starshippointer, starshipstate: StarshipState::new() }
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

    pub fn take_hit(&mut self, damage: Hitpoints) -> StarshipState {
        self.starshipstate.take_hit(damage)
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
    use super::{StarshipSpacePointer, StarShip, MovementDirection};

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

}
