use fxhash::{self, hash64};
use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

use crate::custom_error::AocError;

struct Cache {
    results: HashMap<u64, u64>,
}

impl Cache {
    fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    fn hash(spring_map: &str, bad_records: &[usize]) -> u64 {
        hash64(spring_map).wrapping_add(hash64(bad_records))
    }

    fn insert(&mut self, spring_map: &str, bad_records: &[usize], result: u64) -> u64 {
        self.results
            .insert(Self::hash(spring_map, bad_records), result);
        result
    }

    fn get(&self, spring_map: &str, bad_records: &[usize]) -> Option<&u64> {
        self.results.get(&Self::hash(spring_map, bad_records))
    }
}

fn solve(spring_map: &str, bad_records: &[usize], cache: &mut Cache) -> u64 {
    if let Some(result) = cache.get(spring_map, bad_records) {
        return *result;
    }

    // We ran out of records, so we are done. Just need to check if all remaining items are empty.
    if bad_records.is_empty() {
        let all_spaces = spring_map.chars().all(|c| c == '?' || c == '.');
        return cache.insert(spring_map, bad_records, if all_spaces { 1 } else { 0 });
    }

    // Skip to the next non-empty character and abort of there are none (since we still have records)
    let start_pos = spring_map.chars().position(|c| c != '.');
    if start_pos.is_none() {
        return cache.insert(spring_map, bad_records, 0);
    }

    // Shift the start of the spring map to the first non-empty symbol
    let start_pos = start_pos.unwrap();
    let spring_map = &spring_map[start_pos..];

    // Check if we have enough space left to solve the problem
    let min_solution_len = bad_records.iter().sum::<usize>() + bad_records.len() - 1;
    if spring_map.len() < min_solution_len {
        return cache.insert(spring_map, bad_records, 0);
    }

    // Check if we can fit the first record into the beginning of the map
    // If we can, continue trying to find a solution down that path
    let record = bad_records[0];
    let mut skip_chars = record;
    let mut record_fits = can_fit_record(&spring_map, record);

    // If we want to continue, we need to make sure it is possible to get a space in
    if record_fits && bad_records.len() > 1 {
        let next_char = spring_map[skip_chars..].chars().next().unwrap();
        if next_char != '.' && next_char != '?' {
            record_fits = false;
        }
        skip_chars += 1; // account for the space
    }

    let mut solution = 0;

    // If the first record fits, go deeper
    if record_fits {
        solution = solve(&spring_map[skip_chars..], &bad_records[1..], cache)
    };

    // Only attempt solution at the next position if it starts with an empty character
    if !spring_map.is_empty() {
        let next_char = spring_map.chars().next().unwrap();
        if next_char == '.' || next_char == '?' {
            solution = solution + solve(&spring_map[1..], bad_records, cache)
        }
    }

    cache.insert(spring_map, bad_records, solution)
}

// Receives a slice of a string from the spring map and checks if the slice can
// fit an uninterrupted group of broken springs
fn can_fit_record(spring_map: &str, record_len: usize) -> bool {
    spring_map[0..record_len]
        .chars()
        .all(|c| c == '#' || c == '?')
}

fn real_count_arrangements(records: &str, bad_records: &Vec<usize>) -> u64 {
    let mut cache = Cache::new();
    return solve(records, bad_records, &mut cache);
}

fn count_arrangements(records: &str, og_bad_records: &Vec<usize>) -> u64 {
    let records = format!(
        "{}?{}?{}?{}?{}",
        records, records, records, records, records
    );

    let mut bad_records = og_bad_records.clone();
    bad_records.extend(og_bad_records);
    bad_records.extend(og_bad_records);
    bad_records.extend(og_bad_records);
    bad_records.extend(og_bad_records);

    real_count_arrangements(&records, &bad_records)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut total = 0;
    for line in input.lines() {
        let line = line.trim();
        println!("Solving {:?}...", line);
        let chunks = line.split_whitespace().collect_vec();
        let records = chunks[0];
        let bad_groups = chunks[1].split(',').map(|s| s.parse().unwrap()).collect();
        let start = Instant::now();
        total += count_arrangements(records, &bad_groups);
        let duration = start.elapsed();
        println!("- Finished in: {:?}", duration);
        println!("- Current total: {}", total);
        println!();
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut cache = Cache::new();
        assert_eq!(1, solve("???.###", &vec![1, 1, 3], &mut cache));
        assert_eq!(4, solve(".??..??...?##.", &vec![1, 1, 3], &mut cache));
        assert_eq!(1, solve("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6], &mut cache));
        assert_eq!(1, solve("????.#...#...", &vec![4, 1, 1], &mut cache));
        assert_eq!(4, solve("????.######..#####.", &vec![1, 6, 5], &mut cache));
        assert_eq!(10, solve("?###????????", &vec![3, 2, 1], &mut cache));
    }

    #[test]
    fn test_count_arrangements() {
        assert_eq!(1, count_arrangements("???.###", &vec![1, 1, 3]));
        assert_eq!(16384, count_arrangements(".??..??...?##.", &vec![1, 1, 3]));
        assert_eq!(1, count_arrangements("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]));
        assert_eq!(16, count_arrangements("????.#...#...", &vec![4, 1, 1]));
        assert_eq!(
            2500,
            count_arrangements("????.######..#####.", &vec![1, 6, 5])
        );
        assert_eq!(506250, count_arrangements("?###????????", &vec![3, 2, 1]));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
                     .??..??...?##. 1,1,3
                     ?#?#?#?#?#?#?#? 1,3,1,6
                     ????.#...#... 4,1,1
                     ????.######..#####. 1,6,5
                     ?###???????? 3,2,1";
        assert_eq!("525152", process(input)?);
        Ok(())
    }
}
