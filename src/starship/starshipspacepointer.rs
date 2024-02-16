use measurements::{angle, Angle};


struct SpaceConnection {
    x: f32,
    y: f32
}

#[derive(Debug)]
pub struct SpacePosition {
    x: f32,
    y: f32,
    angle: Angle
}

impl SpacePosition {
    fn new(x: f32, y: f32, angle: Angle) -> Self {
        Self{x: 0.0, y: 0.0, angle: Angle::from_degrees(0.0)}
    }

    fn change(&mut self, angle: &Angle, movement_speed: f32) {
        let mut x_angle = angle.as_radians().cos() as f32;
        let mut y_angle = angle.as_radians().sin() as f32;
        x_angle = (x_angle * 100.0).round() / 100.0;
        y_angle = (y_angle * 100.0).round() / 100.0;

        self.x = self.x + x_angle * movement_speed;
        self.y = self.y + y_angle * movement_speed;
    }

    fn commit(&self) {

    }

    pub fn validate_position(&self, x: f32, y: f32) -> bool {
        let ship_plane_space = (x * y) as u32;
        ship_plane_space <= self.plane_space && x >= 0.0 && y >= 0.0
        }


}

impl SpacePosition for SpaceObject {}

#[derive(Debug)]
pub struct StarshipSpacePointer{
    position: SpacePosition,
    rotation_speed: angle::Angle,
    movement_speed: f32
}

impl StarshipSpacePointer {

    pub fn new(
        position:SpacePosition, rotation_speed: f64, movement_speed: f32) -> Self{
        StarshipSpacePointer {
            position: position, rotation_speed: Angle::from_degrees(rotation_speed),
            movement_speed: movement_speed
        }
    }

    pub fn rotate_pointer_left(&self) ->  angle::Angle {
        self.angle + self.rotation_speed
    }

    pub fn rotate_pointer_right(&self) -> angle::Angle {
        self.angle - self.rotation_speed
    }

    pub fn move_pointer(&self) -> (f32, f32) {
        self.position.change(self.angle);
        self.position.commit()
    }

}

impl Default for StarshipSpacePointer {
    fn default() -> Self {
        let position: SpacePosition = SpacePosition::new(0.0, 0.0, 0.0);
        let rotation_speed = 90.0;
        let movement_speed = 10.0;
        Self::new(position, rotation_speed, movement_speed)
    }
}

pub enum MovementDirection {
    Left,
    Right
}


#[cfg(test)]
mod tests {
    use super::StarshipSpacePointer;

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
}
