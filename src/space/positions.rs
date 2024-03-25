use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::rngs::ThreadRng;
use measurements::Angle;


enum RotationDirection {
    Left,
    Right
}

pub trait SpaceObject {

    fn get_id(&self) -> usize;

    fn set_id(&mut self, object_id: usize);
}


#[derive(Debug, Clone)]
pub struct SpacePosition {
    id: usize,
    x: f32,
    y: f32,
    angle: Angle
}


impl SpacePosition {
    // space positions shoul be aware of all possible movements, --> space borders shoul somehow be noted by the position
    fn new() -> Self {
        // should randomly generate a position
        Self{id: 0, x: 0.0, y: 0.0, angle: Angle::from_degrees(0.0)}
    }

    pub fn change(&mut self, movement_speed: f32) {
        let mut x_angle = self.angle.as_radians().cos() as f32;
        let mut y_angle = self.angle.as_radians().sin() as f32;
        x_angle = (x_angle * 100.0).round() / 100.0;
        y_angle = (y_angle * 100.0).round() / 100.0;

        self.x = self.x + x_angle * movement_speed;
        self.y = self.y + y_angle * movement_speed;
    }

    pub fn rotate(&self, rotation_speed:Angle, rotation_direction: RotationDirection) {
        self.angle =  match rotation_direction {
            RotationDirection::Left => self.angle + rotation_speed,
            RotationDirection::Right => self.angle - rotation_speed
        }
    }
}

impl SpaceObject for SpacePosition {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, object_id: usize){
        self.id = object_id;
    }
}

// TOOO: storage might have to be connected from the actual object used to interact with storage
#[derive(Debug)]
pub struct SpacePositionsStorage {
    positions: HashMap<usize, SpacePosition>,
    counter: AtomicUsize,
    space_area:f32
}
impl SpacePositionsStorage {

    pub fn new() -> SpacePositionsStorage {
        let counter: AtomicUsize = AtomicUsize::new(1);
        Self {counter: counter, positions: HashMap::new(), space_area: 10.0}
    }

    pub fn build(space_area: u32) -> Self {
        let counter: AtomicUsize = AtomicUsize::new(1);
        Self {counter: counter, positions: HashMap::new(), space_area: space_area as f32}
    }

    fn validate(&self, space_position: SpacePosition) {
        let ship_plane_space = space_position.x * space_position.y;
        if !(ship_plane_space <= self.space_area && space_position.x >= 0.0 && space_position.y >= 0.0) {
            panic!("Ship is outside of space area!")
        }
    }

    fn commit(&mut self, space_position: SpacePosition) -> usize {
        let space_position_id = space_position.get_id();
        self.positions.insert(space_position_id, space_position);
        space_position_id
    }

    pub fn save_position(&mut self, space_position: SpacePosition) -> usize {
        self.validate(space_position);
        self.commit(space_position)
    }

    pub fn get_position(&self, object_id: &usize) -> SpacePosition {
        self.positions.get(object_id).cloned().unwrap()
    }
}
