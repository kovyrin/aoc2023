use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pipe {
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

    fn to_char(&self) -> char {
        match self {
            Self::Start => 'S',
            Self::Horizontal => '-',
            Self::Vertical => '|',
            Self::LBend => 'L',
            Self::JBend => 'J',
            Self::SevenBend => '7',
            Self::FBend => 'F',
        }
    }
}

// Returns the directions that can be walked from a given pipe type
// Note: we always walk in a counter-clockwise direction
pub fn neighbours_for(c: Pipe) -> Vec<Direction> {
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

// Returns outside direction for the next piece of pipe
pub fn next_outside_direction(
    current_outside_dir: Direction,
    current_type: Pipe,
    next_type: Pipe,
) -> Direction {
    match next_type {
        Pipe::Horizontal | Pipe::Vertical => {
            current_outside_dir // No change in direction
        }
        Pipe::JBend => {
            if current_type == Pipe::Horizontal
                || current_type == Pipe::FBend
                || current_type == Pipe::LBend
            {
                current_outside_dir.turn_left()
            } else {
                current_outside_dir.turn_right()
            }
        }
        Pipe::LBend => {
            if current_type == Pipe::Horizontal
                || current_type == Pipe::SevenBend
                || current_type == Pipe::JBend
            {
                current_outside_dir.turn_right()
            } else {
                current_outside_dir.turn_left()
            }
        }
        Pipe::FBend => {
            if current_type == Pipe::Horizontal
                || current_type == Pipe::SevenBend
                || current_type == Pipe::JBend
            {
                current_outside_dir.turn_left()
            } else {
                current_outside_dir.turn_right()
            }
        }
        Pipe::SevenBend => {
            if current_type == Pipe::Horizontal
                || current_type == Pipe::FBend
                || current_type == Pipe::LBend
            {
                current_outside_dir.turn_right()
            } else {
                current_outside_dir.turn_left()
            }
        }
        Pipe::Start => {
            panic!("Start pipe should not be encountered in this context")
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str(input, '.');
    let mut map = map.with_padding(1, 1); // Add padding to ensure that we can walk around the edges
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

    loop {
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

    // Figure out the start cell's type based on its two neighbours in the loop
    let start_pipe = calculate_start_pipe(&path);
    println!("Start pipe: {:?}", start_pipe);
    map.set_cell_for_point(&start, start_pipe.to_char());

    // Update the path
    path[0] = (start, start_pipe);

    // Create a new map with the visited points and flood fill it
    // (the map and points are offset by 1 to allow the flood fill to work around all the edges)
    println!("Visited map:");
    let mut fill_map = CharMap::from_dimensions(map.width(), map.height(), '.');
    for point in visited.iter() {
        fill_map.set_cell_for_point(point, 'p');
    }
    fill_map.print();

    println!("Flood fill:");
    fill_map.flood_fill(Point::new(0, 0), 'O');
    fill_map.print();

    // Now we need to find one of the visited points that is adjacent to a filled cell
    let (start, outside_direction) = find_walkaround_start(&fill_map, &path);
    println!("Walkaround start: {:?} {:?}", start, outside_direction);
    fill_map.print_with_current(start, 'S');

    // Make the start step the first step in the path (since the walk is a loop, that is OK)
    let start_idx = path.iter().position(|(point, _)| *point == start).unwrap();
    path.rotate_left(start_idx);

    // Now we need to walk around the outside of the map until we find the start again
    // Each time we turn a corner we need to keep track of the direction that is outside
    // the perimeter we are walking.
    let mut outside_direction = outside_direction;
    for step_idx in 0..path.len() - 1 {
        let (current, current_type) = path[step_idx];
        let (next, next_type) = path[step_idx + 1];

        println!("Current: {:?} {:?}", current, current_type);
        println!("Outside direction: {:?}", outside_direction);
        fill_map.print_with_current(current, outside_direction.to_char());

        // Check if there is an internal section on the other side of the pipe
        if current_type == Pipe::Horizontal || current_type == Pipe::Vertical {
            let internal_direction = outside_direction.opposite();
            let internal_point = current.neighbour(internal_direction);
            let internal_cell = fill_map.cell_for_point(&internal_point);
            if *internal_cell == '.' {
                println!("Internal point: {:?}", internal_point);
                fill_map.flood_fill(internal_point, 'I');
                fill_map.print_with_current(internal_point, '*');
            }
        }
        // TODO: Figure out the next step's outside direction
        outside_direction = next_outside_direction(outside_direction, current_type, next_type)
    }
    let unfilled = fill_map.count('I');
    return Ok(unfilled.to_string());
}

fn calculate_start_pipe(path: &Vec<(Point, Pipe)>) -> Pipe {
    let (start, _) = path[0];
    let (pre_start, pre_start_type) = path.last().unwrap();
    let (post_start, post_start_type) = path[1];

    let in_dir = pre_start.direction_to(&start);
    let out_dir = start.direction_to(&post_start);

    println!("Pre start: {:?} -> {}", pre_start, pre_start_type.to_char());
    println!("Start: {:?} -> S", start);
    println!(
        "Post start: {:?} -> {}",
        post_start,
        post_start_type.to_char()
    );

    println!("In direction: {:?}", in_dir);
    println!("Out direction: {:?}", out_dir);

    match (pre_start_type, post_start_type) {
        (Pipe::Horizontal, Pipe::Horizontal) => Pipe::Horizontal,
        (Pipe::Vertical, Pipe::Vertical) => Pipe::Vertical,

        (Pipe::Vertical, Pipe::Horizontal) => {
            if in_dir == Direction::North {
                if out_dir == Direction::East {
                    Pipe::LBend
                } else {
                    Pipe::JBend
                }
            } else {
                if out_dir == Direction::East {
                    Pipe::FBend
                } else {
                    Pipe::SevenBend
                }
            }
        }

        (Pipe::Horizontal, Pipe::Vertical) => {
            if in_dir == Direction::East {
                if out_dir == Direction::North {
                    Pipe::JBend
                } else {
                    Pipe::SevenBend
                }
            } else {
                if out_dir == Direction::North {
                    Pipe::LBend
                } else {
                    Pipe::FBend
                }
            }
        }

        (Pipe::Horizontal, Pipe::JBend) => todo!(),
        (Pipe::Vertical, Pipe::LBend) => todo!(),
        (Pipe::Horizontal, Pipe::SevenBend) => todo!(),
        (Pipe::Vertical, Pipe::FBend) => todo!(),
        (Pipe::Horizontal, Pipe::FBend) => todo!(),
        (Pipe::Vertical, Pipe::SevenBend) => todo!(),
        _ => panic!("Could not determine start type"),
    }
}

fn find_walkaround_start(fill_map: &CharMap, visited: &Vec<(Point, Pipe)>) -> (Point, Direction) {
    for (point, pipe) in visited.iter() {
        if *pipe != Pipe::Horizontal {
            continue;
        }

        let direction = Direction::North;
        let neighbour = point.neighbour(direction);
        let cell = fill_map.cell_for_point(&neighbour);
        if *cell == 'O' {
            return (point.clone(), direction);
        }
    }
    panic!("Could not find walkaround start");
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
                     .||OOOO||.
                     .||OOOO||.
                     .|L-7F-J|.
                     .|II||II|.
                     .L--JL--J.
                     ..........";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
