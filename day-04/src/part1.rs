use std::collections::HashSet;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<u32>,
    owned_nums: HashSet<u32>,
}

impl Card {
    fn from_str(line: &str) -> Self {
        let tickets = line.split(":").last().unwrap().trim();
        let num_parts = tickets.split("|").collect::<Vec<&str>>();
        let winning_nums = num_parts[0]
            .trim()
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect::<HashSet<u32>>();

        let owned_nums = num_parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect::<HashSet<u32>>();

        Self {
            winning_nums,
            owned_nums,
        }
    }

    fn score(&self) -> u32 {
        let matching_nums = self.winning_nums.intersection(&self.owned_nums).count() as u32;
        if matching_nums == 0 {
            return 0;
        }
        return (2 as u32).pow(matching_nums - 1);
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        let card = Card::from_str(line);
        sum += card.score();
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
