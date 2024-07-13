use crate::garden::vegetables::Vegetable;

pub mod garden;

fn main() {
    let carrot = Vegetable {
        color: String::from("orange"),
        calories: 20,
    };

    println!("{carrot:?}")
}
