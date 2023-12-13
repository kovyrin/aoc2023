use std::time::Instant;

use itertools::Itertools;

use crate::custom_error::AocError;

fn solve(spring_map: &str, bad_records: &[usize], debug: bool) -> u64 {
    if debug {
        println!(
            "\nSolving {:?} (len={}) vs {:?}",
            spring_map,
            spring_map.len(),
            bad_records,
        )
    }

    if bad_records.is_empty() {
        if debug {
            println!("* bad records are empty")
        }
        if spring_map.chars().all(|c| c == '?' || c == '.') {
            if debug {
                println!("+ all remaining records are empty, found a solution")
            }
            return 1;
        } else {
            if debug {
                println!("- ran out of bad records, but there are still #s left")
            }
            return 0;
        }
    }

    let start_pos = spring_map.chars().position(|c| c != '.');
    if start_pos.is_none() {
        if debug {
            println!("- all dots at the end, but we still have records to fit")
        }
        return 0;
    }

    // Shift the start of the spring map to the first non-empty symbol
    let start_pos = start_pos.unwrap();
    let spring_map = &spring_map[start_pos..];
    if debug {
        println!("* starting at {}", start_pos)
    }

    // Check if we have enough space left to solve the problem
    let min_solution_len = bad_records.iter().sum::<usize>() + bad_records.len() - 1;
    if debug {
        println!(
            "* Minimum space required to fit the records: {}",
            min_solution_len
        )
    }
    if spring_map.len() < min_solution_len {
        if debug {
            println!("- Remaining space is not enough to fit the records");
        }
        return 0;
    }

    // Check if we can fit the first record into the beginning of the map
    // If we can, continue trying to find a solution down that path
    let record = bad_records[0];
    let mut skip_chars = record;
    let mut record_fits = can_fit_record(&spring_map, record, debug);

    // If we want to continue, we need to make sure it is possible to get a space in
    if record_fits && bad_records.len() > 1 {
        let next_char = spring_map[skip_chars..].chars().next().unwrap();
        if next_char != '.' && next_char != '?' {
            if debug {
                println!("- cannot place a space after {}", record);
            }
            record_fits = false;
        }
        skip_chars += 1; // account for the space
    }

    let mut solution = if record_fits {
        if debug {
            println!("* We can fit {} in here, let's continue...", record);
        }
        solve(&spring_map[skip_chars..], &bad_records[1..], debug)
    } else {
        if debug {
            println!("- Nope, {} does not fit here", record);
        }
        0
    };

    if !spring_map.is_empty() {
        let next_char = spring_map.chars().next().unwrap();
        if next_char == '.' || next_char == '?' {
            solution = solution + solve(&spring_map[1..], bad_records, debug)
        }
    }

    solution
}

// Receives a slice of a string from the spring map and checks if the slice can
// fit an uninterrupted group of broken springs
fn can_fit_record(spring_map: &str, record_len: usize, debug: bool) -> bool {
    if debug {
        println!(
            "* Checking if {:?} can fit {} broken springs",
            spring_map, record_len
        );
    }

    spring_map[0..record_len]
        .chars()
        .all(|c| c == '#' || c == '?')
}

fn real_count_arrangements(records: &str, bad_records: &Vec<usize>) -> u64 {
    return solve(records, bad_records, false);
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
        assert_eq!(1, solve("???.###", &vec![1, 1, 3], true));
        assert_eq!(4, solve(".??..??...?##.", &vec![1, 1, 3], true));
        assert_eq!(1, solve("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6], true));
        assert_eq!(1, solve("????.#...#...", &vec![4, 1, 1], true));
        assert_eq!(4, solve("????.######..#####.", &vec![1, 6, 5], true));
        assert_eq!(10, solve("?###????????", &vec![3, 2, 1], true));
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
