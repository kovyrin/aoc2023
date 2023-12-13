use core::panic;
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

    let result = maps
        .iter_mut()
        .map(|m| process_map_with_smudge(m))
        .sum::<usize>();
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

fn process_map(map: &CharMap, ignore_result: Option<usize>) -> usize {
    let mut result = process_map_internal(map);
    let mut result_multiplier = 1;

    if let Some(ignore_result) = ignore_result {
        result.remove(&ignore_result);
    }

    if result.is_empty() {
        let map = map.transpose();
        result_multiplier = 100;
        result = process_map_internal(&map);
    }

    if let Some(ignore_result) = ignore_result {
        result.remove(&(ignore_result / result_multiplier));
    }

    *result.iter().next().unwrap_or(&0) * result_multiplier
}

fn process_map_with_smudge(map: &mut CharMap) -> usize {
    let original_score = process_map(map, None);

    // now try to change every single cell to a mirror to a different one
    // and see if we can find a different (non-zero) solution
    for y in 0..map.height() as i64 {
        for x in 0..map.width() as i64 {
            let old_char = *map.cell(x, y);
            let new_char = if old_char == '#' { '.' } else { '#' };
            map.set_cell(x as usize, y as usize, new_char);
            let new_score = process_map(map, Some(original_score));
            if new_score != 0 {
                return new_score;
            }
            map.set_cell(x as usize, y as usize, old_char);
        }
    }

    map.print();
    panic!("No smudge found");
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
    fn test_process_map_fixed_smudge() {
        let input = "..##..##.
                     ..#.##.#.
                     ##......#
                     ##......#
                     ..#.##.#.
                     ..##..##.
                     #.#.##.#.";
        let map = CharMap::from_iter(input.lines().map(|l| l.trim()), ' ');
        assert_eq!(300, process_map(&map, Some(5)))
    }

    #[test]
    fn test_process_map_with_smudge() {
        let input = "#.##..##.
                     ..#.##.#.
                     ##......#
                     ##......#
                     ..#.##.#.
                     ..##..##.
                     #.#.##.#.";
        let mut map = CharMap::from_iter(input.lines().map(|l| l.trim()), ' ');
        assert_eq!(300, process_map_with_smudge(&mut map))
    }

    #[test]
    fn test_process_map_with_fixed_smudge2() {
        let input = "#...##..#
                     #...##..#
                     ..##..###
                     #####.##.
                     #####.##.
                     ..##..###
                     #....#..#";
        let map = CharMap::from_iter(input.lines().map(|l| l.trim()), ' ');
        assert_eq!(100, process_map(&map, Some(400)))
    }

    #[test]
    fn test_process_map_with_smudge2() {
        let input = "#...##..#
                     #....#..#
                     ..##..###
                     #####.##.
                     #####.##.
                     ..##..###
                     #....#..#";
        let mut map = CharMap::from_iter(input.lines().map(|l| l.trim()), ' ');
        assert_eq!(100, process_map_with_smudge(&mut map))
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
        assert_eq!("400", process(input)?);
        Ok(())
    }
}
