use std::collections::HashSet;
use Instruction::*;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Count(isize),
    Jump(isize),
    Empty,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Option<Vec<Instruction>> {
    input
        .lines() // Split by lines
        .map(|s| s.trim()) // No extra whitespace
        .map(|s| {
            let mut s = s.split_whitespace();
            let (inst, num) = (s.next()?, s.next()?);

            Some(match inst {
                "acc" => Count(num.parse().ok()?),
                "jmp" => Jump(num.parse().ok()?),
                "nop" => Empty,
                _ => return None,
            })
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> Option<isize> {
    let mut inst_visited = HashSet::new();
    let mut accumulator = 0;
    let mut program_counter = 0;

    loop {
        if inst_visited.contains(&program_counter) {
            return Some(accumulator);
        } else {
            inst_visited.insert(program_counter);
        }

        match input.get(program_counter)? {
            Empty => {
                program_counter += 1;
            }
            Count(count) => {
                program_counter += 1;
                accumulator += count;
            }
            Jump(target) => {
                program_counter = program_counter.wrapping_add(*target as usize);
            }
        }
    }
}
