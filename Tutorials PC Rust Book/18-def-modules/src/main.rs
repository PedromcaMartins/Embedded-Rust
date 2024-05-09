use crate::garden::vegetables::Aspargus;

use restaurant::hosting;

pub mod garden;

fn main() {
    let plant = Aspargus {};
    println!("I'm growing {:?}!", plant);

    hosting::add_to_waitlist();
}
