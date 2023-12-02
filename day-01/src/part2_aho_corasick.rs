use crate::custom_error::AocError;
use aho_corasick::{AhoCorasick, Match};

const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

pub fn match_to_digit(matched: Option<Match>) -> Option<u32> {
    if let Some(m) = matched {
        let pattern_idx = m.pattern().as_u32();
        let digit = if pattern_idx < 9 {
            pattern_idx + 1
        } else {
            pattern_idx - 9
        };
        return Some(digit);
    } else {
        None
    }
}

pub fn process_line(line: &str, ac: &AhoCorasick) -> u32 {
    let mut iter = ac.find_overlapping_iter(line);
    let first_digit = match_to_digit(iter.next());
    let last_digit = match_to_digit(iter.last()).or(first_digit);
    return first_digit.unwrap() * 10 + last_digit.unwrap();
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let ac = AhoCorasick::new(NUMBERS).unwrap();
    let sum: u32 = input.lines().map(|line| process_line(line, &ac)).sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_to_digit() -> miette::Result<()> {
        let ac = AhoCorasick::new(NUMBERS).unwrap();
        let mut iter = ac.find_overlapping_iter("one");

        assert_eq!(Some(1), match_to_digit(iter.next()));
        assert_eq!(None, match_to_digit(iter.next()));

        Ok(())
    }
    #[test]
    fn test_iterator_words() -> miette::Result<()> {
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        for (number, word) in words.iter().enumerate() {
            let ac = AhoCorasick::new(NUMBERS).unwrap();
            let line = format!("foo1bar{}baz3boom", word);
            let mut iter = ac.find_overlapping_iter(&line);

            assert_eq!(Some(1), match_to_digit(iter.next()));

            let second_num = match_to_digit(iter.next());
            assert_eq!(number + 1, second_num.unwrap() as usize);

            assert_eq!(Some(3), match_to_digit(iter.next()));
            assert_eq!(None, iter.next());
        }

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#;
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
