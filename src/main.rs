use starships::space::QuadraticSpace;
use starships::starship::ship_components::StarshipPosition;
use starships::starship::{StarShip};
use starships::starship::ship_components::RotationDirection;


fn main() {


    // desired behavior below:
    let mut space = QuadraticSpace::new(100.0);

    // can be soled over a builder factory, which uses starship_definitions for construction of a ship
    let pos_tuple = space.generate_position();
    let mut position = StarshipPosition::new(pos_tuple.0, pos_tuple.1, pos_tuple.2);
    let mut starship_1 = StarShip::new(position, 30.0, 10.0);

    let pos_tuple = space.generate_position();
    let position = StarshipPosition::new(pos_tuple.0, pos_tuple.1, pos_tuple.2);
    let mut starship_2 = StarShip::new(position, 30.0, 10.0);

    while starship_2.get_health() > 0 {

        println!("{:?}", starship_1);
        let movement_direction = RotationDirection::Right;
        starship_1.move_starship(movement_direction);  // this should change the position in space
        println!("{:?}", starship_1);

        println!("{:?}", starship_2);
        let movement_direction = RotationDirection::Left;
        starship_2.move_starship(movement_direction);
        starship_2.take_hit(50);
        println!("{:?}", starship_2);
    }

}
// TODO: start including timesteps

