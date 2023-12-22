use std::str::FromStr;
use std::collections::HashMap;

pub fn solve() {
    println!("Part 1: {}", solve_1("src/day3/basic_input.txt"));
    println!("Part 2: {}", solve_2("src/day3/basic_input.txt"));
}

fn solve_1(input_path: &str) -> u32 {
    let input = std::fs::read_to_string(input_path).unwrap();

    let mut sch: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut vec_line: Vec<char> = Vec::new();

        for c in line.chars() {
            vec_line.push(c);
        }

        sch.push(vec_line);
    }

    let width = sch[0].len();
    let mut y = 0;
    let mut sum = 0;

    while y < sch.len() {
        let mut x = 0;

        while x < width {
            let mut digits = String::new();
            let mut is_part_number = false;

            while x < width && sch[y][x].is_digit(10) {
                digits.push(sch[y][x]);

                if has_symbol_adjacency(x, y, &sch).is_some() {
                    is_part_number = true;
                }

                x += 1;
            }

            if is_part_number {
                let num = u32::from_str(&digits).unwrap();

                sum += num;
            }

            x += 1;
        }

        y += 1;
    }

    sum
}

fn solve_2(input_path: &str) -> u32 {
    let input = std::fs::read_to_string(input_path).unwrap();

    let mut sch: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut vec_line: Vec<char> = Vec::new();

        for c in line.chars() {
            vec_line.push(c);
        }

        sch.push(vec_line);
    }

    let width = sch[0].len();
    let mut y = 0;
    let mut sum = 0;
    let mut symbols_seen: HashMap<(usize, usize), u32> = HashMap::new();

    while y < sch.len() {
        let mut x = 0;

        while x < width {
            let mut digits = String::new();
            let mut symbol_pos = None;

            while x < width && sch[y][x].is_digit(10) {
                digits.push(sch[y][x]);

                if let Some((symbol_x, symbol_y)) = has_symbol_adjacency(x, y, &sch) {
                    symbol_pos = Some((symbol_x, symbol_y));
                }

                x += 1;
            }

            if symbol_pos.is_some() {
                let symbol_pos = symbol_pos.unwrap();
                let num = u32::from_str(&digits).unwrap();

                if let Some(prev_num) = symbols_seen.get(&symbol_pos) {
                    sum += num * prev_num;
                } else {
                    symbols_seen.insert(symbol_pos, num);
                }
            }

            x += 1;
        }

        y += 1;
    }

    sum
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn has_symbol_adjacency(char_x: usize, char_y: usize, sch: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let width = sch[0].len();
    let height = sch.len();

    for y in char_y.checked_sub(1).unwrap_or(0)..char_y + 2 {
        if y >= height {
            continue;
        }

        for x in char_x.checked_sub(1).unwrap_or(0)..char_x + 2 {
            if x < width && is_symbol(sch[y][x]) {
                return Some((x, y));
            }
        }
    }

    None
}
