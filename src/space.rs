pub mod positions;
use rand::Rng;
use rand::rngs;
use crate::space::positions::{SpacePositionsStorage, SpacePosition};
use crate::starship::StarShip;






trait Space {
    pub fn get_position(position_id: usize) -> &SpacePosition {}

    pub fn store_position(positions: SpacePosition) -> usize {}
}


#[derive(Debug)]
pub struct QuadraticSpace {
    // we are currently assuming a space is a flat quadratic plane (2D)
    width: f32,
    height: f32,
    space_positions: SpacePositionsStorage,
    // TODO: random number generator has to move out of the Space
    rng: rngs::ThreadRng,
}

impl QuadraticSpace {
    pub fn new(width: f32) -> Self {
        let rng = rand::thread_rng();
        Self { width: width, height: width.clone(), rng: rng, space_positions: SpacePositionsStorage::new() }
    }

    pub fn register_ship(&mut self, starship: &mut StarShip) {
        let space_position = SpacePosition::new(self.rng);
        let space_position_id = self.space_positions.save_position(space_position);
        starship.set_position_id(&space_position_id);
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


