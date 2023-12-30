use rand::{rngs, Rng};

pub trait MatrixSpace {
    fn col_vec(&self) -> &Vec<u32>;
    fn row_vec(&self) -> &Vec<u32>;
}


#[derive(Debug)]
pub struct QuadraticSpace {
    // we are currently assuming a space is a flat quadratic plane (2D)
    row_vec: Vec<u32>,
    rng: rngs::ThreadRng,
}

impl QuadraticSpace {
    pub fn build(scale: usize) -> Self {
        // size of the scaling has to be increased, currently smallest value is 1, which is really small
        let row_vec = (1..101).step_by(scale).collect();
        let mut rng = rand::thread_rng();
        Self { row_vec: row_vec, rng: rng }
    }

    fn _buil_space_def_vec(scale: u32) -> Vec<u32> {
        let mut row_vec: Vec<u32>= Vec::new();
        let mut value = 0;
        while value < 100 {
            value += scale;
            row_vec.push(value)
        }
    row_vec
    }

    pub fn get_random_coordinates(&mut self) -> (u32, u32) {
        let x: u32 = self.rng.gen_range(0..self.row_vec().len()) as u32;
        let y: u32 = self.rng.gen_range(0..self.row_vec().len()) as u32;
        (x, y)
    }
}

impl MatrixSpace for QuadraticSpace {

    fn col_vec(&self) -> &Vec<u32> {
        &self.row_vec
    }

    fn row_vec(&self) -> &Vec<u32> {
        &self.row_vec
    }

}


#[cfg(test)]
mod tests {
    use super::{QuadraticSpace, MatrixSpace};

    #[test]
    fn test_normal_vals_quadratic_space_getters() {
        // prepare
        let scale = 10;

        // perform
        let space = QuadraticSpace::build(scale);

        // assert
        let comp:Vec<u32> = (1..101).step_by(10).collect();
        assert_eq!(&comp, space.row_vec());
        assert_eq!(&comp, space.col_vec())

    }
}
