use std::str::FromStr;

pub fn solve(input: &str,is_part_2: bool) -> String {
    let races = match is_part_2 {
        false => Race::parse_list(input),
        true => Race::parse_list(&input.replace(" ", "")),
    };

    let mut wins = Vec::new();

    for race in races {
        let mut num_wins = 0;

        for hold_time in 1..race.time {
            let dist_trav = (race.time - hold_time) * hold_time;

            if dist_trav > race.distance {
                num_wins += 1;
            }
        }

        wins.push(num_wins);
    }

    wins.into_iter().fold(1, |v, x| v * x).to_string()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn parse_list(input: &str) -> Vec<Self> {
        let mut races = Vec::new();

        let lines = input.lines().collect::<Vec<&str>>();
        let times = parse_space_sep_nums(&lines[0][5..]);
        let distances = parse_space_sep_nums(&lines[1][9..]);

        for (time, distance) in times.into_iter().zip(distances.into_iter()) {
            races.push(Race { time, distance });
        }

        races
    }
}

fn parse_space_sep_nums(space_sep_nums: &str) -> Vec<u64> {
    let mut v = Vec::new();

    for str_n in space_sep_nums.trim().split(" ") {
        if !str_n.is_empty() {
            v.push(u64::from_str(str_n.trim()).unwrap());
        }
    }

    v
}
