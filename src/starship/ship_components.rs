use measurements::{angle, Angle};



pub enum RotationDirection {
    Left,
    Right
}

#[derive(Debug, PartialEq, Clone)]
pub struct StarshipPosition {
    x: f32,
    y: f32,
    angle: Angle
}


impl StarshipPosition {
    // space positions shoul be aware of all possible movements, --> space borders shoul somehow be noted by the position
    pub fn new(x: f32, y:f32, angle: f64) -> Self {
        Self{x: x, y: y, angle: Angle::from_degrees(angle)}
    }

    pub fn default() -> Self {
        Self {x: 0.0, y: 0.0, angle: Angle::from_degrees(0.0)}
    }

    pub fn change(&mut self, movement_speed: f32) {
        let mut x_angle = self.angle.as_radians().cos() as f32;
        let mut y_angle = self.angle.as_radians().sin() as f32;
        x_angle = (x_angle * 100.0).round() / 100.0;
        y_angle = (y_angle * 100.0).round() / 100.0;

        self.x = self.x + x_angle * movement_speed;
        self.y = self.y + y_angle * movement_speed;
    }

    pub fn rotate(&mut self, rotation_speed:Angle, rotation_direction: RotationDirection) {
        self.angle =  match rotation_direction {
            RotationDirection::Left => self.angle + rotation_speed,
            RotationDirection::Right => self.angle - rotation_speed
            };
    }

}



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

    pub fn get_rotation_speed(&self) -> Angle {
        self.rotation_speed
    }

    pub fn get_movement_speed(&self) -> f32{
        self.movement_speed
    }
}

impl Default for StarshipEngine {
    fn default() -> Self {
        let rotation_speed = 30.0;
        let movement_speed = 10.0;
        Self::new(rotation_speed, movement_speed)
    }
}

