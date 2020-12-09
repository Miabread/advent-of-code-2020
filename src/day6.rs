use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<Vec<char>>> {
    let input = input
        .lines() // Split by lines
        .map(|s| s.trim()); // No extra whitespace

    let mut entries = Vec::new();
    let mut entry: Vec<Vec<char>> = Vec::new();

    for line in input {
        if line.is_empty() {
            entries.push(entry);
            entry = Vec::new();
        } else {
            entry.push(line.chars().unique().collect());
        }
    }

    entries.push(entry);

    entries
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<Vec<char>>]) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .iter()
                .flat_map(|person| person.iter())
                .unique()
                .count()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Vec<Vec<char>>]) -> usize {
    input
        .iter()
        .map(|entry| {
            let mut keys = HashMap::<char, usize>::new();

            for person in entry {
                for &key in person.iter().unique() {
                    *keys.entry(key).or_insert(0) += 1;
                }
            }

            keys.values().filter(|&&value| value == entry.len()).count()
        })
        .sum()
}
