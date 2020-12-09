use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Option<Vec<HashSet<String>>> {
    input
        .lines() // Split by lines
        .map(|s| s.trim()) // No extra whitespace
        .batching(|it| {
            let mut output = String::new();
            loop {
                let part = it.next()?;
                if part.is_empty() {
                    break;
                } else {
                    output.push_str(part);
                }
            }
            Some(output)
        })
        .map(|entry| {
            entry
                .split_whitespace()
                .map(|pair| Some(pair.split(':').next()?.to_owned()))
                .collect()
        })
        .collect()
}

pub const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
pub const OPTIONAL_FIELDS: &[&str] = &["cid"];

#[aoc(day4, part1)]
pub fn solve_part1(input: &[HashSet<String>]) -> usize {
    println!("{:#?}", input.get(0));
    input
        .iter()
        .filter(|entry| {
            for &field in REQUIRED_FIELDS {
                if entry.contains(field) {
                    continue;
                } else {
                    return false;
                }
            }
            true
        })
        .count()
}
