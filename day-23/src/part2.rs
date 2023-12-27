use std::collections::{HashMap, HashSet};

use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = input
        .replace("<", ".")
        .replace(">", ".")
        .replace("^", ".")
        .replace("v", ".");

    let map = CharMap::from_str_with_trim(&input, '#');

    let start = Point::new(1, 0);
    let visited = HashSet::new();

    let mut best_steps = BestVisits::new();
    best_steps.insert((start, Direction::South), 0);

    let longest = hike(&map, start, Direction::South, visited, &mut best_steps);

    Ok(longest.to_string())
}

type Visit = (Point<i64>, Direction);
type BestVisits = HashMap<Visit, u64>;

fn hike(
    map: &CharMap,
    pos: Point<i64>,
    dir: Direction,
    visited: HashSet<Point<i64>>,
    best: &mut BestVisits, // Best known steps to get to a point (longest)
) -> u64 {
    let finish = Point::new(map.width() as i64 - 2, map.height() as i64 - 1);

    let visit = (pos, dir);
    let steps = visited.iter().count() as u64;

    if let Some(best_steps) = best.get(&visit) {
        if steps < *best_steps {
            println!(
                "Skipping visit {:?} after {} steps because previously reached it in {}",
                visit, steps, best_steps
            );
            return 0;
        }
    }
    best.insert(visit, steps);

    let mut visited = visited.clone();
    visited.insert(pos);

    if pos == finish {
        println!(
            "Found finish at {} in {} steps. Best = {}",
            pos,
            steps,
            best.get(&visit).unwrap()
        );
        // print_visits(map, &pos, &visited, best);
        return steps;
    }

    // Take next steps
    let mut candidates = vec![];
    for dir in Direction::each() {
        let next = pos + dir.delta();

        if visited.contains(&next) {
            continue;
        }

        if *map.cell_for_point(&next) == '#' {
            continue;
        }

        let res = hike(map, next, dir, visited.clone(), best);
        candidates.push(res);
    }

    candidates.iter().max().copied().unwrap_or(0)
}

fn print_visits(
    map: &CharMap,
    pos: &Point<i64>,
    visited: &HashSet<Point<i64>>,
    best: &mut HashMap<(Point<i64>, Direction), u64>,
) {
    println!("---------------------------------------------------------------------------------------------------------------------------");
    // println!("Best visits: {:#?}", best);

    // let debug_west = (Point::new(6, 13), Direction::West);
    // assert!(best.contains_key(&debug_west));

    // let debug_east = (Point::new(6, 13), Direction::East);
    // assert!(best.contains_key(&debug_east));

    // println!("---------------------------------------------------------------------------------------------------------------------------");

    // Create a map where all visited points are marked with a direction char from the best visits map
    let mut visited_map = map.clone();
    // for (pos, dir) in best.keys() {
    //     visited_map.set_cell_for_point(&pos, dir.to_char());
    // }

    for pos in visited {
        visited_map.set_cell_for_point(&pos, 'O');
    }

    visited_map.print_with_current(pos, 'X');
    println!("---------------------------------------------------------------------------------------------------------------------------");
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
        assert_eq!("154", process(input)?);
        Ok(())
    }
}

// Submissions:
// 5130 - ?
// 5466 - too low
// 6055 - too low
// 6079 - too low
// 6122 - incorrect
// 6410 - correct!
