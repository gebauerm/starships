use starships::space::QuadraticSpace;
use starships::starship::StarShip;

fn main() {
    let mut space = QuadraticSpace::build(1);
    let mut spaceship_1 = StarShip::new(&mut space);
    let mut spaceship_2 = StarShip::new(&mut space);

    println!("{:?}", spaceship_1);
    spaceship_1.move_left();
    println!("{:?}", spaceship_1);

    println!("{:?}", spaceship_2);
    spaceship_1.move_right();
    println!("{:?}", spaceship_2);

}
