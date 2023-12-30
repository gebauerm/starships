use std::ops::{Sub, Add};
use crate::space::QuadraticSpace;


#[derive(Debug)]
pub struct StarshipSpacePointer {
    // this is currently waste of space, as the pointer can not be negative, but has to be as the delta_x and delta_y can
    x: u32,
    y: u32,
}

impl StarshipSpacePointer {
    pub fn build(space: &mut QuadraticSpace) -> Self {
        let (x, y) = space.get_random_coordinates();
        StarshipSpacePointer { x: x, y: y }
    }

    fn move_pointer_left(&mut self, delta_y: u32) {
        // TODO: this is inefficient, see above comment
        self.y = self.y.sub(delta_y);
    }

    fn move_pointer_right(&mut self, delta_y: u32) {
        self.y = self.y.add(delta_y);
    }

    fn move_pointer_up(&mut self, delta_x: u32) {
        self.x = self.x.add(delta_x);
    }

    fn move_pointer_down(&mut self, delta_x: u32) {
        self.x = self.x.sub(delta_x)
    }

    fn move_pointer_left_up(&mut self, delta_x:u32, delta_y: u32) {
        self.move_pointer_left(delta_y);
        self.move_pointer_up(delta_x);
    }

    fn move_pointer_right_up(&mut self, delta_x: u32, delta_y:u32) {
        self.move_pointer_right(delta_y);
        self.move_pointer_up(delta_x);
    }

    fn move_pointer_left_down(&mut self, delta_x: u32, delta_y: u32) {
        self.move_pointer_left(delta_y);
        self.move_pointer_down(delta_x);
    }

    fn move_pointer_right_down(&mut self, delta_x: u32, delta_y: u32) {
        self.move_pointer_right(delta_y);
        self.move_pointer_down(delta_x);
    }
}


#[derive(Debug)]
pub struct StarShip {
    starshipspacepointer: StarshipSpacePointer,
    speed: u32,
}

impl StarShip{
    pub fn new(space: &mut QuadraticSpace) -> Self {
        let starshippointer = StarshipSpacePointer::build(space);
        StarShip { starshipspacepointer: starshippointer, speed: 1 }
    }

    pub fn move_left(&mut self) {
        self.starshipspacepointer.move_pointer_left(self.speed);
    }

    pub fn move_right(&mut self) {
        self.starshipspacepointer.move_pointer_right(self.speed);
    }

    pub fn move_up(&mut self) {
        self.starshipspacepointer.move_pointer_up(self.speed);
    }

    pub fn move_down(&mut self) {
        self.starshipspacepointer.move_pointer_down(self.speed);
    }

    pub fn move_left_up(&mut self) {
        self.starshipspacepointer.move_pointer_left_up(self.speed, self.speed);
    }

    pub fn move_right_up(&mut self) {
        self.starshipspacepointer.move_pointer_right_up(self.speed, self.speed);
    }

    pub fn move_pointer_left_down(&mut self) {
        self.starshipspacepointer.move_pointer_left_down(self.speed, self.speed);
    }

    pub fn move_pointer_right_down(&mut self) {
        self.starshipspacepointer.move_pointer_right_down(self.speed, self.speed)
    }
}

