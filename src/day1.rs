use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines() // Split by lines
        .map(|s| s.trim()) // No whitespace
        .filter(|s| !s.is_empty()) // No empty strings
        .map(|s| s.parse().unwrap()) // Parse into numbers
        .collect() // Collect all good results or fail
}

pub const TARGET: u32 = 2020;

#[aoc(day1, part1, for_loop)]
pub fn solve_part1_for_loop(input: &[u32]) -> Option<u32> {
    for x in input {
        for y in input {
            if x + y == TARGET {
                return Some(x * y);
            }
        }
    }
    None
}

#[aoc(day1, part1, iter)]
pub fn solve_part1_iter(input: &[u32]) -> Option<u32> {
    let (x, y) = input
        .iter()
        .cartesian_product(input)
        .find(|&(x, y)| x + y == TARGET)?;

    Some(x * y)
}

#[aoc(day1, part2, for_loop)]
pub fn solve_part2_for_loop(input: &[u32]) -> Option<u32> {
    for x in input {
        for y in input {
            for z in input {
                if x + y + z == TARGET {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

#[aoc(day1, part2, iter)]
pub fn solve_part2_iter(input: &[u32]) -> Option<u32> {
    let ((x, y), z) = input
        .iter()
        .cartesian_product(input)
        .cartesian_product(input)
        .find(|&((x, y), z)| x + y + z == TARGET)?;

    Some(x * y * z)
}
