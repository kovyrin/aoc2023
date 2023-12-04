use std::collections::HashMap;

use crate::custom_error::AocError;
use crate::util::{CharMap, CharRow};

#[derive(Debug)]
pub struct Gear {
    row: i32,
    col: i32,
    part_number: u32,
}

#[derive(Debug)]
pub struct PartNumber {
    number: u32,
    line_idx: i32,
    start: i32,
    end: i32,
}

impl PartNumber {
    fn from_str(line: &CharRow, line_idx: i32, start: i32, end: i32) -> Self {
        let number = parse_number(line, start, end);
        Self {
            number,
            line_idx,
            start,
            end,
        }
    }

    // Finds all adjacent cells that have a '*' in them and returns their coordinates
    fn find_gears(&self, map: &CharMap) -> Vec<Gear> {
        let lines_to_check = vec![self.line_idx - 1, self.line_idx, self.line_idx + 1];

        let mut gears = Vec::new();
        for row in lines_to_check {
            let line = &map.line(row as i32);
            let start = self.start - 1;
            let end = self.end + 1;

            for col in start..=end {
                let c = line.cell(col);
                if *c == '*' {
                    gears.push(Gear {
                        row,
                        col,
                        part_number: self.number,
                    });
                }
            }
        }

        return gears;
    }
}

pub fn parse_number(line: &CharRow, start: i32, end: i32) -> u32 {
    let number_string = line
        .iter()
        .skip(start as usize)
        .take((end - start + 1) as usize)
        .collect::<String>();
    number_string.parse().unwrap()
}

pub fn parse_numbers(line: &CharRow, line_idx: i32) -> miette::Result<Vec<PartNumber>, AocError> {
    let mut numbers = Vec::new();
    let mut number_start = -1;
    let mut number_end = -1;
    for (x, c) in line.iter().enumerate() {
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
    let map = CharMap::from_str(input, '.');
    let mut gears = Vec::new();
    for (y, line) in map.lines().enumerate() {
        for number in parse_numbers(line, y as i32)? {
            gears.append(&mut number.find_gears(&map));
        }
    }

    // Find all unique gears (by row and line)
    let mut unique_gears = HashMap::new(); // (row, line) -> vec![part_number]
    for gear in gears {
        let part_numbers = unique_gears
            .entry((gear.row, gear.col))
            .or_insert(Vec::new());
        part_numbers.push(gear.part_number);
    }

    // Find a sum of gear ratios (product of all part numbers adjacent to a gear) for gears with 2 or more parts
    let mut sum = 0;
    for part_numbers in unique_gears.values() {
        if part_numbers.len() > 1 {
            sum += part_numbers.iter().product::<u32>();
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
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
