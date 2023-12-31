use crate::custom_error::AocError;

#[derive(Debug)]
pub struct PartNumber {
    number: u32,
    line_idx: usize,
    start: usize,
    end: usize,
}

impl PartNumber {
    fn from_str(line: &str, line_idx: usize, start: i32, end: i32) -> Self {
        let number = parse_number(line, start, end);
        Self {
            number,
            line_idx,
            start: start as usize,
            end: end as usize,
        }
    }

    // Checks if the part number is real (is has any non-'.' character anywhere in an adjacent cell)
    fn is_real(&self, map: &[String]) -> bool {
        let mut lines_to_check = vec![self.line_idx];
        if self.line_idx > 0 {
            lines_to_check.push(self.line_idx - 1)
        }
        if self.line_idx < map.len() - 1 {
            lines_to_check.push(self.line_idx + 1)
        }

        for line_idx in lines_to_check {
            let line = &map[line_idx];
            let start = if self.start > 0 {
                self.start - 1
            } else {
                self.start
            };
            let end = if self.end < line.len() - 1 {
                self.end + 1
            } else {
                self.end
            };

            for x in start..=end {
                let c = line.chars().nth(x).unwrap();
                if !c.is_numeric() && c != '.' {
                    return true;
                }
            }
        }

        false
    }
}

pub fn parse_number(line: &str, start: i32, end: i32) -> u32 {
    line[start as usize..=end as usize].parse().unwrap()
}

pub fn parse_numbers(line: &str, line_idx: usize) -> miette::Result<Vec<PartNumber>, AocError> {
    let mut numbers = Vec::new();
    let mut number_start = -1;
    let mut number_end = -1;
    for (x, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if number_start == -1 {
                number_start = x as i32;
            }
            number_end = x as i32;
        } else {
            if number_start != -1 {
                numbers.push(PartNumber::from_str(
                    line,
                    line_idx,
                    number_start,
                    number_end,
                ));
                number_start = -1;
                number_end = -1;
            }
        }
    }

    if number_start != -1 {
        numbers.push(PartNumber::from_str(
            line,
            line_idx,
            number_start,
            number_end,
        ))
    }

    Ok(numbers)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();
    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        // scan the line to find all numbers (including multi-digit numbers)
        let numbers = parse_numbers(line, y)?;
        for number in numbers {
            if number.is_real(&lines) {
                sum += number.number;
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
                     ...*......
                     ..35..633.
                     ......#...
                     617*......
                     .....+.58.
                     ..592.....
                     ......755.
                     ...$.*....
                     .664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
