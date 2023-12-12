use itertools::Itertools;

use crate::custom_error::AocError;

// Implements a bitmask generator that is given a mask vector and returns
// all possible masks where items from the initial mask are replaced with
// either true or false.
// So, given a mask of [true, false, true], the generator will return
// - [true, false, true]
// - [true, false, false]
// - [false, false, true],
// - [false, false, false]
//
// The generator will return None when all possible masks have been returned.
//
// The implementation uses an internal counter where the first len(mask fields that are true)
// bits are used to determine which (true) bits in the original mask should be set to
// which value. So, for the example above, the counter will be incremented from
// 0 to 3 (since there are only 2 bits set to true in the mask).
//
#[derive(Debug)]
struct MaskGenerator {
    mask: u128,
    mask_len: usize,
    bit_counter: usize,
    max_counter: usize,
}

impl MaskGenerator {
    fn new(mask: &str) -> Self {
        let mask = mask
            .chars()
            .rev()
            .map(|c| if c == '?' { '1' } else { '0' })
            .collect::<String>();
        let mask_len = mask.len();
        let bitmask = u128::from_str_radix(&mask, 2).unwrap();
        let enabled_bits = mask.chars().filter(|b| *b == '1').count();

        Self {
            mask: bitmask,
            mask_len,
            bit_counter: 0,
            max_counter: 2usize.pow(enabled_bits as u32),
        }
    }

    fn next(&mut self) -> Option<Vec<bool>> {
        if self.bit_counter == self.max_counter {
            return None;
        }

        let mut result = vec![false; self.mask_len];
        let mut bit_counter = self.bit_counter;

        for i in 0..self.mask_len {
            if self.mask & (1 << i) != 0 {
                result[i] = bit_counter % 2 == 1;
                bit_counter /= 2;
            }
        }
        self.bit_counter += 1;
        Some(result)
    }
}

fn count_arrangements(records: &str, bad_records: &Vec<u8>) -> u64 {
    let record_chars = records.chars().collect::<Vec<_>>();

    let mut count = 0;
    let mut mask_generator = MaskGenerator::new(records);
    while let Some(mask) = mask_generator.next() {
        let mut result = record_chars.clone();
        for i in 0..mask.len() {
            result[i] = match mask[i] {
                true => '#',
                false => {
                    let input_char = record_chars[i];
                    if input_char == '?' {
                        '.'
                    } else {
                        input_char
                    }
                }
            }
        }

        if check_bad_records(&result, bad_records) {
            count += 1;
        }
    }

    count
}

fn check_bad_records(result: &[char], expected: &Vec<u8>) -> bool {
    let mut bad_record_idx = 0;
    let bad_record_count = expected.len();
    let mut count = 0;
    for c in result {
        if *c == '#' {
            count += 1;
            if bad_record_idx >= bad_record_count {
                return false;
            }

            if count > expected[bad_record_idx] {
                return false;
            }
            continue;
        }

        if count > 0 {
            if count != expected[bad_record_idx] {
                return false;
            }

            bad_record_idx += 1;
            count = 0;
        }
    }

    if count > 0 {
        bad_record_idx += 1;
    }

    if bad_record_idx != bad_record_count {
        return false;
    }

    true
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut total = 0;
    for line in input.lines() {
        let chunks = line.trim().split_whitespace().collect_vec();
        let records = chunks[0];
        let bad_groups = chunks[1].split(',').map(|s| s.parse().unwrap()).collect();
        total += count_arrangements(records, &bad_groups);
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let mut mask_generator = MaskGenerator::new("?.?");
        assert_eq!(Some(vec![false, false, false]), mask_generator.next());
        assert_eq!(Some(vec![true, false, false]), mask_generator.next());
        assert_eq!(Some(vec![false, false, true]), mask_generator.next());
        assert_eq!(Some(vec![true, false, true]), mask_generator.next());
        assert_eq!(None, mask_generator.next());
    }

    #[test]
    fn test_check_bad_records() {
        assert_eq!(
            false,
            check_bad_records(
                &vec!['.', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '.'],
                &vec![3, 2, 1]
            )
        );
    }

    #[test]
    fn test_count_arrangements() {
        // assert_eq!(1, count_arrangements("???.###", &vec![1, 1, 3]));
        // assert_eq!(4, count_arrangements(".??..??...?##.", &vec![1, 1, 3]));
        // assert_eq!(1, count_arrangements("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]));
        // assert_eq!(1, count_arrangements("????.#...#...", &vec![4, 1, 1]));
        // assert_eq!(4, count_arrangements("????.######..#####.", &vec![1, 6, 5]));
        assert_eq!(10, count_arrangements("?###????????", &vec![3, 2, 1]));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
                     .??..??...?##. 1,1,3
                     ?#?#?#?#?#?#?#? 1,3,1,6
                     ????.#...#... 4,1,1
                     ????.######..#####. 1,6,5
                     ?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}

// Submissions:
// 7716 - correct
