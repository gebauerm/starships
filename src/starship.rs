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

    pub fn new(x: f32, y: f32, angle: f64, rotation_speed: f64, movement_speed: f32) -> Self{
        let mut pointer = StarshipSpacePointer::default();
        // TODO: implement that arguments are taken if they exist
        pointer
    }
    pub fn build(space: &mut QuadraticSpace) -> Self {
        // TODO: random space coordinates have to be improved
        let (x, y) = space.get_random_coordinates();
        let angle = Angle::from_degrees(0.0);
        let rotation_speed = Angle::from_degrees(90.0);
        let plane_space = (space.width * space.height) as u32;
        StarshipSpacePointer {
            x: x, y: y, angle: angle, rotation_speed: rotation_speed, movement_speed: 10.0, plane_space: plane_space }
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
        ship_plane_space <= self.plane_space
        }

    fn commit_move(&mut self, x: f32, y: f32, angle: angle::Angle) {
        self.x = x;
        self.y = y;
        self.angle = angle;
    }
}

impl Default for StarshipSpacePointer {
    fn default() -> Self {
        let angle = Angle::from_degrees(0.0);
        let rotation_speed = Angle::from_degrees(90.0);
        let plane_space =100 * 100;
        Self{x: 0.0, y: 0.0, angle: angle, rotation_speed: rotation_speed, movement_speed: 1.0,
            plane_space: plane_space }
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
    pub fn build(space: &mut QuadraticSpace) -> Self {
        let starshippointer = StarshipSpacePointer::build(space);
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



#[cfg(test)]
mod tests {
    use super::StarshipSpacePointer;


    #[test]
    fn test_pointer_movement() {
        // prepare
        let mut starshipspacepointer = StarshipSpacePointer::default();

        // perform
        let angle = starshipspacepointer.rotate_pointer_left();
        let (x,y) = starshipspacepointer.move_pointer(&angle);
        starshipspacepointer.commit_move(x, y, angle);
        let x = 0.0;
        let y = 1.0;

        // assert
        assert_eq!(x, starshipspacepointer.x);
        assert_eq!(y, starshipspacepointer.y)
    }

}
