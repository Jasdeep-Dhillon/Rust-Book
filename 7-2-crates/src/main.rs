use crate::garden::vegetables::Asparagus;
use crates::front_of_house;
mod garden;

// Binary Crate because it has main
fn main() {
    let plant = Asparagus::default();
    println!("{plant:?}");
    print!("Plant's weight {}", plant.get_weight());

    front_of_house::housing::add_to_waitlist();
    front_of_house::housing::seat_at_table();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_order();
    front_of_house::serving::take_payment();
}
