use std::str::FromStr;

pub fn solve(input: &str, is_part_2: bool) -> String {
    match is_part_2 {
        false => solve_1(input),
        true => solve_2(input),
    }.to_string()
}

fn solve_1(input: &str) -> isize {
    let v = parse(&input);

    let mut v = v
        .into_iter()
        .map(|seq| vec![seq] )
        .collect::<Vec<Vec<Vec<isize>>>>();

    build_seq_list(&mut v);

    for seq_list in &mut v {
        seq_list.last_mut().unwrap().push(0);

        for i in (0..seq_list.len() - 1).rev() {
            let prev_seq_last_num = *seq_list[i + 1].last().unwrap();
            let seq = &mut seq_list[i];
            let last_num = *seq.last().unwrap();

            seq.push(last_num + prev_seq_last_num);
        }
    }

    let ans = v
        .into_iter()
        .fold(0, |acc, seq| {
            acc + seq.first().unwrap().last().unwrap()
        });

    ans
}

fn solve_2(input: &str) -> isize {
    let v = parse(&input);

    let mut v = v
        .into_iter()
        .map(|seq| vec![seq] )
        .collect::<Vec<Vec<Vec<isize>>>>();

    build_seq_list(&mut v);

    for seq_list in &mut v {
        seq_list.last_mut().unwrap().push(0);

        for i in (0..seq_list.len() - 1).rev() {
            let prev_seq_first_num = *seq_list[i + 1].first().unwrap();
            let seq = &mut seq_list[i];
            let first_num = *seq.first().unwrap();

            seq.insert(0, first_num - prev_seq_first_num);
        }
    }

    let ans = v
        .into_iter()
        .fold(0, |acc, seq| {
            acc + seq.first().unwrap().first().unwrap()
        });

    ans
}

fn build_seq_list(v: &mut Vec<Vec<Vec<isize>>>) {
    for seq_list in v {
        while !seq_list.last().unwrap().iter().all(|n| *n == 0) {
            let mut new_seq = Vec::new();

            let last_seq = seq_list.last().unwrap();
            for i in 0..last_seq.len() - 1 {
                new_seq.push(last_seq[i + 1] - last_seq[i]);
            }

            seq_list.push(new_seq);
        }
    }

}

fn parse(input: &str) -> Vec<Vec<isize>> {
    let mut v = Vec::new();

    for line in input.lines() {
        v.push(
            line
            .split(" ")
            .map(|s| isize::from_str(s).unwrap())
            .collect()
        );
    }

    v
}
