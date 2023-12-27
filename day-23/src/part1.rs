use std::collections::HashSet;

use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str_with_trim(input, '#');

    let start = Point::new(1, 0);
    let steps = 0;
    let visited = HashSet::new();
    let longest = hike(&map, start, steps, visited);

    Ok(longest.to_string())
}

fn hike(map: &CharMap, pos: Point<i64>, steps: u64, visited: HashSet<Point<i64>>) -> u64 {
    let finish = Point::new(map.width() as i64 - 2, map.height() as i64 - 1);
    if pos == finish {
        println!("Found finish at {} in {} steps", pos, steps);
        return steps;
    }

    let mut visited = visited.clone();
    visited.insert(pos);

    let mut candidates = vec![];

    for dir in Direction::each() {
        let next = pos + dir.delta();
        if visited.contains(&next) {
            continue;
        }

        let cell = *map.cell_for_point(&next);
        if cell == '#' {
            continue;
        }

        if cell == '<' || cell == '>' || cell == '^' || cell == 'v' {
            if cell != dir.to_char() {
                continue;
            }
        }

        let steps = hike(map, next, steps + 1, visited.clone());
        candidates.push(steps);
    }

    *candidates.iter().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.#####################
                     #.......#########...###
                     #######.#########.#.###
                     ###.....#.>.>.###.#.###
                     ###v#####.#v#.###.#.###
                     ###.>...#.#.#.....#...#
                     ###v###.#.#.#########.#
                     ###...#.#.#.......#...#
                     #####.#.#.#######.#.###
                     #.....#.#.#.......#...#
                     #.#####.#.#.#########v#
                     #.#...#...#...###...>.#
                     #.#.#v#######v###.###v#
                     #...#.>.#...>.>.#.###.#
                     #####v#.#.###v#.#.###.#
                     #.....#...#...#.#.#...#
                     #.#########.###.#.#.###
                     #...###...#...#...#.###
                     ###.###.#.###v#####v###
                     #...#...#.#.>.>.#.>.###
                     #.###.###.#.###.#.#v###
                     #.....###...###...#...#
                     #####################.#";
        assert_eq!("94", process(input)?);
        Ok(())
    }
}
