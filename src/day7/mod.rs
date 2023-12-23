use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::Ordering;

type Bid = u64;

// poor decision
const PART_2: bool = true;

pub fn solve(input: &str, _is_part_2: bool) -> String {
    let mut hand_bids = parse_input(&input);

    hand_bids.sort_by(|(hand1, _bid1), (hand2, _bid2)| {
        hand1.cmp(&hand2)
    });

    hand_bids
        .into_iter()
        .enumerate()
        .fold(0, |x, (i, (_hand, bid))| x + (i + 1) * bid as usize)
        .to_string()
}

fn parse_input(input: &str) -> Vec<(Hand, Bid)> {
    let mut hand_bid_pairs = Vec::new();

    for line in input.lines() {
        hand_bid_pairs.push(parse_hand_bid_pair(line));
    }

    hand_bid_pairs
}

fn parse_hand_bid_pair(line: &str) -> (Hand, Bid) {
    let sep = line.find(" ").unwrap();

    (
        Hand::from_str(&line[..sep]).unwrap(),
        Bid::from_str(&line[sep + 1..]).unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> Option<HandType> {
        if self.has_occurrences(&[5]) || self.cards.into_iter().all(|c| c == Card::CJ) {
            Some(HandType::FiveOfAKind)
        } else if self.has_occurrences(&[4]) {
            Some(HandType::FourOfAKind)
        } else if self.has_occurrences(&[3, 2]) {
            Some(HandType::FullHouse)
        } else if self.has_occurrences(&[3]) {
            Some(HandType::ThreeOfAKind)
        } else if self.has_occurrences(&[2, 2, 1]) {
            Some(HandType::TwoPair)
        } else if self.has_occurrences(&[2, 1, 1]) {
            Some(HandType::OnePair)
        } else if self.has_occurrences(&[1, 1, 1, 1, 1]) {
            Some(HandType::HighCard)
        } else {
            None
        }
    }

    fn occurrences(&self) -> (HashMap<Card, usize>, usize) {
        let mut m = HashMap::new();

        for c in &self.cards {
            if c != &Card::CJ {
                *m.entry(c.clone()).or_insert(0) += 1;
            }
        }

        (m, self.num_jokers())
    }

    fn has_occurrences(&self, occurrences: &[usize]) -> bool {
        let (cur_occurrences, mut jokers) = self.occurrences();
        let mut cur_occurrences = cur_occurrences.into_values().collect::<Vec<usize>>();

        let mut occurrences = occurrences.to_vec();
        for i in 0..occurrences.len() {
            while occurrences[i] > 0 && jokers > 0 {
                occurrences[i] -= 1;
                jokers -= 1;
            }
        }

        let mut have = 0;

        for o1 in occurrences.iter() {
            for (i, o2) in cur_occurrences.iter().enumerate() {
                if o1 == o2 {
                    have += 1;
                    cur_occurrences.remove(i);
                    break;
                }
            }
        }

        have == occurrences.len()
    }

    fn num_jokers(&self) -> usize {
        self.cards.into_iter().filter(|c| c == &Card::CJ).count()
    }
}

impl FromStr for Hand {
    type Err = usize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::Invalid; 5];

        for (i, c) in s.chars().enumerate() {
            cards[i] = Card::from_char(c);
        }

        Ok(Hand { cards } )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let c = self.hand_type().partial_cmp(&other.hand_type()).unwrap();

        if c != Ordering::Equal {
            return c;
        }

        for i in 0..self.cards.len() {
            if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            } else if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Card {
    CA,
    CK,
    CQ,
    CJ,
    CT,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    Invalid,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::CA,
            'K' => Self::CK,
            'Q' => Self::CQ,
            'J' => Self::CJ,
            'T' => Self::CT,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => Self::Invalid,
        }
    }

    fn weight(&self) -> usize {
        match PART_2 {
            false => match self {
                Self::CA => 13,
                Self::CK => 12,
                Self::CQ => 11,
                Self::CJ => 10,
                Self::CT => 9,
                Self::C9 => 8,
                Self::C8 => 7,
                Self::C7 => 6,
                Self::C6 => 5,
                Self::C5 => 4,
                Self::C4 => 3,
                Self::C3 => 2,
                Self::C2 => 1,
                _ => usize::max_value(),
            },
            true => match self {
                Self::CA => 13,
                Self::CK => 12,
                Self::CQ => 11,
                Self::CT => 10,
                Self::C9 => 9,
                Self::C8 => 8,
                Self::C7 => 7,
                Self::C6 => 6,
                Self::C5 => 5,
                Self::C4 => 4,
                Self::C3 => 3,
                Self::C2 => 2,
                Self::CJ => 1,
                _ => usize::max_value(),
            },
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.weight().cmp(&other.weight()))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn weight(&self) -> usize {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.weight().cmp(&other.weight()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hand_type_wrap(hand: &str, expected: HandType) {
        assert_eq!(Hand::from_str(hand).unwrap().hand_type(), Some(expected));
    }

    #[test]
    fn test_hand_type() {
        test_hand_type_wrap("32T3K", HandType::OnePair);
        test_hand_type_wrap("3K55Q", HandType::OnePair);
        test_hand_type_wrap("T55J5", HandType::FourOfAKind);
        test_hand_type_wrap("KTJJT", HandType::FourOfAKind);
        test_hand_type_wrap("QQQJA", HandType::FourOfAKind);
        test_hand_type_wrap("KK677", HandType::TwoPair);
        test_hand_type_wrap("J7777", HandType::FiveOfAKind);
        test_hand_type_wrap("J7A67", HandType::ThreeOfAKind);
        test_hand_type_wrap("JJJJJ", HandType::FiveOfAKind);
        test_hand_type_wrap("KJJJJ", HandType::FiveOfAKind);
        test_hand_type_wrap("79793", HandType::TwoPair);
        test_hand_type_wrap("QJ579", HandType::OnePair);
        test_hand_type_wrap("Q65Q3", HandType::OnePair);
        test_hand_type_wrap("J65J3", HandType::ThreeOfAKind);
        test_hand_type_wrap("QQJJJ", HandType::FiveOfAKind);
        test_hand_type_wrap("KJJKK", HandType::FiveOfAKind);
        test_hand_type_wrap("97J58", HandType::OnePair);
        test_hand_type_wrap("6K6JK", HandType::FullHouse);
        test_hand_type_wrap("TTTTA", HandType::FourOfAKind);
        test_hand_type_wrap("JKJKQ", HandType::FourOfAKind);
        test_hand_type_wrap("JJ5T5", HandType::FourOfAKind);
        test_hand_type_wrap("JQQ8Q", HandType::FourOfAKind);
        test_hand_type_wrap("QJ94T", HandType::OnePair);
        test_hand_type_wrap("77J26", HandType::ThreeOfAKind);
    }
}
