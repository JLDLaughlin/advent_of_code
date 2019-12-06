use std::fs;

// In Rust, global variables are static variables: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable
// Static variables are similar to constants, but:
//     - always have a fixed address in memory
//     - can be mutable (although accessing/modifying is unsafe)
static DATA_FILEPATH: &str = "src/data/day_one.txt";

/// First star.
/// Get the fuel required given the masses of each module.
pub fn get_required_fuel_for_modules() -> f32 {
    get_required_fuel_given_input_file_and_function(DATA_FILEPATH, |input: f32| {
        (input / 3.0).floor() - 2.0
    })
}

/// Second star.
/// Get the fuel required given the masses of each module AND
/// and required fuel.
pub fn get_required_fuel_for_modules_and_fuel() -> f32 {
    get_required_fuel_given_input_file_and_function(
        DATA_FILEPATH,
        Box::from(get_required_fuel_decaying),
    )
}

/// Calculate fuel requirements for module mass
/// PLUS add fuel for fuel (recursive bit).
fn get_required_fuel_decaying(input: f32) -> f32 {
    if input <= 0.0 {
        return 0.0;
    }

    let current_sum: f32 = ((input / 3.0).floor() - 2.0).max(0.0);
    current_sum + get_required_fuel_decaying(current_sum)
}

fn get_required_fuel_given_input_file_and_function<F>(filepath: &str, f: F) -> f32
where
    F: Fn(f32) -> f32,
{
    let mass_text = fs::read_to_string(filepath).unwrap();
    let masses: Vec<&str> = mass_text.split("\n").collect();

    let mut sum: f32 = 0.0;
    for m in masses {
        sum += f(m.parse().unwrap());
    }
    sum
}


