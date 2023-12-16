use std::collections::HashSet;

use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    point: Point<i64>,
    direction: Direction,
}

// Recursively simulate the beam
fn simulate(
    map: &CharMap,
    mut position: Position,
    seen: &mut HashSet<Position>,
    energized: &mut HashSet<Point<i64>>,
) {
    // If we've already seen this position from this direction, stop
    if seen.contains(&position) {
        return;
    }

    seen.insert(position);
    energized.insert(position.point);

    let next_point = position.point + position.direction.delta();
    let next_cell = map.cell_for_point(&next_point);

    // If the next cell is a wall, stop
    if *next_cell == '#' {
        return;
    }

    // Take a step forward
    position.point = next_point;

    // If the next cell is empty, continue
    if *next_cell == '.' {
        return simulate(map, position, seen, energized);
    }

    // If we hit a mirror, change direction
    let dir = position.direction;
    if *next_cell == '/' {
        position.direction = if dir == Direction::North || dir == Direction::South {
            dir.turn_right()
        } else {
            dir.turn_left()
        };
        return simulate(map, position, seen, energized);
    }

    // Another mirror
    if *next_cell == '\\' {
        position.direction = if dir == Direction::North || dir == Direction::South {
            dir.turn_left()
        } else {
            dir.turn_right()
        };
        return simulate(map, position, seen, energized);
    }

    // If the next cell is a splitter and we hit it from the pointy end, treat it as empty
    if (*next_cell == '|' && (dir == Direction::North || dir == Direction::South))
        || (*next_cell == '-' && (dir == Direction::East || dir == Direction::West))
    {
        return simulate(map, position, seen, energized);
    }

    // If the next cell is a splitter and we hit it from the flat end, split into two beams
    if (*next_cell == '|' && (dir == Direction::East || dir == Direction::West))
        || (*next_cell == '-' || (dir == Direction::North || dir == Direction::South))
    {
        let mut left = position.clone();
        left.direction = position.direction.turn_left();
        simulate(map, left, seen, energized);

        let mut right = position.clone();
        right.direction = position.direction.turn_right();
        simulate(map, right, seen, energized);
        return;
    }

    unreachable!("We should have handled all cases by now");
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str_with_trim(input, '#');
    map.print();

    let start = Position {
        point: Point::new(-1, 0),
        direction: Direction::East,
    };
    let mut seen = HashSet::new();
    let mut energized = HashSet::new();

    simulate(&map, start, &mut seen, &mut energized);

    Ok((energized.iter().count() - 1).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#".|...\....
                       |.-.\.....
                       .....|-...
                       ........|.
                       ..........
                       .........\
                       ..../.\\..
                       .-.-/..|..
                       .|....-|.\
                       ..//.|...."#;
        assert_eq!("46", process(input)?);
        Ok(())
    }
}

// Submissions:
// 6796 - too high
// 6795 - correct! (off by one error due to the fact we started outside the map)
