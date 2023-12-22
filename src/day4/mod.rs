use std::str::FromStr;
use std::collections::HashMap;

pub fn solve() {
    println!("Part 1: {}", solve_1("src/day4/basic_input.txt"));
    println!("Part 2: {}", solve_2("src/day4/basic_input.txt"));
}

fn solve_1(input_path: &str) -> usize {
    let input = std::fs::read_to_string(input_path).unwrap();

    let mut points_sum = 0;

    for card in parse_cards(&input) {
        let mut points = 0;

        for num in card.given_nums {
            if card.winning_nums.contains(&num) {
                points = match points {
                    0 => 1,
                    _ => points * 2,
                };
            }
        }

        points_sum += points;
    }

    points_sum
}

fn solve_2(input_path: &str) -> u32 {
    let input = std::fs::read_to_string(input_path).unwrap();

    let cards = parse_cards(&input);
    let mut total_cards = 0;
    let mut card_winners_cache: HashMap<u32, u32> = HashMap::new();

    for card in cards.iter().rev() {
        let mut cards = 1;

        for i in card.id + 1..card.id + 1 + card.winners() {
            cards += card_winners_cache.get(&i).unwrap();
        }

        assert!(card_winners_cache.insert(card.id, cards).is_none());

        total_cards += cards;
    }

    total_cards
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let first_space_pos = line.find(" ").unwrap();
        let colon_pos = line.find(":").unwrap();
        let id = u32::from_str(&line[first_space_pos + 1..colon_pos].trim()).unwrap();
        let mut num_chunks = Vec::new();

        for num_chunk in (&line[colon_pos + 1..]).split("|") {
            let mut nums = Vec::new();

            for str_num in num_chunk.trim().split(" ") {
                let str_num = str_num.trim();

                if str_num.is_empty() {
                    continue;
                }

                nums.push(u32::from_str(str_num).unwrap());
            }

            num_chunks.push(nums);
        }

        let given_nums = num_chunks.pop().unwrap();
        let winning_nums = num_chunks.pop().unwrap();

        cards.push(Card::new(id, winning_nums, given_nums));
    }

    cards
}

struct Card {
    id: u32,
    winning_nums: Vec<u32>,
    given_nums: Vec<u32>,
}

impl Card {
    fn new(id: u32, winning_nums: Vec<u32>, given_nums: Vec<u32>) -> Self {
        Self {
            id,
            winning_nums,
            given_nums,
        }
    }

    fn winners(&self) -> u32 { 
        let mut count = 0;

        for num in &self.given_nums {
            if self.winning_nums.contains(&num) {
                count += 1;
            }
        }

        count
    }
}
