use starships::space::QuadraticSpace;
use starships::starship::StarShip;

fn main() {
    let mut space = QuadraticSpace::build(1);
    let mut starship_1 = StarShip::new(&mut space);
    let mut starship_2 = StarShip::new(&mut space);

    println!("{:?}", starship_1);
    starship_1.move_left();
    println!("{:?}", starship_1);

    println!("{:?}", starship_2);
    starship_2.move_down();
    println!("{:?}", starship_2);

}
