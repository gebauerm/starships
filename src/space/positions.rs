use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::rngs::ThreadRng;
use measurements::Angle;
use rand::Rng;




pub trait SpaceObject {

    fn get_id(&self) {
    }
}


#[derive(Debug)]
pub struct SpacePosition {
    id: Option<usize>,
    x: f32,
    y: f32,
    angle: Angle
}

impl SpacePosition {
    // space positions shoul be aware of all possible movements, --> space borders shoul somehow be noted by the position
    fn new(x: f32, y: f32, angle: Angle) -> Self {
        Self{id: None, x: 0.0, y: 0.0, angle: Angle::from_degrees(0.0)}
    }

    fn change(&mut self, rotation_angle: &Angle, movement_speed: f32) {
        let mut x_angle = rotation_angle.as_radians().cos() as f32;
        let mut y_angle = rotation_angle.as_radians().sin() as f32;
        x_angle = (x_angle * 100.0).round() / 100.0;
        y_angle = (y_angle * 100.0).round() / 100.0;

        self.x = self.x + x_angle * movement_speed;
        self.y = self.y + y_angle * movement_speed;
    }

    pub fn rotate_left(&self, rotation_speed:Angle ) ->  Angle {
        self.angle + rotation_speed
    }

    pub fn rotate_right(&self, rotation_speed:Angle) -> Angle {
        self.angle - rotation_speed
    }

    fn get_id(&self) -> Option<usize> {
        self.id
    }
}

impl SpaceObject for SpacePosition {}



#[derive(Debug)]
pub struct SpacePositionsList {
    positions: HashMap<usize, SpacePosition>,
    counter: AtomicUsize,
    rng: ThreadRng
}
impl SpacePositionsList {

    pub fn new() -> SpacePositionsList {
        let counter: AtomicUsize = AtomicUsize::new(1);
        let mut rng = rand::thread_rng();
        Self {counter: counter, positions: HashMap::new(), rng: rng}
    }

    pub fn store(&self, space_position: SpacePosition) -> usize {
        self.validate(space_position);
        let uid =self.get_id();
        self.positions.insert(uid, space_position);
        uid
    }

    fn get_id(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }

    fn validate(&self, position: SpacePosition) -> bool {
        let ship_plane_space = (x * y) as u32;
        ship_plane_space <= self.plane_space && x >= 0.0 && y >= 0.0
        }

    fn create_position(&self) -> usize {
        let id =  self.get_id();
        let position = self.init_position();
        self.positions.insert(id, position);
        id
    }

    pub fn get_position(&self, id: &Option<usize>) -> &SpacePosition {;
        if let Option::None = id {
            let id = self.create_position();
        }
        else {
            let id = id.unwrap();
        }

        self.positions.get(id).unwrap()
    }

    fn init_position(&mut self) -> SpacePosition {
        let x: f32 = self.rng.gen_range(0.0..self.width);
        let y: f32 = self.rng.gen_range(0.0..self.height);
        let angle: f64 = self.rng.gen_range(0.0..360.0);
        SpacePosition::new(x, y, Angle::from_degrees(angle))
    }
}
