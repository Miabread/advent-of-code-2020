#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Option<Map> {
    Map::parse(input)
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    Tree,
}

/// Data structure to handle the map of trees and free spaces
#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    buffer: Vec<Tile>,
}

impl Map {
    /// Create a Map from a string containing the puzzle input
    pub fn parse(input: &str) -> Option<Self> {
        let mut input = input
            .lines() // Split by lines
            .map(|s| s.trim()) // No extra whitespace
            .filter(|s| !s.is_empty()) // No empty strings
            .peekable();

        let width = input.peek()?.len();
        let height = input.clone().count();

        let buffer = input
            .flat_map(|s| s.chars())
            .map(|c| match c {
                '#' => Some(Tile::Tree),
                '.' => Some(Tile::Empty),
                _ => None,
            })
            .collect::<Option<_>>()?;

        Some(Map {
            width,
            height,
            buffer,
        })
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        // Make sure cord is in world
        if y >= self.height {
            return None;
        }

        let x = x % self.width; // Use modulo to fake repeating world width-wise
        let index = x + y * self.width; // Convert 2d cords to 1d index

        Some(self.buffer[index])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2d {
    x: usize,
    y: usize,
}

const PART1_SLOPE: Vec2d = Vec2d { x: 3, y: 1 };
const PART2_SLOPES: &[Vec2d] = &[
    Vec2d { x: 1, y: 1 },
    Vec2d { x: 3, y: 1 },
    Vec2d { x: 5, y: 1 },
    Vec2d { x: 7, y: 1 },
    Vec2d { x: 1, y: 2 },
];

pub fn solve(input: &Map, slope: Vec2d) -> usize {
    let mut count = 0;
    let mut current = slope;

    while let Some(tree) = input.get(current.x, current.y) {
        if let Tile::Tree = tree {
            count += 1;
        }

        current.x += slope.x;
        current.y += slope.y;
    }

    count
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> usize {
    solve(input, PART1_SLOPE)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> usize {
    PART2_SLOPES
        .iter()
        .map(|&slope| solve(input, slope))
        .product()
}
