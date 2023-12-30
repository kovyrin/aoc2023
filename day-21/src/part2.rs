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

    #[test]
    fn test_experiments() {
        let input = include_str!("../input2.txt");

        let steps = 26501365;
        let map_width = input.lines().next().unwrap().len() as u64;
        let start = steps % (2 * map_width);

        for i in 0..10 {
            let step = start + i * 2 * map_width;
            let result = process(input, step as u64).unwrap().parse::<i64>().unwrap();
            println!("Step {} -> {}", step, result,);
        }
    }
}

/*
Notes:

steps = 26501365
map_width = 131

If the answer is a function f(x) and after we exit the first square map, it repeats every 2*map_width steps, then we can
define a new function g(x) = f(map_width/2 + 2*map_width).

Now, we can run our flood fill algorithm for the first few values of x (1,2,3,4,5,6),
mapping to step counts of 327, 589, 851 and 1113, 1375, 1637 we get a set of g(x) values:

g(1) = f(327) = 94475
g(2) = f(589) = 305871
g(3) = f(851) = 637987
g(4) = f(1113) = 1090823
g(5) = f(1375) = 1664379
g(6) = f(1637) = 2358655

If we put those into a quadratic regression we get a set of parameters for a quadratic function:

y = ax^2 + bx + c

a = 60360
b = 30316
c = 3799

Putting it all together, the response to our problem is:

g(steps / (2*map_width)) = g(101150) = round(60360 * 101150^2 + 30316 * 101150 + 3799) = 617565692567200

*/
