pub mod positions;
use rand::Rng;
use rand::rngs;
use crate::space::positions::{SpacePositionsList, SpacePosition};






trait Space {
    pub fn get_position(position_id: usize) {}

    pub fn store_position(positions: SpacePosition) {}
}


#[derive(Debug)]
pub struct QuadraticSpace {
    // we are currently assuming a space is a flat quadratic plane (2D)
    pub width: f32,
    pub height: f32,
    pub positions: SpacePositionsList,
    // TODO: random number generator has to move out of the Space
    rng: rngs::ThreadRng,
}

impl QuadraticSpace {
    pub fn new(width: f32) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self { width: width, height: width.clone(),  rng: rng, positions: SpacePositionsList::new() }
    }

    pub fn get_position(&self, position_id: &usize) {
        self.positions.get_position(&position_id)
    }
}

impl Space for QuadraticSpace {}





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


