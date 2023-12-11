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

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn opposite(&self) -> Self {
        self.turn_left().turn_left()
    }

    pub fn turn_right(&self) -> Self {
        self.opposite().turn_left()
    }

    pub fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        }
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

    pub fn with_offset(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
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

    pub fn direction_to(&self, other: &Self) -> Direction {
        if self.x == other.x {
            if self.y < other.y {
                Direction::South
            } else {
                Direction::North
            }
        } else if self.x < other.x {
            Direction::East
        } else {
            Direction::West
        }
    }
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct CharMap {
    map: Vec<CharRow>,
    height: usize,
    default_row: CharRow,
}

impl CharMap {
    pub fn from_dimensions(width: usize, height: usize, default: char) -> Self {
        let default_row = CharRow::from_str(&default.to_string().repeat(width), default);
        let map = vec![default_row.clone(); height];

        Self {
            map,
            height,
            default_row,
        }
    }

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

    pub fn with_padding(&self, x_padding: usize, y_padding: usize) -> Self {
        let mut map = CharMap::from_dimensions(
            self.width() + (x_padding * 2) as usize,
            self.height() + (y_padding * 2) as usize,
            self.default_row.default,
        );
        for (y, line) in self.lines().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                map.set_cell(x + x_padding, y + y_padding, *cell);
            }
        }
        map
    }

    pub fn width(&self) -> usize {
        self.default_row.len()
    }

    pub fn height(&self) -> usize {
        self.height
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
        if x < 0 || y < 0 || x as usize > self.width() || y as usize > self.height() {
            return &self.default_row.default;
        }
        self.line(y).cell(x)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: char) {
        self.map[y].row[x] = value;
    }

    pub fn cell_for_point(&self, point: &Point) -> &char {
        self.cell(point.x, point.y)
    }

    pub fn set_cell_for_point(&mut self, point: &Point, value: char) {
        self.map[point.y as usize].row[point.x as usize] = value;
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

    pub fn print(&self) {
        for line in self.lines() {
            for cell in line.iter() {
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }

    pub fn print_with_current(&self, current: Point, current_char: char) {
        for (y, line) in self.lines().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if x == current.x as usize && y == current.y as usize {
                    print!("{}", current_char);
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
        println!();
    }

    pub fn flood_fill(&mut self, start: Point, fill_with: char) {
        let mut stack = Vec::new();
        stack.push(start);

        while let Some(point) = stack.pop() {
            self.set_cell_for_point(&point, fill_with);

            for (_, neighbour) in point.neighbours() {
                if self.out_of_bounds(&neighbour) {
                    continue;
                }
                let cell = self.cell_for_point(&neighbour);
                if *cell != self.default_row.default {
                    continue;
                }
                stack.push(neighbour);
            }
        }
    }

    pub fn out_of_bounds(&self, point: &Point) -> bool {
        point.x < 0
            || point.y < 0
            || point.x as usize >= self.width()
            || point.y as usize >= self.height()
    }

    pub fn count(&self, c: char) -> usize {
        let mut count = 0;
        for line in self.lines() {
            for cell in line.iter() {
                if *cell == c {
                    count += 1;
                }
            }
        }
        count
    }
}
