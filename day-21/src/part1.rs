use std::collections::{HashSet, VecDeque};

use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};

#[tracing::instrument]
pub fn process(input: &str, max_steps: u64) -> miette::Result<String, AocError> {
    let mut map = CharMap::from_str_with_trim(input, '#');

    let start = map.find('S').unwrap();
    map.set_cell_for_point(&start, '.');

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut seen: HashSet<(Point<i64>, u64)> = HashSet::new();
    let mut reached_at_limit: HashSet<Point<i64>> = HashSet::new();

    while let Some((pos, steps)) = queue.pop_front() {
        if steps > max_steps || map.out_of_bounds(&pos) || seen.contains(&(pos, steps)) {
            continue;
        }

        let cell = map.cell_for_point(&pos);
        if *cell == '#' {
            continue;
        }
        if *cell == '.' {
            seen.insert((pos, steps));
            if steps == max_steps {
                reached_at_limit.insert(pos);
            }
        }

        for dir in Direction::each() {
            queue.push_back((pos + dir.delta(), steps + 1));
        }
    }

    Ok(reached_at_limit.iter().count().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...........
                     .....###.#.
                     .###.##..#.
                     ..#.#...#..
                     ....#.#....
                     .##..S####.
                     .##..#...#.
                     .......##..
                     .##.#.####.
                     .##..##.##.
                     ...........";
        assert_eq!("16", process(input, 6)?);
        Ok(())
    }
}
