use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pipe {
    Horizontal, // -
    Vertical,   // |
    LBend,      // L
    JBend,      // J
    FBend,      // F
    SevenBend,  // 7
    Start,      // S
}

impl Pipe {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'S' => Some(Self::Start),
            '-' => Some(Self::Horizontal),
            '|' => Some(Self::Vertical),
            'L' => Some(Self::LBend),
            'J' => Some(Self::JBend),
            '7' => Some(Self::SevenBend),
            'F' => Some(Self::FBend),
            _ => None,
        }
    }
}

// Returns the directions that can be walked from a given pipe type
// Note: we always walk in a counter-clockwise direction
fn neighbours_for(c: Pipe) -> Vec<Direction> {
    match c {
        Pipe::Horizontal => vec![Direction::West, Direction::East],
        Pipe::Vertical => vec![Direction::North, Direction::South],
        Pipe::LBend => vec![Direction::East, Direction::North],
        Pipe::JBend => vec![Direction::West, Direction::North],
        Pipe::FBend => vec![Direction::South, Direction::East],
        Pipe::SevenBend => vec![Direction::South, Direction::West],
        Pipe::Start => vec![
            Direction::South,
            Direction::East,
            Direction::North,
            Direction::West,
        ],
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str(input, '.');
    let map = map.with_padding(1, 1); // Add padding to ensure that we can walk around the edges
    let start = map.find('S').unwrap();

    // A map of a direction (from a current point) to a possible pipe types that can
    // be connected from that direction.
    let mut pipe_connections = HashMap::new();
    pipe_connections.insert(
        Direction::North,
        vec![Pipe::Vertical, Pipe::FBend, Pipe::SevenBend],
    );
    pipe_connections.insert(
        Direction::South,
        vec![Pipe::Vertical, Pipe::LBend, Pipe::JBend],
    );
    pipe_connections.insert(
        Direction::West,
        vec![Pipe::Horizontal, Pipe::LBend, Pipe::FBend],
    );
    pipe_connections.insert(
        Direction::East,
        vec![Pipe::Horizontal, Pipe::JBend, Pipe::SevenBend],
    );

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut current = start;
    println!("Start: {:?}", start);
    let mut path = Vec::new();

    map.print();

    loop {
        // map.print_with_current(current, '*');
        let current_cell = map.cell_for_point(&current);
        let current_pipe = Pipe::from_char(*current_cell).unwrap();
        path.push((current, current_pipe));

        let neighbour_directions = neighbours_for(current_pipe);
        let mut possible_directions = Vec::new();
        for direction in neighbour_directions.iter() {
            let next = current.neighbour(*direction);
            if visited.contains(&next) {
                continue;
            }

            let allowed_connections = pipe_connections.get(&direction).unwrap();
            let next_cell = map.cell_for_point(&next);
            if let Some(next_pipe) = Pipe::from_char(*next_cell) {
                if allowed_connections.contains(&next_pipe) {
                    possible_directions.push(*direction);
                }
            }
        }

        if possible_directions.is_empty() {
            break;
        }

        // pick first direction and follow it
        current = current.neighbour(possible_directions[0]);
        visited.insert(current);
    }

    // Create a new map with the visited points and flood fill it
    // (the map and points are offset by 1 to allow the flood fill to work around all the edges)
    println!("Visited map:");
    let mut fill_map = CharMap::from_dimensions(map.width(), map.height(), '.');
    for point in visited.iter() {
        fill_map.set_cell_for_point(&point, *map.cell_for_point(&point));
    }
    fill_map.print();
    fill_map.flood_fill(Point::new(0, 0), 'O');

    for y in 0..fill_map.height() - 1 {
        let mut outside = true;
        for x in 0..fill_map.width() - 1 {
            let cell = fill_map.cell(x as i32, y as i32);
            if *cell == '.' {
                if outside {
                    fill_map.set_cell(x, y, 'O');
                } else {
                    fill_map.set_cell(x, y, 'I');
                }
            } else if *cell == '|' || *cell == 'L' || *cell == 'J' {
                outside = !outside;
            }
        }
    }

    fill_map.print();

    let internal = fill_map.count('I');
    return Ok(internal.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".....
                     .S-7.
                     .|.|.
                     .L-J.
                     .....";
        assert_eq!("1", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_messy() -> miette::Result<()> {
        let input = "...........
                     .S-------7.
                     .|F-----7|.
                     .||.....||.
                     .||.....||.
                     .|L-7.F-J|.
                     .|..|.|..|.
                     .L--J.L--J.
                     ...........";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_impassable() -> miette::Result<()> {
        let input = "..........
                     .S------7.
                     .|F----7|.
                     .||....||.
                     .||....||.
                     .|L-7F-J|.
                     .|..||..|.
                     .L--JL--J.
                     ..........";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_random_bits() -> miette::Result<()> {
        let input = ".F----7F7F7F7F-7....
                     .|F--7||||||||FJ....
                     .||.FJ||||||||L7....
                     FJL7L7LJLJ||LJ.L-7..
                     L--J.L7...LJS7F-7L7.
                     ....F-J..F7FJ|L7L7L7
                     ....L7.F7||L7|.L7L7|
                     .....|FJLJ|FJ|F7|.LJ
                     ....FJL-7.||.||||...
                     ....L---J.LJ.LJLJ...";
        assert_eq!("8", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_junk() -> miette::Result<()> {
        let input = "FF7FSF7F7F7F7F7F---7
                     L|LJ||||||||||||F--J
                     FL-7LJLJ||||||LJL-77
                     F--JF--7||LJLJ7F7FJ-
                     L---JF-JLJ.||-FJLJJ7
                     |F|F-JF---7F7-L7L|7|
                     |FFJF7L7F-JF7|JL---7
                     7-L-JL7||F7|L7F-7F7|
                     L.L7LFJ|||||FJL7||LJ
                     L7JLJL-JLJLJL--JLJ.L";
        assert_eq!("10", process(input)?);
        Ok(())
    }
}

// Submissions:
// - 393 - too high
// - 273 - too high
// - 265 - correct
