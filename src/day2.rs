use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct PasswordEntry {
    range: RangeInclusive<usize>,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Option<Vec<PasswordEntry>> {
    input
        .lines() // Split by lines
        .map(|s| s.trim()) // No extra whitespace
        .filter(|s| !s.is_empty()) // No empty strings
        .map(parse_entry) // Parse into entry
        .collect() // Any parse errors, fail
}

fn parse_entry(input: &str) -> Option<PasswordEntry> {
    // Split the input into the three parts
    let mut input = input.split(' ');
    let (range, letter, password) = (input.next()?, input.next()?, input.next()?);

    // Split the range and parse it into a number range
    let mut range = range.split('-');
    let range = RangeInclusive::new(range.next()?.parse().ok()?, range.next()?.parse().ok()?);

    // Get the required letter
    let letter = letter.chars().next()?;
    let password = password.to_owned();

    Some(PasswordEntry {
        range,
        letter,
        password,
    })
}

#[aoc(day2, part1)]
fn solve_part1(input: &[PasswordEntry]) -> usize {
    input
        .iter()
        .map(|entry| {
            let letter_count = entry
                .password
                .chars() // Split by char
                .filter(|c| *c == entry.letter) // Match letter
                .count(); // Count matches

            // Check if count is in range
            entry.range.contains(&letter_count)
        })
        .filter(|b| *b)
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[PasswordEntry]) -> usize {
    input
        .iter()
        .map(|entry| {
            // Retrieve the chars in the two positions
            let first_pos = entry.password.chars().nth(entry.range.start() - 1)?;
            let second_pos = entry.password.chars().nth(entry.range.end() - 1)?;

            // Exactly one must match the letter
            Some((first_pos == entry.letter) ^ (second_pos == entry.letter))
        })
        .filter(|b| b.unwrap_or(false))
        .count()
}
