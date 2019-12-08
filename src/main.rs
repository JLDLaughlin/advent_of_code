#![allow(dead_code)] // crate attribute. in main.rs because this is a binary.
mod day_one;
mod day_two;

fn main() {
    println!("day_two first_star: {}", day_two::first_star());
    println!("day_two second_star: {}", day_two::second_star());
}
