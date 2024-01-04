use rand::{self, Rng};
use rand::rngs;


pub trait MatrixSpace {
    fn col_vec(&self) -> &Vec<u32>;
    fn row_vec(&self) -> &Vec<u32>;
}

#[derive(Debug)]
pub struct QuadraticSpace {
    // we are currently assuming a space is a flat quadratic plane (2D)
    pub width: f32,
    pub height: f32,
    // TODO: random number generator has to move out of the Space
    rng: rngs::ThreadRng,
}

impl QuadraticSpace {
    pub fn build(width: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self { width: width, height: width.clone(),  rng: rng }
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
        let space = QuadraticSpace::build(width);
        panic!("Test!");

        // assert
        assert_eq!(width, space.width);
        assert_eq!(width, space.height);

    }
}
