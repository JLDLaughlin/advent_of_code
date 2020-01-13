#![allow(dead_code)] // crate attribute. in main.rs because this is a binary.
mod day_one;
mod day_three;
mod day_two;

fn main() {
    //    println!("day_two first_star: {}", day_two::first_star());
    //    println!("day_two second_star: {}", day_two::second_star());
    //    println!("day_three first star: {}", day_three::first_star().unwrap());
    println!(
        "day_three first star: {}",
        day_three::second_star().unwrap()
    );
}
