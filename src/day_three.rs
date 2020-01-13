use failure;
use std::collections::{HashMap, HashSet};
use std::fs;

static DATA_FILEPATH: &str = "src/data/day_three.txt";

pub fn first_star() -> Result<i64, failure::Error> {
    // Parse text file.
    let text = fs::read_to_string(DATA_FILEPATH).unwrap();
    let split: Vec<&str> = text.split("\n").collect();
    if split.len() != 2 {
        failure::bail!("Expected two sets of coordinates, found {}", split.len());
    }

    // Call and return result of inner function.
    first_star_inner(split[0], split[1])
}

// Broken out to simplify testing.
fn first_star_inner(first_input: &str, second_input: &str) -> Result<i64, failure::Error> {
    // Get all coordinates.
    let (first_coords, _) = get_all_coordinates_from_input(first_input).unwrap();
    let (second_coords, _) = get_all_coordinates_from_input(second_input).unwrap();

    // Get min distance, if any.
    let mut min_distance = std::i64::MAX;
    for coord in first_coords.intersection(&second_coords) {
        let distance = get_manhattan_distance(coord);
        if distance != 0 && distance < min_distance {
            min_distance = distance;
        }
    }

    if min_distance < std::i64::MAX {
        Ok(min_distance)
    } else {
        failure::bail!("No min distance, they don't overlap!");
    }
}

/// From: https://xlinux.nist.gov/dads/HTML/manhattanDistance.html
/// "The distance between two points measured along axes at right angles.
/// In a plane with p1 at (x1, y1) and p2 at (x2, y2), it is |x1 - x2| + |y1 - y2|."
///
/// Find the Manhattan distance between some Coordinate pair and 0,0.
fn get_manhattan_distance(coordinate: &(i64, i64)) -> i64 {
    coordinate.0.abs() + coordinate.1.abs()
}

pub fn second_star() -> Result<i64, failure::Error> {
    // Parse text file.
    let text = fs::read_to_string(DATA_FILEPATH).unwrap();
    let split: Vec<&str> = text.split("\n").collect();
    if split.len() != 2 {
        failure::bail!("Expected two sets of coordinates, found {}", split.len());
    }

    // Call and return result of inner function.
    second_star_inner(split[0], split[1])
}

fn second_star_inner(first_input: &str, second_input: &str) -> Result<i64, failure::Error> {
    // Get all coordinates and steps.
    let (first_coords, first_steps_map) = get_all_coordinates_from_input(first_input).unwrap();
    let (second_coords, second_steps_map) = get_all_coordinates_from_input(second_input).unwrap();

    // Get min of steps for both wires to reach coord, if exists.
    let mut min_distance = std::i64::MAX;
    for coord in first_coords.intersection(&second_coords) {
        let first_steps = first_steps_map.get(coord).unwrap();
        let second_steps = second_steps_map.get(coord).unwrap();
        if first_steps + second_steps != 0 && first_steps + second_steps < min_distance {
            min_distance = first_steps + second_steps;
        }
    }

    Ok(min_distance)
}

fn get_all_coordinates_from_input(
    input: &str,
) -> Result<(HashSet<(i64, i64)>, HashMap<(i64, i64), i64>), failure::Error> {
    let mut coordinates: HashSet<(i64, i64)> = HashSet::new();
    let mut coords_to_steps: HashMap<(i64, i64), i64> = HashMap::new();

    let mut last_x = 0;
    let mut last_y = 0;
    // Number of steps to reach coordinate (last_x, last_y).
    let mut steps: i64 = 0;

    coordinates.insert((last_x, last_y));
    coords_to_steps.insert((last_x, last_y), steps);
    steps += 1;

    for i in input.split(",") {
        let direction: &str = &i[..1];
        let magnitude: i64 = (&i[1..]).parse().unwrap();
        match direction {
            "U" => {
                let new_y = last_y + magnitude;
                let mut y = last_y + 1;
                while y <= new_y {
                    coordinates.insert((last_x, y));
                    coords_to_steps.insert((last_x, y), steps);
                    y += 1;
                    steps += 1;
                }
                last_y = new_y;
            }
            "D" => {
                let new_y = last_y - magnitude;
                let mut y = last_y - 1;
                while y >= new_y {
                    coordinates.insert((last_x, y));
                    coords_to_steps.insert((last_x, y), steps);
                    y -= 1;
                    steps += 1;
                }
                last_y = new_y;
            }
            "R" => {
                let new_x = last_x + magnitude;
                let mut x = last_x + 1;
                while x <= new_x {
                    coordinates.insert((x, last_y));
                    coords_to_steps.insert((x, last_y), steps);
                    x += 1;
                    steps += 1;
                }
                last_x = new_x;
            }
            "L" => {
                let new_x = last_x - magnitude;
                let mut x = last_x - 1;
                while x >= new_x {
                    coordinates.insert((x, last_y));
                    coords_to_steps.insert((x, last_y), steps);
                    x -= 1;
                    steps += 1;
                }
                last_x = new_x;
            }
            _ => failure::bail!("Invalid instruction type: {}", i),
        }
    }
    Ok((coordinates, coords_to_steps))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct TestCase {
        first_input: &'static str,
        second_input: &'static str,
        expected_distance: i64,
    }

    impl TestCase {
        pub fn new(
            first_input: &'static str,
            second_input: &'static str,
            expected_distance: i64,
        ) -> Self {
            TestCase {
                first_input,
                second_input,
                expected_distance,
            }
        }
    }

    #[test]
    fn test_day_one() {
        let test_cases = vec![
            TestCase::new("R8,U5,L5,D3", "U7,R6,D4,L4", 6),
            TestCase::new(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
                159,
            ),
            TestCase::new(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ];

        for case in test_cases {
            assert_eq!(
                case.expected_distance,
                first_star_inner(case.first_input, case.second_input).unwrap()
            )
        }
    }

    #[test]
    fn test_day_two() {
        let test_cases = vec![
            TestCase::new(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
                610,
            ),
            TestCase::new(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410,
            ),
        ];

        for case in test_cases {
            assert_eq!(
                case.expected_distance,
                second_star_inner(case.first_input, case.second_input).unwrap()
            )
        }
    }

}
