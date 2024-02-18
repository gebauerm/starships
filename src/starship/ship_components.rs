use measurements::{angle, Angle};
use crate::space::SpacePosition;



#[derive(Debug)]
pub struct StarshipEngine{
    rotation_speed: angle::Angle,
    movement_speed: f32
}

impl StarshipEngine {

    pub fn new(
        rotation_speed: f64, movement_speed: f32) -> Self{
        StarshipEngine {
            rotation_speed: Angle::from_degrees(rotation_speed),
            movement_speed: movement_speed
        }
    }
}

impl Default for StarshipEngine {
    fn default() -> Self {
        let position: SpacePosition = SpacePosition::new(0.0, 0.0, 0.0);
        let rotation_speed = 90.0;
        let movement_speed = 10.0;
        Self::new(rotation_speed, movement_speed)
    }
}

pub enum MovementDirection {
    Left,
    Right
}


#[cfg(test)]
mod tests {
    use super::StarshipEngine;

    #[test]
    fn test_pointer_left_movement() {
        // prepare
        let starshipspacepointer = StarshipEngine::default();

        // perform
        let angle = starshipspacepointer.rotate_pointer_left();
        let (x,y) = starshipspacepointer.move_pointer();

        // assert
        assert_eq!(0.0, x);
        assert_eq!(10.0, y)
    }

    #[test]
    fn test_pointer_right_movement() {
        // prepare
        let starshipspacepointer = StarshipEngine::default();

        // perform
        let angle = starshipspacepointer.rotate_pointer_right();
        let (x,y) = starshipspacepointer.move_pointer();

        // assert
        assert_eq!(0.0, x);
        assert_eq!(-10.0, y)
    }
}
