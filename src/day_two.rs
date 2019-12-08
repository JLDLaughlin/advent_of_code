/// Questions:
/// 1. ? vs unwrap
///     -- ? matches Rust Result<Ok, Err>. Returns Ok(inner) inner for Ok.
///        Returns Err(err) for Err. Reduces boilerplate code.
///     -- unwrap() returns inner T of Some(T) or panic!s (unrecoverable error)
/// 2. Usize for indices
///     -- pointer-sized integer type, ex: 4 bytes for 32 bit target, etc.
///     -- can only index into SLICEs (and any other type linear in memory) with type usize
/// 3. Most efficient way to read ints out of a file
///     -- the std::io::Read Trait:
///         reads bytes from a source
///             read() will attempt to pull bytes from source into a provided buffer
///                 may invoke system calls (? look up ?)
///             other funcs implemented in terms of read()
///         implementors are called Readers
///     -- still not sure... have to measure.
use std::fs; // Filesystem manipulation operations.
static DATA_FILEPATH: &str = "src/data/day_two.txt";

/// First star.
pub fn first_star() -> i32 {
    let mut ints = get_instructions_from_file();
    calculate_instruction_results(&mut ints);
    return ints[0];
}

/// Second star.
/// "determine what pair of inputs produces the output 19690720"
pub fn second_star() -> i32 {
    let goal = 19_690_720;
    let initial_instructions = get_instructions_from_file();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut instructions = initial_instructions.clone();
            instructions[1] = noun;
            instructions[2] = verb;

            calculate_instruction_results(&mut instructions);
            if instructions[0] == goal {
                let result = 100 * noun + verb;
                println!(
                    "Found it! Noun: {}, verb: {}, 100 * noun + verb: {}",
                    noun, verb, result
                );
                return result;
            }
        }
    }
    -1
}

pub fn get_instructions_from_file() -> Vec<i32> {
    let ints_string = fs::read_to_string(DATA_FILEPATH)
        .expect(&format!("Unable to read file: {}", DATA_FILEPATH));
    ints_string
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn calculate_instruction_results(ints: &mut Vec<i32>) {
    let mut instruction_pointer = 0;
    while instruction_pointer < ints.len() {
        let opcode = ints[instruction_pointer];
        match opcode {
            1 | 2 => {
                let first_arg_index = ints[instruction_pointer + 1] as usize;
                let second_arg_index = ints[instruction_pointer + 2] as usize;
                let result_index = ints[instruction_pointer + 3] as usize;
                match opcode {
                    1 => {
                        ints[result_index] = ints[first_arg_index] + ints[second_arg_index];
                    }
                    2 => {
                        ints[result_index] = ints[first_arg_index] * ints[second_arg_index];
                    }
                    _ => {
                        println!("Invalid opcode: {}", opcode);
                    }
                }
            }
            99 => {
                return;
            }
            _ => {
                println!("Invalid opcode: {}", opcode);
            }
        }
        instruction_pointer += 4;
    }
}
