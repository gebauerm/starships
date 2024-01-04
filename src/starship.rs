use measurements::{angle, Angle};
use crate::space::QuadraticSpace;


#[derive(Debug)]
pub struct StarshipSpacePointer{
    x: f32,
    y: f32,
    angle: angle::Angle,
    rotation_speed: angle::Angle,
    movement_speed: f32,
    plane_space: u32
}

impl StarshipSpacePointer {

    pub fn new(
        x: f32, y: f32, angle: f64, rotation_speed: f64, movement_speed: f32, space_width: f32, space_height: f32) -> Self{
        let plane_space = (space_width * space_height) as u32;
        StarshipSpacePointer {
            x: x, y:y, angle: Angle::from_degrees(angle), rotation_speed: Angle::from_degrees(rotation_speed),
            movement_speed: movement_speed, plane_space: plane_space
        }
    }

    fn rotate_pointer_left(&self) ->  angle::Angle {
        self.angle + self.rotation_speed
    }

    fn rotate_pointer_right(&self) -> angle::Angle {
        self.angle - self.rotation_speed
    }

    fn move_pointer(&self, angle: &Angle) -> (f32, f32) {
        let mut x_angle = angle.as_radians().cos() as f32;
        let mut y_angle = angle.as_radians().sin() as f32;
        x_angle = (x_angle * 100.0).round() / 100.0;
        y_angle = (y_angle * 100.0).round() / 100.0;

        let new_x = self.x + x_angle*self.movement_speed;
        let new_y = self.y + y_angle*self.movement_speed;
        (new_x, new_y)
    }

    fn validate_pointer_position(&self, x: f32, y: f32) -> bool {
        let ship_plane_space = (x * y) as u32;
        ship_plane_space <= self.plane_space && x >= 0.0 && y >= 0.0
        }

    fn commit_move(&mut self, x: f32, y: f32, angle: angle::Angle) {
        self.x = x;
        self.y = y;
        self.angle = angle;
    }
}

impl Default for StarshipSpacePointer {
    fn default() -> Self {
        let x = 0.0;
        let y = 0.0;
        let angle = 0.0;
        let rotation_speed = 90.0;
        let movement_speed = 10.0;
        let space_width = 100.0;
        let space_height = 100.0;
        Self::new(x, y, angle, rotation_speed, movement_speed, space_width, space_height)
    }
}

pub enum MovementDirection {
    Left,
    Right
}



#[derive(Debug)]
pub struct StarShip {
    starshipspacepointer: StarshipSpacePointer,
}

impl StarShip{

    pub fn new(starshippointer: StarshipSpacePointer) -> Self {
        Self { starshipspacepointer: starshippointer }
    }

    pub fn build(space: &mut QuadraticSpace) -> Self {
        let (x, y) = space.get_random_coordinates();
        let angle = 0.0;
        let rotation_speed = 90.0;
        let movement_speed = 10.0;
        let starshippointer = StarshipSpacePointer::new(
            x, y, angle, rotation_speed, movement_speed, space.width, space.height);
        StarShip { starshipspacepointer: starshippointer }
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
}

impl Default for StarShip {
    fn default() -> Self {
        let starshippointer = StarshipSpacePointer::default();
        Self::new(starshippointer)
    }
}



#[cfg(test)]
mod tests {
    use super::{StarshipSpacePointer, StarShip, MovementDirection};


    #[test]
    fn test_pointer_left_movement() {
        // prepare
        let starshipspacepointer = StarshipSpacePointer::default();

        // perform
        let angle = starshipspacepointer.rotate_pointer_left();
        let (x,y) = starshipspacepointer.move_pointer(&angle);

        // assert
        assert_eq!(0.0, x);
        assert_eq!(10.0, y)
    }

    #[test]
    fn test_pointer_right_movement() {
        // prepare
        let starshipspacepointer = StarshipSpacePointer::default();

        // perform
        let angle = starshipspacepointer.rotate_pointer_right();
        let (x,y) = starshipspacepointer.move_pointer(&angle);

        // assert
        assert_eq!(0.0, x);
        assert_eq!(-10.0, y)
    }

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
