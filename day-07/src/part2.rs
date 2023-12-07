use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "J" => Some(Self::Joker),
            "A" => Some(Self::Ace),
            "K" => Some(Self::King),
            "Q" => Some(Self::Queen),
            "T" => Some(Self::Ten),
            "9" => Some(Self::Nine),
            "8" => Some(Self::Eight),
            "7" => Some(Self::Seven),
            "6" => Some(Self::Six),
            "5" => Some(Self::Five),
            "4" => Some(Self::Four),
            "3" => Some(Self::Three),
            "2" => Some(Self::Two),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> HandType {
        let joker_count = cards.iter().filter(|c| **c == Card::Joker).count();
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        let counts_by_card = cards
            .iter()
            .filter(|c| **c != Card::Joker)
            .counts_by(|c| *c);
        let counts = counts_by_card.values().sorted().rev().collect_vec();

        match counts[0] + joker_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn from_str(s: &str) -> Self {
        println!("parsing hand: {}", s);
        let mut cards = Vec::new();
        for c in s.chars() {
            if let Some(card) = Card::from_str(&c.to_string()) {
                cards.push(card);
            } else {
                panic!("Invalid card: {}", c);
            }
        }

        let hand_type = HandType::from_cards(&cards);
        Self { cards, hand_type }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type = self.hand_type;
        let other_hand_type = other.hand_type;
        if hand_type != other_hand_type {
            return hand_type.partial_cmp(&other_hand_type);
        }

        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            match card1.cmp(card2) {
                std::cmp::Ordering::Equal => continue,
                other => return Some(other),
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

struct Play {
    hand: Hand,
    bid: u64,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut plays = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = Hand::from_str(parts.next().unwrap());
            let bid = parts.next().unwrap().parse::<u64>().unwrap();
            Play { hand, bid }
        })
        .collect_vec();

    plays.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

    let mut result = 0;
    for (i, play) in plays.iter().enumerate() {
        result += play.bid * (i + 1) as u64;
    }

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        let hand = Hand::from_str("32T3K");
        assert_eq!(
            hand,
            Hand {
                cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                hand_type: HandType::OnePair
            }
        );
    }

    #[test]
    fn test_card_from_str() {
        assert_eq!(Card::from_str("A"), Some(Card::Ace));
        assert_eq!(Card::from_str("K"), Some(Card::King));
        assert_eq!(Card::from_str("Q"), Some(Card::Queen));
        assert_eq!(Card::from_str("J"), Some(Card::Joker));
        assert_eq!(Card::from_str("T"), Some(Card::Ten));
        assert_eq!(Card::from_str("9"), Some(Card::Nine));
        assert_eq!(Card::from_str("8"), Some(Card::Eight));
        assert_eq!(Card::from_str("7"), Some(Card::Seven));
        assert_eq!(Card::from_str("6"), Some(Card::Six));
        assert_eq!(Card::from_str("5"), Some(Card::Five));
        assert_eq!(Card::from_str("4"), Some(Card::Four));
        assert_eq!(Card::from_str("3"), Some(Card::Three));
        assert_eq!(Card::from_str("2"), Some(Card::Two));
    }

    #[test]
    fn test_card_ord() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Ten);
        assert!(Card::Ten > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Eight > Card::Seven);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Six > Card::Five);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Three > Card::Two);
        assert!(Card::Two > Card::Joker);
    }

    #[test]
    fn test_hand_type() {
        let tests = vec![
            ("AAAAA", HandType::FiveOfAKind),
            ("AA8AA", HandType::FourOfAKind),
            ("23332", HandType::FullHouse),
            ("TTT98", HandType::ThreeOfAKind),
            ("23432", HandType::TwoPair),
            ("A23A4", HandType::OnePair),
            ("23456", HandType::HighCard),
            ("T55J5", HandType::FourOfAKind),
            ("KTJJT", HandType::FourOfAKind),
            ("QQQJA", HandType::FourOfAKind),
            ("JJJJJ", HandType::FiveOfAKind),
            ("2345J", HandType::OnePair),
        ];

        for (hand_str, expected_hand_type) in tests {
            let hand = Hand::from_str(hand_str);
            assert_eq!(hand.hand_type, expected_hand_type);
        }
    }

    #[test]
    fn test_hand_type_ord() {
        let five_of_a_kind = Hand::from_str("AAAAA");
        let four_of_a_kind = Hand::from_str("AA8AA");
        assert!(five_of_a_kind > four_of_a_kind);

        let four_of_a_kind = Hand::from_str("AA8AA");
        let full_house = Hand::from_str("23332");
        assert!(four_of_a_kind > full_house);

        let full_house = Hand::from_str("23332");
        let three_of_a_kind = Hand::from_str("TTT98");
        assert!(full_house > three_of_a_kind);

        let three_of_a_kind = Hand::from_str("TTT98");
        let two_pair = Hand::from_str("23432");
        assert!(three_of_a_kind > two_pair);

        let two_pair = Hand::from_str("23432");
        let one_pair = Hand::from_str("A23A4");
        assert!(two_pair > one_pair);

        let five_of_a_kind = Hand::from_str("22222");
        let file_jacks = Hand::from_str("JJJJJ");
        assert!(five_of_a_kind > file_jacks);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
                     T55J5 684
                     KK677 28
                     KTJJT 220
                     QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}

// Submissions:
// 250475140 - too low
// 250665248 - correct
