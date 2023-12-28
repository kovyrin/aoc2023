use std::collections::{HashSet, VecDeque};

use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};

fn to_map_point(pos: Point<i64>, map: &CharMap) -> Point<i64> {
    let real_x = pos.x.rem_euclid(map.width() as i64);
    let real_y = pos.y.rem_euclid(map.height() as i64);
    Point::new(real_x, real_y)
}

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
        if steps > max_steps || seen.contains(&(pos, steps)) {
            continue;
        }

        // Convert to real position by wrapping around the map.
        let real_pos = to_map_point(pos, &map);
        let cell = map.cell_for_point(&real_pos);
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

    #[test]
    fn test_process10() {
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
        assert_eq!("50", process(input, 10).unwrap());
    }

    #[test]
    fn test_process50() {
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
        assert_eq!("1594", process(input, 50).unwrap());
    }

    #[test]
    fn test_process100() {
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
        assert_eq!("6536", process(input, 100).unwrap());
    }

    #[test]
    fn test_process500() {
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
        assert_eq!("167004", process(input, 500).unwrap());
    }
}

// Submissions:
// 616953970868785 - too low
//
// 617565692567194
//
// 617565692567204 - incorrect
// 617565692567205 - incorrect
//
// 617820084546712 - incorrect
// 617820084546713 - too high
// 641973551585794 - incorrect
// 26741036441321596 - too high
