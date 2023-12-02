use crate::custom_error::AocError;

struct EncodedLine {
    line: String,
    index: usize,
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Iterator for EncodedLine {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.line.len() <= self.index {
                return None;
            }
            let line_slice = &self.line[self.index..];
            self.index += 1;

            let c = line_slice.chars().nth(0).unwrap();
            if c.is_numeric() {
                return Some(c);
            }

            for (pos, word) in WORDS.iter().enumerate() {
                if line_slice.starts_with(word) {
                    let c = std::char::from_digit(pos as u32 + 1, 10).unwrap();
                    return Some(c);
                }
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sum = 0;

    input.lines().for_each(|line| {
        let mut encoded_line = EncodedLine {
            line: line.to_string(),
            index: 0,
        };

        let first_digit = encoded_line.next();
        let mut last_digit = encoded_line.last();

        // If there is only one digit, use it twice
        if last_digit.is_none() {
            last_digit = first_digit;
        }

        let number = first_digit.unwrap().to_digit(10).unwrap() * 10
            + last_digit.unwrap().to_digit(10).unwrap();

        sum += number;
        println!("{}: {} => {}", line, number, sum);
    });

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_words() -> miette::Result<()> {
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        for (number, word) in words.iter().enumerate() {
            let mut encoded_line = EncodedLine {
                line: format!("foo1bar{}baz3boom", word),
                index: 0,
            };
            assert_eq!(Some('1'), encoded_line.next());
            let second_char = encoded_line.next().unwrap();
            assert_eq!(Some('3'), encoded_line.next());

            let extracted_num = second_char.to_digit(10).unwrap() as usize;
            assert_eq!(number + 1, extracted_num);
            assert_eq!(None, encoded_line.next());
        }

        Ok(())
    }

    #[test]
    fn test_iterator() -> miette::Result<()> {
        let input = "one2threefourfive6seven8nine";
        let mut encoded_line = EncodedLine {
            line: input.to_string(),
            index: 0,
        };

        assert_eq!(Some('1'), encoded_line.next());
        assert_eq!(Some('2'), encoded_line.next());
        assert_eq!(Some('3'), encoded_line.next());
        assert_eq!(Some('4'), encoded_line.next());
        assert_eq!(Some('5'), encoded_line.next());
        assert_eq!(Some('6'), encoded_line.next());
        assert_eq!(Some('7'), encoded_line.next());
        assert_eq!(Some('8'), encoded_line.next());
        assert_eq!(Some('9'), encoded_line.next());
        assert_eq!(None, encoded_line.next());

        Ok(())
    }

    #[test]
    fn test_iterator_one_digit() -> miette::Result<()> {
        let input = "foo1bar";
        let mut encoded_line = EncodedLine {
            line: input.to_string(),
            index: 0,
        };

        assert_eq!(Some('1'), encoded_line.next());
        assert_eq!(None, encoded_line.next());

        Ok(())
    }

    #[test]
    fn test_overlap() -> miette::Result<()> {
        let input = "fivezg8jmf6hrxnhgxxttwoneg";
        let mut encoded_line = EncodedLine {
            line: input.to_string(),
            index: 0,
        };

        assert_eq!(Some('5'), encoded_line.next());
        assert_eq!(Some('8'), encoded_line.next());
        assert_eq!(Some('6'), encoded_line.next());
        assert_eq!(Some('2'), encoded_line.next());
        assert_eq!(Some('1'), encoded_line.next());
        assert_eq!(None, encoded_line.next());

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
