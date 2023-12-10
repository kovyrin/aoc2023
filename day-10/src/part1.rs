use std::collections::{HashMap, HashSet};

use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction},
};

/*
The pipes are arranged in a two-dimensional grid of tiles:

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pipe {
    Horizontal, // -
    Vertical,   // |
    NEBend,     // L
    NWBend,     // J
    SEBend,     // F
    SWBend,     // 7
    Start,      // S
}

impl Pipe {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'S' => Some(Self::Start),
            '-' => Some(Self::Horizontal),
            '|' => Some(Self::Vertical),
            'L' => Some(Self::NEBend),
            'J' => Some(Self::NWBend),
            '7' => Some(Self::SWBend),
            'F' => Some(Self::SEBend),
            _ => None,
        }
    }
}

pub fn neighbours_for(c: Pipe) -> Vec<Direction> {
    match c {
        Pipe::Horizontal => vec![Direction::West, Direction::East],
        Pipe::Vertical => vec![Direction::North, Direction::South],
        Pipe::NEBend => vec![Direction::North, Direction::East],
        Pipe::NWBend => vec![Direction::North, Direction::West],
        Pipe::SEBend => vec![Direction::South, Direction::East],
        Pipe::SWBend => vec![Direction::South, Direction::West],
        Pipe::Start => vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str(input, '.');

    let start = map.find('S').unwrap();

    // A map of a direction (from a current point) to a possible pipe types that can
    // be connected from that direction.
    let mut pipe_connections = HashMap::new();
    pipe_connections.insert(
        Direction::North,
        vec![Pipe::Vertical, Pipe::SEBend, Pipe::SWBend],
    );
    pipe_connections.insert(
        Direction::South,
        vec![Pipe::Vertical, Pipe::NEBend, Pipe::NWBend],
    );
    pipe_connections.insert(
        Direction::West,
        vec![Pipe::Horizontal, Pipe::NEBend, Pipe::SEBend],
    );
    pipe_connections.insert(
        Direction::East,
        vec![Pipe::Horizontal, Pipe::NWBend, Pipe::SWBend],
    );

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut current = start;
    loop {
        let cell = map.cell_for_point(&current);
        println!("Current: {:?} => {:?}", current, cell);

        let cell_type = Pipe::from_char(*cell).unwrap();
        let neighbours = neighbours_for(cell_type);

        let mut possible_directions = Vec::new();
        for direction in neighbours.iter() {
            let point = current.neighbour(*direction);
            if visited.contains(&point) {
                continue;
            }

            let allowed_connections = pipe_connections.get(&direction).unwrap();
            let cell = map.cell_for_point(&point);
            let pipe_type = Pipe::from_char(*cell);
            if let Some(pipe_type) = pipe_type {
                if allowed_connections.contains(&pipe_type) {
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

    println!("Visited {} cells", visited.len());

    return Ok((visited.len() / 2).to_string());
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
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_messy() -> miette::Result<()> {
        let input = "-L|F7
                     7S-7|
                     L|7||
                     -L-J|
                     L|-JF";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
