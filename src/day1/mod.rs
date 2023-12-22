use std::collections::HashMap;

pub fn solve() {
    println!("Part 1: {}",
        std::fs::read_to_string("src/day1/basic_input_1.txt")
            .unwrap()
            .lines()
            .fold(0, |acc, l| acc + line_num(l, true)));

    println!("Part 2: {}",
        std::fs::read_to_string("src/day1/basic_input_2.txt")
            .unwrap()
            .lines()
            .fold(0, |acc, l| acc + line_num(l, false)));

}

fn line_num(s: &str, digits_only: bool) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut front: Option<usize> = None;
    let mut back: Option<usize> = None;

    for i in 0..chars.len() {
        let c = chars[i];

        if c.is_digit(10) {
            let d = c.to_digit(10).unwrap() as usize;

            if front.is_none() {
                front = Some(d);
            }

            back = Some(d);
        } else if !digits_only {
            if let Some(digit_word) = get_digit_word(&s[i..]) {
                if front.is_none() {
                    front = Some(digit_word);
                }

                back = Some(digit_word);
            }
        }
    }

    back.unwrap() + front.unwrap() * 10
}

fn get_digit_word(s: &str) -> Option<usize> {
    let digit_words: HashMap<&str, usize> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].into();

    for (word, num) in digit_words {
        if s.starts_with(word) {
            return Some(num);
        }
    }

    None
}
