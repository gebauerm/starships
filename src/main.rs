use std::cmp::Ordering;
use std::sync::atomic::AtomicUsize;

use starships::space::QuadraticSpace;
use starships::starship::{self, StarShip};
use starships::starship::starshipspacepointer::MovementDirection;


fn main() {


    // desired behavior below:
    let mut space = QuadraticSpace::new(100.0);

    // can be soled over a builder factory, which uses starship_definitions for construction of a ship
    let mut starship_1 = StarShip::new(rotation_speed, movement_speed);
    let mut starship_2 = StarShip::new(rotiation_speed, movement_speed);

    starship_1.register_ship(space); //creates space position in space and stores a reference of it in the ship
    starship_2.register_ship(space);

    println!("{:?}", starship_1);
    let movement_direction = MovementDirection::Right;
    let validated = starship_1.move_starship(movement_direction);  // this should change the position in space
    println!("{}", validated);
    println!("{:?}", starship_1);


    println!("{:?}", starship_2);
    let movement_direction = MovementDirection::Left;
    let validated = starship_2.move_starship(movement_direction);
    println!("{}", validated);
    println!("{:?}", starship_2);

}
// TODO: start including timesteps

