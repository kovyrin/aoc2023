use std::collections::HashSet;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Card {
    count: u32,
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
            count: 1,
            winning_nums,
            owned_nums,
        }
    }

    fn matching_cards(&self) -> u32 {
        self.winning_nums.intersection(&self.owned_nums).count() as u32
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        cards.push(Card::from_str(line));
    }

    for i in 0..cards.len() {
        let card = &cards[i];
        let current_card_count = card.count;
        let matching_count = card.matching_cards();

        if matching_count > 0 {
            for j in 0..matching_count {
                cards[j as usize + i as usize + 1].count += current_card_count;
            }
        }
    }

    // Count all the cards we have at the end
    let sum = cards.iter().map(|card| card.count).sum::<u32>();
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
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
