pub fn solve(input: &str, is_part_2: bool) -> String {
    let mut map = parse(&input);

    match is_part_2 {
        false => better_dist_sum(&mut map, 2),
        true => better_dist_sum(&mut map, 1000000),
    }.to_string()
}

/*
fn basic_dist_sum(map: &mut Vec<Vec<char>>, expand_factor: usize) -> usize {
    expand_map(map, expand_factor);

    let galaxies = get_galaxies(&map);
    let num_galaxies = galaxies.len();

    let mut dist_sum = 0;

    for g1 in 0..num_galaxies - 1 {
        for g2 in g1 + 1..num_galaxies {
            let (g1_x, g1_y) = galaxies[g1];
            let (g2_x, g2_y) = galaxies[g2];

            let dist = (g1_x as isize - g2_x as isize).abs()
                + (g1_y as isize - g2_y as isize).abs();

            //println!("Galaxy {} ({}, {}) <--> {} ({}, {}) = {}",
                //g1 + 1, g1_x, g1_y, g2 + 1, g2_x, g2_y, dist);

            dist_sum += dist;
        }
    }

    dist_sum as usize
}
*/

fn better_dist_sum(map: &mut Vec<Vec<char>>, expand_factor: isize) -> usize {
    let galaxies = get_galaxies(&map);
    let num_galaxies = galaxies.len();

    let mut dist_sum = 0;

    for g1 in 0..num_galaxies - 1 {
        for g2 in g1 + 1..num_galaxies {
            let (g1_x, g1_y) = galaxies[g1];
            let (g2_x, g2_y) = galaxies[g2];

            //println!("Galaxy {} ({}, {}) <--> {} ({}, {})",
                //g1 + 1, g1_x, g1_y, g2 + 1, g2_x, g2_y);

            let x_dist = (g1_x as isize - g2_x as isize).abs();
            let y_dist = (g1_y as isize - g2_y as isize).abs();

            let num_empty_rows = get_num_empty_rows(map, g1_y, g2_y) as isize;
            let num_empty_columns = get_num_empty_columns(map, g1_x, g2_x) as isize;

            let dist = (x_dist + num_empty_columns * (expand_factor - 1))
                + (y_dist + num_empty_rows * (expand_factor - 1));

            dist_sum += dist;
        }
    }

    dist_sum as usize
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

/*
fn print_map(map: &Vec<Vec<char>>) {
    map
        .iter()
        .for_each(|line| println!("{}", line.iter().collect::<String>()));
}

fn expand_map(map: &mut Vec<Vec<char>>, mut expand_factor: usize) {
    expand_factor -= 1; // account for already present empty one

    { // Expand rows
        let mut y = 0;
        while y < map.len() {
            if is_empty_row(map, y) {
                for _ in 0..expand_factor {
                    map.insert(y, map[y].clone());
                    y += 1;
                }
            }

            y += 1;
        }
    }

    { // Expand columns
        let mut x = 0;
        while x < map[0].len() {
            if is_empty_column(map, x) {
                for _ in 0..expand_factor {
                    expand_column(map, x);
                    x += 1;
                }
            }

            x += 1;
        }
    }
}
*/

fn is_empty_row(map: &Vec<Vec<char>>, y: usize) -> bool {
    map[y].iter().all(|c| *c == '.')
}

fn is_empty_column(map: &Vec<Vec<char>>, x: usize) -> bool {
    let mut y = 0;

    while y < map.len() {
        if map[y][x] != '.' {
            return false;
        }

        y += 1;
    }

    true
}

/*
fn expand_column(map: &mut Vec<Vec<char>>, x: usize) {
    for y in 0..map.len() {
        map[y].insert(x, '.');
    }
}
*/

fn get_galaxies(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    galaxies
}

fn get_num_empty_columns(map: &Vec<Vec<char>>, x1: usize, x2: usize) -> usize {
    let mut count = 0;

    let (start_x, end_x) = if x2 >= x1 {
        (x1, x2)
    } else {
        (x2, x1)
    };

    for x in start_x + 1..end_x {
        if is_empty_column(&map, x) {
            count += 1;
        }
    }

    count
}

fn get_num_empty_rows(map: &Vec<Vec<char>>, y1: usize, y2: usize) -> usize {
    let mut count = 0;

    for y in y1 + 1..y2 {
        if is_empty_row(&map, y) {
            count += 1;
        }
    }

    count
}
