use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    custom_error::AocError,
    utils::{CharMap, CharRow},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut maps = vec![];
    let mut map_lines = vec![];
    for line in input.lines() {
        if line.is_empty() {
            let map = CharMap::from_iter(map_lines.iter(), ' ');
            maps.push(map);
            map_lines = vec![]
        } else {
            map_lines.push(line)
        }
    }

    if !map_lines.is_empty() {
        let map = CharMap::from_iter(map_lines.iter(), ' ');
        maps.push(map);
    }

    let result = maps.iter().map(|m| process_map(&m)).sum::<usize>();
    Ok(result.to_string())
}

fn process_map_internal(map: &CharMap) -> HashSet<usize> {
    let mirrors_per_row = map
        .lines()
        .map(|l| potential_mirror_positions(l))
        .collect_vec();

    let mut iter = mirrors_per_row.iter();
    let first_set = iter.next().unwrap().clone();
    iter.fold(first_set, |acc, set| {
        acc.intersection(set).cloned().collect()
    })
}

fn process_map(map: &CharMap) -> usize {
    let mut result = process_map_internal(map);
    let mut result_multiplier = 1;

    if result.is_empty() {
        let map = map.transpose();
        result_multiplier = 100;
        result = process_map_internal(&map);
        if result.is_empty() {
            map.print();
            panic!("No mirrors found");
        }
    }

    *result.iter().next().unwrap() * result_multiplier
}

fn potential_mirror_positions(line: &CharRow) -> HashSet<usize> {
    let mut result = HashSet::new();
    for pos in 1..line.len() {
        if is_mirror_at(pos, line) {
            result.insert(pos);
        }
    }
    result
}

fn is_mirror_at(mirror_pos: usize, line: &CharRow) -> bool {
    let mirror_pos = mirror_pos as i64;
    let row_len = line.len();
    let range_size = (row_len as f64 / 2.0).ceil() as i64;
    let left_range_start = mirror_pos - range_size as i64;
    let left_range = left_range_start..mirror_pos;

    for left_pos in left_range {
        let right_pos = mirror_pos + (range_size - (left_pos - left_range_start)) - 1;
        let left_char = *line.cell(left_pos);
        let right_char = *line.cell(right_pos);
        if left_char != right_char && left_char != ' ' && right_char != ' ' {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mirror_at() {
        assert_eq!(true, is_mirror_at(1, &CharRow::from_str("##", ' ')));
        assert_eq!(true, is_mirror_at(2, &CharRow::from_str(".##.", ' ')));
        assert_eq!(true, is_mirror_at(3, &CharRow::from_str("..##.", ' ')));
        assert_eq!(true, is_mirror_at(5, &CharRow::from_str("#.##..##", ' ')));
    }

    #[test]
    fn test_process_map_horizontal() {
        let input = "#...##..#
                     #....#..#
                     ..##..###
                     #####.##.
                     #####.##.
                     ..##..###
                     #....#..#";
        let map = CharMap::from_iter(input.lines().map(|l| l.trim()), ' ');
        assert_eq!(400, process_map(&map))
    }

    #[test]
    fn test_process_map_vertical() {
        let input = "#.##..##.
                     ..#.##.#.
                     ##......#
                     ##......#
                     ..#.##.#.
                     ..##..##.
                     #.#.##.#.";
        let map = CharMap::from_iter(input.lines().map(|l| l.trim()), ' ');
        assert_eq!(5, process_map(&map))
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
                     ..#.##.#.
                     ##......#
                     ##......#
                     ..#.##.#.
                     ..##..##.
                     #.#.##.#.

                     #...##..#
                     #....#..#
                     ..##..###
                     #####.##.
                     #####.##.
                     ..##..###
                     #....#..#";
        assert_eq!("405", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_map() {
        let input = ".#.#...
                     ###.#..
                     ###.##.
                     .#.#...
                     #.#.###
                     .####..
                     .....##
                     #.#.#..
                     .#.###.
                     ###...#
                     ###.##.
                     .####.#
                     ###.###
                     ####..#
                     ####..#";
        let map = CharMap::from_iter(input.lines().map(|l| l.trim()), ' ');
        assert_eq!(1400, process_map(&map))
    }
}

// Submissions:
// 11344 - too low
// 31956 - correct
