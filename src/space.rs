pub mod positions;
use rand::Rng;
use crate::space::positions::{SpacePositionsStorage};




#[derive(Debug)]
pub struct QuadraticSpace {
    // we are currently assuming a space is a flat quadratic plane (2D)
    width: f32,
    height: f32,
    space_positions: SpacePositionsStorage
}

impl QuadraticSpace {
    pub fn new(width: f32) -> Self {
        Self { width: width, height: width.clone(), space_positions: SpacePositionsStorage::new() }
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn generate_position(&self) -> (f32, f32, f64) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..self.width);
        let y = rng.gen_range(0.0..self.height);
        let angle = rng.gen_range(0.0..360.0);
        (x, y, angle)
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


