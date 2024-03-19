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


pub trait ObjectStorage {
    fn create_id(&self, space_object: &mut Box<dyn SpaceObject>);

    fn commit(&mut self, space_object: Box<dyn SpaceObject>);

    fn get_object(&self, object_id: &usize) -> &Box<dyn SpaceObject>;

    fn save(&mut self, mut space_object: Box<dyn SpaceObject>) -> usize {
        self.create_id(&mut space_object);
        self.commit(space_object);
        space_object.get_id()
    }
}


#[derive(Debug)]
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

    fn build(x: f32, y: f32, angle: Angle) -> Self {

    }

    fn change(&mut self, rotation_angle: &Angle, movement_speed: f32) {
        let mut x_angle = rotation_angle.as_radians().cos() as f32;
        let mut y_angle = rotation_angle.as_radians().sin() as f32;
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


pub struct SpacePositionsStorage {
    positions: HashMap<usize, Box<dyn SpaceObject>>,
    counter: AtomicUsize,
    rng: ThreadRng
}
impl SpacePositionsStorage {

    pub fn new() -> SpacePositionsStorage {
        let counter: AtomicUsize = AtomicUsize::new(1);
        let mut rng = rand::thread_rng();
        Self {counter: counter, positions: HashMap::new(), rng: rng}
    }

    fn validate(&self, position: SpacePosition) -> bool {
        let ship_plane_space = (x * y) as u32;
        ship_plane_space <= self.plane_space && x >= 0.0 && y >= 0.0
        }
}

impl ObjectStorage for SpacePositionsStorage {
    fn create_id(&self, space_position: &mut Box<dyn SpaceObject>) {
        let object_id = self.counter.fetch_add(1, Ordering::Relaxed);
        space_position.set_id(object_id);
    }
    fn commit(&mut self, space_object: Box<dyn SpaceObject>) {
        self.positions.insert(space_object.get_id(), space_object);
    }

    fn get_object(&self, object_id: &usize) -> &Box<dyn SpaceObject> {
        self.positions.get(object_id).unwrap()
    }

}
