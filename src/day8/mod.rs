use std::collections::HashMap;

pub fn solve(input: &str, is_part_2: bool) -> String {
    match is_part_2 {
        false => solve_1(input),
        true => solve_2(input),
    }.to_string()
}

fn solve_1(input: &str) -> usize {
    let (directions, map) = parse(&input);
    let directions = directions.chars().collect::<Vec<char>>();

    let mut location = get_starting_locations(&map)[0].to_owned();
    let mut steps = 0;

    while location != "ZZZ" {
        let dir_c = directions[steps % directions.len()];
        let index = direction_to_index(dir_c);
        location = map.get(&location).unwrap()[index].to_owned();

        steps += 1;
    }

    steps
}

fn solve_2(input: &str) -> usize {
    let (directions, map) = parse(&input);
    let directions = directions.chars().collect::<Vec<char>>();

    let mut locations = get_starting_locations(&map);

    let mut distances = Vec::new();

    for i in 0..locations.len() {
        let mut steps = 0;

        while !locations[i].ends_with("Z") {
            let index = direction_to_index(directions[steps % directions.len()]);

            locations[i] = map
                .get(&locations[i])
                .unwrap()[index]
                .to_owned();

            steps += 1;
        }

        distances.push(steps);
    }

    let mut x = distances[0];

    for y in &distances[1..] {
        x = lcm(x, *y);
    }

    x
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
    //(a * b) / gcd(a, b) // can overflow if a and b are very large
}

fn direction_to_index(dir: char) -> usize {
    match dir {
        'L' => 0,
        'R' => 1,
        _ => panic!(),
    }
}

fn get_starting_locations(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    map.keys().filter_map(|s| {
        match s.ends_with("A") {
            true => Some(s.to_owned()),
            false => None,
        }
    }).collect()
}

fn parse(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let lines = input.lines().collect::<Vec<&str>>();

    let directions = lines[0].trim();
    let mut map = HashMap::new();

    for line in &lines[2..] {
        let eq_pos = line.find(" = ").unwrap();

        let key = (&line[..eq_pos]).to_string();

        let values = (&line[eq_pos + 4..line.len() - 1])
            .split(", ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        map.insert(key, values);
    }

    (directions.to_string(), map)
}
