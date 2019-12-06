mod day_one;
use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("{:#?}", path);

    println!(
        "dayone partone: {}",
        day_one::get_required_fuel_for_modules()
    );
    println!(
        "dayone parttwo: {}",
        day_one::get_required_fuel_for_modules_and_fuel()
    );
}
