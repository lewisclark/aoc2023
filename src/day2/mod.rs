use std::str::FromStr;

const START_CUBESET: CubeSet = CubeSet::new(12, 13, 14);

pub fn solve(input: &str, is_part_2: bool) -> String {
    match is_part_2 {
        false => solve_1(input),
        true => solve_2(input),
    }.to_string()
}

fn solve_1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.trim();
        let game = Game::from_str(line).unwrap();

        if game.cubesets.iter().all(|c| c.is_possible(&START_CUBESET)) {
            sum += game.id;
        }
    }

    sum
}

fn solve_2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.trim();
        let game = Game::from_str(line).unwrap();
        let max = game.max_cubeset().unwrap();
        let power = max.red * max.green * max.blue;

        sum += power;
    }

    sum
}

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    const fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    const fn is_possible(&self, start: &CubeSet) -> bool {
        self.red <= start.red
            && self.green <= start.green
            && self.blue <= start.blue
    }
}

struct Game {
    id: u32,
    cubesets: Vec<CubeSet>,
}

impl Game {
    fn max_cubeset(&self) -> Option<CubeSet> {
        if self.cubesets.is_empty() {
            return None;
        }

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cubeset in &self.cubesets {
            red = red.max(cubeset.red);
            green = green.max(cubeset.green);
            blue = blue.max(cubeset.blue);
        }

        Some(CubeSet::new(red, green, blue))
    }
}

impl FromStr for Game {
    type Err = usize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_pos = s.find(":").unwrap();
        let game_id = u32::from_str(&s[s.find(" ").unwrap() + 1..colon_pos]).unwrap();
        let mut cubesets: Vec<CubeSet> = Vec::new();

        for cubeset in (&s[colon_pos + 2..]).split(";") {
            let cubeset: Vec<&str> = cubeset.split(",").collect();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube in cubeset {
                let cube = cube.trim();
                let space_pos = cube.find(" ").unwrap();
                let quantity = u32::from_str(&cube[..space_pos]).unwrap();
                let color = &cube[space_pos + 1..];

                let c = match color {
                    "red" => &mut red,
                    "green" => &mut green,
                    "blue" => &mut blue,
                    _ => return Err(0),
                };

                *c += quantity;
            }

            cubesets.push(CubeSet::new(red, green, blue));
        }

        Ok(Self {
            id: game_id,
            cubesets,
        })
    }
}
