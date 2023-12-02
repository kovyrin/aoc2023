use crate::custom_error::AocError;

struct EncodedLine {
    line: String,
    index: usize,
}

impl Iterator for EncodedLine {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            dbg!(self.index, self.line[self.index..].to_string());
            if self.line.len() <= self.index {
                return None;
            }
            let line_slice = &self.line[self.index..];

            let c = line_slice.chars().nth(0).unwrap();
            if c.is_numeric() {
                self.index += 1;
                return Some(c);
            }

            match c {
                'o' => {
                    if line_slice.starts_with("one") {
                        self.index += 3;
                        return Some('1');
                    }
                }
                't' => {
                    if line_slice.starts_with("two") {
                        self.index += 3;
                        return Some('2');
                    }
                    if line_slice.starts_with("three") {
                        self.index += 5;
                        return Some('3');
                    }
                }
                'f' => {
                    if line_slice.starts_with("four") {
                        self.index += 4;
                        return Some('4');
                    }
                    if line_slice.starts_with("five") {
                        self.index += 4;
                        return Some('5');
                    }
                }
                's' => {
                    if line_slice.starts_with("six") {
                        self.index += 3;
                        return Some('6');
                    }
                    if line_slice.starts_with("seven") {
                        self.index += 5;
                        return Some('7');
                    }
                }
                'e' => {
                    if line_slice.starts_with("eight") {
                        self.index += 5;
                        return Some('8');
                    }
                }
                'n' => {
                    if line_slice.starts_with("nine") {
                        self.index += 4;
                        return Some('9');
                    }
                }
                _ => {}
            }
            self.index += 1;
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

        let digit1 = encoded_line.next();
        let mut digit2 = encoded_line.last();

        if digit2.is_none() {
            digit2 = digit1;
        }

        let number =
            digit1.unwrap().to_digit(10).unwrap() * 10 + digit2.unwrap().to_digit(10).unwrap();

        println!("{}: {}", line, number);
        sum += number;
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
        let input = "one2three";
        let mut encoded_line = EncodedLine {
            line: input.to_string(),
            index: 0,
        };

        assert_eq!(Some('1'), encoded_line.next());
        assert_eq!(Some('2'), encoded_line.next());
        assert_eq!(Some('3'), encoded_line.next());
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
