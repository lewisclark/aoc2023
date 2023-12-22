use std::collections::HashMap;

pub fn solve_2() {
    let now = std::time::Instant::now();
    let input_path = "src/day8/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();

    let (directions, map) = parse(&input);
    let directions = directions.chars().collect::<Vec<char>>();

    println!("{:?}", directions);
    println!("{:?}", map);

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

    dbg!(&distances);

    let mut x = distances[0];

    for y in &distances[1..] {
        x = lcm(x, *y);
    }

    dbg!(x);

    println!("elapsed: {:.2?}", now.elapsed());
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    //(a / gcd(a, b)) * b
    (a * b) / gcd(a, b)
}

fn direction_to_index(dir: char) -> usize {
    match dir {
        'L' => 0,
        'R' => 1,
        _ => panic!(),
    }
}

/*
pub fn solve_2() {
    let input_path = "src/day8/basic_input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();

    let (directions, map) = parse(&input);

    println!("{}", directions);
    println!("{:?}", map);

    let mut locations = get_starting_locations(&map);

    let dir_arr = directions.chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!(),
    }).collect::<Vec<usize>>();

    let mut cache: HashMap<(String, usize), (usize, String)> = HashMap::new();
    let mut history = Vec::new();

    for _i in 0..locations.len() {
        history.push(Vec::new());
    }

    loop {
        for x in &history[0] {
            for y in &history[1] {
                if x == y {
                    println!("done");
                    return;
                }
            }
        }

        for i in 0..locations.len() {
            println!("\n");
            let (start_loc, steps) = &mut locations[i];

            if let Some((s, loc)) = cache.get(&(start_loc.to_owned(), *steps % dir_arr.len())) {
                print!("{} ({}) -> ", start_loc, steps);
                *start_loc = loc.to_owned();
                *steps += s;
                println!("{} ({})", start_loc, steps);

                history[i].push(*steps);
                println!("history: {:?}", history);
            } else {
                println!("({}, {}) is not cached", start_loc, *steps % dir_arr.len());

                let mut loc = start_loc.to_owned();
                let mut s = 0;
                let mut run_once = true;

                while !loc.ends_with("Z") || run_once {
                    run_once = false;
                    let dir = dir_arr[s % dir_arr.len()];

                    print!("{} ({}) -> ", loc, s);
                    loc = map.get(&loc).unwrap()[dir].to_owned();
                    s += 1;
                    println!("{} ({})", loc, s);
                }

                cache.insert((start_loc.to_owned(), *steps % dir_arr.len()), (s, loc));
                println!("Saved ({}, {}) = {}", start_loc, *steps % dir_arr.len(), s);
            }
        }
    }

    while !locations.iter().all(|s| s.ends_with("Z")) {
        let index = match directions.next().unwrap() {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        };

        steps += 1;

        for i in 0..locations.len() {
            // if (location[i], steps) in cache
            // increment steps

            locations[i] = map.get(&locations[i]).unwrap()[index].to_owned();
        }

        if steps % 500000000 == 0 {
            println!("current steps: {}", steps);
        }
    }

    println!("steps: {}", steps);
*/

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
