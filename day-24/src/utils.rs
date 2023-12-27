use std::{
    collections::HashMap,
    fmt::Formatter,
    hash::Hash,
    hash::Hasher,
    ops::{Range, Sub},
};

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

    pub fn delta(&self) -> Point<i64> {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
            Direction::East => Point::new(1, 0),
        }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: std::ops::Add<Output = T>
        + Sub<Output = T>
        + Copy
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + num_traits::sign::Signed
        + From<i32>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn with_offset(&self, x: T, y: T) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn manhattan_distance(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbour(&self, dir: Direction) -> Point<T> {
        let one = T::from(1);
        match dir {
            Direction::North => Point::<T>::new(self.x, self.y - one),
            Direction::South => Self::new(self.x, self.y + one),
            Direction::West => Self::new(self.x - one, self.y),
            Direction::East => Self::new(self.x + one, self.y),
        }
    }

    pub fn neighbours(&self) -> HashMap<Direction, Self> {
        let mut result = HashMap::new();
        for dir in Direction::each() {
            result.insert(dir, self.neighbour(dir));
        }
        result
    }

    pub fn neighbours_list(&self) -> Vec<Self> {
        let mut result = Vec::with_capacity(4);
        for dir in Direction::each() {
            result.push(self.neighbour(dir));
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

    pub fn vector_to(&self, other: &Self) -> Point<T> {
        Point::new(other.x - self.x, other.y - self.y)
    }
}

impl<T> std::ops::Add for Point<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::fmt::Display for Point<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T> std::fmt::Debug for Point<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharRow {
    row: Vec<char>,
    width: usize,
    default: char,
}

impl CharRow {
    pub fn from_str(input: &str, default: char) -> Self {
        let row = input.chars().collect::<Vec<char>>();
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

    pub fn cell(&self, idx: i64) -> &char {
        if idx < 0 || idx as usize >= self.width {
            return &self.default;
        }
        &self.row[idx as usize]
    }

    pub fn slice(&self, range: &Range<i64>) -> Vec<char> {
        let mut result = Vec::with_capacity(range.clone().count());
        for idx in range.start..range.end {
            result.push(*self.cell(idx));
        }
        result
    }
}

// A data structure representing a rectangular map where each cell is a char
// It behaves like a 2D array, but allows out of bounds access (returns the default char)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn from_iter<T>(lines: impl Iterator<Item = T>, default: char) -> Self
    where
        T: AsRef<str>,
    {
        let mut map = Vec::new();
        for line in lines {
            map.push(CharRow::from_str(line.as_ref(), default));
        }

        let width = map[0].len();
        assert!(width > 0);

        let height = map.len();
        assert!(height > 0);

        let default_row = CharRow::from_str(&default.to_string().repeat(width), default);
        assert!(default_row.len() == width);

        Self {
            map,
            height,
            default_row,
        }
    }

    pub fn from_str_with_trim(input: &str, default: char) -> Self {
        let iter = input.lines().map(|line| line.trim());
        Self::from_iter(iter, default)
    }

    pub fn from_str(input: &str, default: char) -> Self {
        Self::from_iter(input.lines(), default)
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

    pub fn top_left(&self) -> Point<i64> {
        Point::new(0, 0)
    }

    pub fn top_right(&self) -> Point<i64> {
        Point::new(self.width() as i64 - 1, 0)
    }

    pub fn bottom_left(&self) -> Point<i64> {
        Point::new(0, self.height() as i64 - 1)
    }

    pub fn bottom_right(&self) -> Point<i64> {
        Point::new(self.width() as i64 - 1, self.height() as i64 - 1)
    }

    pub fn lines(&self) -> impl Iterator<Item = &CharRow> {
        self.map.iter()
    }

    pub fn lines_mut(&mut self) -> impl IntoIterator<Item = &mut CharRow> {
        self.map.iter_mut()
    }

    pub fn line(&self, idx: i64) -> &CharRow {
        if idx < 0 || idx as usize >= self.height {
            return &self.default_row;
        }
        &self.map[idx as usize]
    }

    pub fn cell(&self, x: i64, y: i64) -> &char {
        if x < 0 || y < 0 || x as usize > self.width() || y as usize > self.height() {
            return &self.default_row.default;
        }
        self.line(y).cell(x)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: char) {
        self.map[y].row[x] = value;
    }

    pub fn cell_for_point(&self, point: &Point<i64>) -> &char {
        self.cell(point.x, point.y)
    }

    pub fn cell_digit_for_point(&self, point: &Point<i64>) -> i64 {
        let cell = self.cell_for_point(point);
        (*cell as i64) - ('0' as i64)
    }

    pub fn set_cell_for_point(&mut self, point: &Point<i64>, value: char) {
        if self.out_of_bounds(point) {
            return;
        }
        self.map[point.y as usize].row[point.x as usize] = value;
    }

    pub fn find(&self, c: char) -> Option<Point<i64>> {
        for (y, line) in self.lines().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == c {
                    return Some(Point::new(x as i64, y as i64));
                }
            }
        }
        None
    }

    pub fn find_all(&self, c: char) -> Vec<Point<i64>> {
        let mut result = Vec::new();
        for (y, line) in self.lines().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == c {
                    result.push(Point::new(x as i64, y as i64));
                }
            }
        }
        result
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

    pub fn print_with_current(&self, current: &Point<i64>, current_char: char) {
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

    pub fn flood_fill(&mut self, start: Point<i64>, fill_with: char) {
        let mut stack = Vec::new();
        stack.push(start);

        while let Some(point) = stack.pop() {
            self.set_cell_for_point(&point, fill_with);

            for neighbour in point.neighbours_list() {
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

    pub fn out_of_bounds(&self, point: &Point<i64>) -> bool {
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

    pub fn copy_from_vec(&mut self, new_map: &Vec<Vec<char>>) {
        for (y, line) in new_map.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                self.set_cell(x, y, *cell);
            }
        }
    }

    pub fn transpose(&self) -> CharMap {
        let mut new_map =
            CharMap::from_dimensions(self.height(), self.width(), self.default_row.default);

        for (y, line) in self.lines().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                new_map.set_cell(y, x, *cell);
            }
        }
        new_map
    }

    pub fn flip_horizontal(&self) -> CharMap {
        let mut new_map = self.clone();
        for row in new_map.lines_mut() {
            row.row.reverse();
        }
        new_map
    }

    pub fn rotate_left(&self) -> CharMap {
        self.flip_horizontal().transpose()
    }

    pub fn rotate_right(&self) -> CharMap {
        self.transpose().flip_horizontal()
    }

    pub fn hash64(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, PartialEq)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_str(s: &str) -> Self {
        let mut parts = s.trim().split(',');
        let x = parts.next().unwrap().trim().parse::<f64>().unwrap();
        let y = parts.next().unwrap().trim().parse::<f64>().unwrap();
        let z = parts.next().unwrap().trim().parse::<f64>().unwrap();
        Self::new(x, y, z)
    }

    pub fn on_top_of(&self, p2: &Point3D) -> bool {
        self.x == p2.x && self.y == p2.y && self.z == p2.z + 1.0
    }

    pub fn xy(&self) -> Point<f64> {
        Point::new(self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub p1: Point<f64>,
    pub p2: Point<f64>,
}

impl Line {
    pub fn new(p1: Point<f64>, p2: Point<f64>) -> Self {
        Self { p1, p2 }
    }

    // Returns Some(point) if the lines intersect, None otherwise
    pub fn intersects(&self, other: &Line) -> Option<Point<f64>> {
        let a1 = self.p2.y - self.p1.y;
        let b1 = self.p1.x - self.p2.x;
        let c1 = a1 * self.p1.x + b1 * self.p1.y;

        let a2 = other.p2.y - other.p1.y;
        let b2 = other.p1.x - other.p2.x;
        let c2 = a2 * other.p1.x + b2 * other.p1.y;

        let determinant = a1 * b2 - a2 * b1;

        if determinant == 0.0 {
            // Lines are parallel, no intersection
            None
        } else {
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            Some(Point { x, y })
        }
    }

    pub fn vector(&self) -> Point<f64> {
        Point::new(
            self.p2.x as f64 - self.p1.x as f64,
            self.p2.y as f64 - self.p1.y as f64,
        )
    }
}
