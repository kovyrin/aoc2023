#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn each() -> Vec<Self> {
        vec![Self::North, Self::South, Self::West, Self::East]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbour(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::West => Self::new(self.x - 1, self.y),
            Direction::East => Self::new(self.x + 1, self.y),
        }
    }

    pub fn neighbours(&self) -> HashMap<Direction, Self> {
        let mut result = HashMap::new();
        for dir in Direction::each() {
            result.insert(dir, self.neighbour(dir));
        }
        result
    }
}

#[derive(Debug)]
pub struct CharRow {
    row: Vec<char>,
    width: usize,
    default: char,
}

impl CharRow {
    pub fn from_str(input: &str, default: char) -> Self {
        let row = input.trim().chars().collect::<Vec<char>>();
        let width = row.len();

        Self {
            row,
            width,
            default,
        }
    }

    pub fn len(&self) -> usize {
        self.width
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.row.iter()
    }

    pub fn cell(&self, idx: i32) -> &char {
        if idx < 0 || idx as usize >= self.width {
            return &self.default;
        }
        &self.row[idx as usize]
    }
}

// A data structure representing a rectangular map where each cell is a char
// It behaves like a 2D array, but allows out of bounds access (returns the default char)
#[derive(Debug)]
pub struct CharMap {
    map: Vec<CharRow>,
    height: usize,
    default_row: CharRow,
}

impl CharMap {
    pub fn from_str(input: &str, default: char) -> Self {
        let mut map = Vec::new();
        for line in input.lines() {
            map.push(CharRow::from_str(&line, default));
        }

        let width = map[0].len();
        let height = map.len();
        let default_row = CharRow::from_str(&default.to_string().repeat(width), default);

        Self {
            map,
            height,
            default_row,
        }
    }

    pub fn lines(&self) -> impl Iterator<Item = &CharRow> {
        self.map.iter()
    }

    pub fn line(&self, idx: i32) -> &CharRow {
        if idx < 0 || idx as usize >= self.height {
            return &self.default_row;
        }
        &self.map[idx as usize]
    }

    pub fn cell(&self, x: i32, y: i32) -> &char {
        self.line(y).cell(x)
    }

    pub fn cell_for_point(&self, point: &Point) -> &char {
        self.cell(point.x, point.y)
    }

    pub fn find(&self, c: char) -> Option<Point> {
        for (y, line) in self.lines().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == c {
                    return Some(Point::new(x as i32, y as i32));
                }
            }
        }
        None
    }
}
