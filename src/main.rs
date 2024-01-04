use starships::space::QuadraticSpace;
use starships::starship::StarShip;
use starships::starship::MovementDirection;

fn main() {
    // random generator needs to move out of space, as i need to space currently as mutable reference. I dont want that
    let mut space = QuadraticSpace::new(100.0);
    let mut starship_1 = StarShip::build(&mut space);
    let mut starship_2 = StarShip::build(&mut space);

    println!("{:?}", starship_1);
    let movement_direction = MovementDirection::Right;
    let validated = starship_1.move_starship(movement_direction);
    println!("{}", validated);
    println!("{:?}", starship_1);


    println!("{:?}", starship_2);
    let movement_direction = MovementDirection::Left;
    let validated = starship_2.move_starship(movement_direction);
    println!("{}", validated);
    println!("{:?}", starship_2);

}

// TODO: check for cleanups
// TODO: more testing
// TODO: start including timesteps
