pub fn solve_1() {
    let input = std::fs::read_to_string("src/day10/input.txt").unwrap();

    println!("{}", input);

    let mut pipes = parse_input(&input);
    let (start_x, start_y) = find_start(&pipes);
    let (width, height) = (pipes[0].len(), pipes.len());

    println!("start: ({}, {})", start_x, start_y);
    println!("width, height = ({}, {})", width, height);

    print_pipes(&pipes);

    println!("half distance: {:?}", walk(&mut pipes, start_x, start_y) / 2);

    print_pipes(&pipes);

    println!("space: {}", pipes
        .iter()
        .fold(0, |acc, line| acc + line.into_iter().filter(|c| !['#', '%'].contains(c)).count()));
}

fn flood_fill(pipes: &mut Vec<Vec<char>>, x: usize, y: usize) {
    if x >= pipes[0].len() || y >= pipes.len() {
        return;
    }

    if ['#', '%'].contains(&pipes[y][x]) {
        return;
    }

    pipes[y][x] = '%';

    flood_fill(pipes, x, y.wrapping_sub(1));
    flood_fill(pipes, x, y + 1);
    flood_fill(pipes, x.wrapping_sub(1), y);
    flood_fill(pipes, x + 1, y);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn find_start(pipes: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..pipes.len() {
        for x in 0..pipes[0].len() {
            if pipes[y][x] == 'S' {
                return (x, y)
            }
        }
    }

    panic!()
}

fn print_pipes(pipes: &Vec<Vec<char>>) {
    println!("{}", pipes
        .into_iter()
        .fold(String::new(), |mut acc, line| {
            line.into_iter().for_each(|c| acc.push(*c));
            acc.push('\n');

            acc
        })
        .trim()
    );
}

fn walk(pipes: &mut Vec<Vec<char>>, start_x: usize, start_y: usize) -> usize {
    pipes[start_y][start_x] = '#';

    let mut vertices: Vec<(usize, usize)> = Vec::new();
    vertices.push((start_x, start_y));

    let mut path = vec![(start_x, start_y, 'S')];
    let (mut x, mut y) = (start_x, start_y + 1);

    let mut distance = 1;

    loop {
        //std::thread::sleep_ms(3000);

        //println!("At ({}, {}) - {}", x, y, pipes[y][x]);

        let c = pipes[y][x];

        pipes[y][x] = '#';

        match c {
            '|' => if pipes[y + 1][x] == '#' {
                path.push((x, y, 'N'));
                y -= 1;
            } else {
                path.push((x, y, 'S'));
                y += 1;
            },
            '-' => if pipes[y][x + 1] == '#' {
                path.push((x, y, 'W'));
                x -= 1;
            } else {
                path.push((x, y, 'E'));
                x += 1;
            },
            'L' => if pipes[y - 1][x] == '#' {
                vertices.push((x, y));
                path.push((x, y, 'E'));
                x += 1;
            } else {
                vertices.push((x, y));
                path.push((x, y, 'N'));
                y -= 1;
            },
            'J' => if pipes[y - 1][x] == '#' {
                vertices.push((x, y));
                path.push((x, y, 'W'));
                x -= 1;
            } else {
                vertices.push((x, y));
                path.push((x, y, 'N'));
                y -= 1;
            },
            '7' => if pipes[y + 1][x] == '#' {
                vertices.push((x, y));
                path.push((x, y, 'W'));
                x -= 1;
            } else {
                vertices.push((x, y));
                path.push((x, y, 'S'));
                y += 1;
            },
            'F' => if pipes[y + 1][x] == '#' {
                vertices.push((x, y));
                path.push((x, y, 'E'));
                x += 1;
            } else {
                vertices.push((x, y));
                path.push((x, y, 'S'));
                y += 1;
            },
            _ => break
        };

        distance += 1;
    }

    println!("vertices: {:?}", vertices);

    let area = shoelace_area(&vertices);

    let interior_num = area - (distance / 2) + 1;

    println!("area: {}", area);
    println!("interior_num: {}", interior_num);

    /*
    for (path_x, path_y, dir) in path.into_iter() {
        match dir {
            'N' => flood_fill(pipes, path_x - 1, path_y),
            'E' => flood_fill(pipes, path_x, path_y - 1),
            'S' => flood_fill(pipes, path_x + 1, path_y),
            'W' => flood_fill(pipes, path_x, path_y + 1),
            _ => panic!(),
        }
    }
    */

    distance
}

fn shoelace_area(vertices: &[(usize, usize)]) -> usize {
    let mut sum_1: isize = 0;
    let mut sum_2: isize = 0;

    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2)  = vertices[(i + 1) % vertices.len()];

        sum_1 += (x1 * y2) as isize;
        sum_2 += (y1 * x2) as isize;
    }

    ((sum_1 - sum_2).abs() / 2) as usize
}
