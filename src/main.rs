use std::str::FromStr;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
/*
mod day9;
mod day10;
mod day11;
*/

fn main() {
    println!("AOC 2023!");

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        panic!("Args format: <day.part> <input_path>");
    }

    let (day, part) = {
        let day_part = args[1].split(".").collect::<Vec<&str>>();

        let day = usize::from_str(day_part[0]).unwrap();
        let part = usize::from_str(day_part[1]).unwrap();

        if day < 1 || day > 25 {
            panic!("Invalid day");
        }

        if ![1, 2].contains(&part) {
            panic!("Part can either be 1 or 2");
        };

        (day, part)
    };

    let input = {
        let input_path = &args[2];

        std::fs::read_to_string(input_path).unwrap()
    };

    let is_part_2 = part == 2;

    let res = match day {
        1 => day1::solve(&input, is_part_2),
        2 => day2::solve(&input, is_part_2),
        3 => day3::solve(&input, is_part_2),
        4 => day4::solve(&input, is_part_2),
        5 => day5::solve(&input, is_part_2),
        6 => day6::solve(&input, is_part_2),
        7 => day7::solve(&input, is_part_2),
        8 => day8::solve(&input, is_part_2),
        _ => unimplemented!(),
    };

    println!("Running day {}, part {}\n", day, part);
    println!("{}", res);
}
