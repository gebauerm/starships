use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::starship::starshipspacepointer::SpacePosition;

use rand::{self, Rng};
use rand::rngs;

trait SpaceObject {

}

struct SpaceConnection {
    space: QuadraticSpace
}
impl SpaceConnection {
    pub fn commit(&self, object: impl SpaceObject) {
        self.space.shippositions.store(object);
    }
}


#[derive(Debug)]
struct ShipPositionsList {
    positions: HashMap<usize, SpacePosition>,
    counter: AtomicUsize
}
impl ShipPositionsList {
    pub fn store(&self, space_position: SpacePosition) -> usize {
        self.validate(space_position);
        let uid =self.get_id();
        self.positions.insert(uid, space_position);
        uid
    }
    fn validate(&self, space_position: SpacePosition) {

    }

    fn get_id(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}



#[derive(Debug)]
pub struct QuadraticSpace {
    // we are currently assuming a space is a flat quadratic plane (2D)
    pub width: f32,
    pub height: f32,
    pub shippositions: ShipPositionsList,
    // TODO: random number generator has to move out of the Space
    rng: rngs::ThreadRng,
}

impl QuadraticSpace {
    pub fn new(width: f32) -> Self {
        let mut rng = rand::thread_rng();
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self { width: width, height: width.clone(),  rng: rng, shippositions: ShipPositionsList { positions: ShipPositionsList::new() } }
    }

    pub fn get_random_coordinates(&mut self) -> (f32, f32) {
        let x: f32 = self.rng.gen_range(0.0..self.width);
        let y: f32 = self.rng.gen_range(0.0..self.height);
        (x, y)
    }
}


#[cfg(test)]
mod tests {
    use super::QuadraticSpace;

    #[test]
    fn test_normal_vals_quadratic_space_getters() {
        // prepare
        let width: f32 = 10.0;

        // perform
        let space = QuadraticSpace::new(width);

        // assert
        assert_eq!(width, space.width);
        assert_eq!(width, space.height);

    }
}
