use std::str::FromStr;

pub fn solve(input_path: &str) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let almanac = Almanac::parse(&input);
    let mut min_result = u64::max_value();

    //println!("seeds: {:?}", almanac.seeds);

    for (i, seed) in almanac.seeds.iter().enumerate() {
        if i % (almanac.seeds.len() / 100) == 0 {
            println!("{}% done ({}, {})",
                ((i as f64 / almanac.seeds.len() as f64) * 100.0).round(),
                i, almanac.seeds.len());
        }

        let mut n = *seed;

        for map_table in &almanac.maps {
            for map in map_table {
                if map.in_src_range(n) {
                    //println!("[{}] translated {} to {} using {:?}",
                        //seed, n, map.translate(n), map);

                    n = map.translate(n);
                    break;
                }
            }

            //println!("[{}] no mapping", seed);
        }

        //println!("soil: {}", n);

        min_result = min_result.min(n);
    }

    println!("min: {}", min_result);
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<Mapping>>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let input = input.trim();
        let mut lines = input.trim().lines();

        //let seeds = Almanac::parse_seeds_1(lines.next().unwrap());
        let seeds = Almanac::parse_seeds_2(lines.next().unwrap());

        lines.next().unwrap(); // skip newline

        let mut master_maps: Vec<Vec<Mapping>> = Vec::new();
        let mut maps: Vec<Mapping> = Vec::new();
        for line in lines {
            if line.is_empty() {
                master_maps.push(maps.clone());
                maps.clear();
            } else if let Some(colon_pos) = line.find(":") { // header
                
            } else {
                maps.push(Mapping::parse(line));
            }
        }

        if !maps.is_empty() {
            master_maps.push(maps.clone());
        }

        Self {
            seeds: seeds.into_iter().flatten().collect(),
            maps: master_maps,
        }
    }

    fn parse_seeds_1(seeds_line: &str) -> Vec<u64> {
        let mut seeds = Vec::new();
        let space_pos = seeds_line.find(" ").unwrap();

        for seed in (&seeds_line[space_pos + 1..]).trim().split(" ") {
            seeds.push(u64::from_str(seed.trim()).unwrap());
        }

        seeds
    }

    fn parse_seeds_2(seeds_line: &str) -> Vec<Vec<u64>> {
        let mut seeds = Vec::new();

        let single_seeds = Almanac::parse_seeds_1(seeds_line);
        println!("single_seeds: {:?}", single_seeds);
        let mut i = 0;
        while i < single_seeds.len() {
            let start = single_seeds[i];
            let length = single_seeds[i + 1];
            let mut s = Vec::new();

            for j in start..start + length {
                s.push(j);
            }

            seeds.push(s);

            i += 2;
        }

        //println!("range seeds: {:?}", seeds);

        seeds
    }
}

#[derive(Clone, Debug)]
struct Mapping {
    dst_start: u64,
    src_start: u64,
    range: u64,
}

impl Mapping {
    fn parse(input: &str) -> Self {
        let nums: Vec<&str> = input.trim().split(" ").collect();

        Self {
            dst_start: u64::from_str(nums[0]).unwrap(),
            src_start: u64::from_str(nums[1]).unwrap(),
            range: u64::from_str(nums[2]).unwrap(),
        }
    }

    fn in_src_range(&self, n: u64) -> bool {
        n >= self.src_start && n < self.src_start + self.range
    }

    fn translate(&self, n: u64) -> u64 {
        self.dst_start + (n - self.src_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {
        let input = "seeds: 950527520 85181200 546703948 123777711 63627802 279111951 1141059215 246466925 1655973293 98210926 3948361820 92804510 2424412143 247735408 4140139679 82572647 2009732824 325159757 3575518161 370114248";

        let seeds = Almanac::parse_seeds_1(input);

        assert_eq!(seeds, [950527520, 85181200, 546703948, 123777711, 63627802, 279111951, 1141059215, 246466925, 1655973293, 98210926, 3948361820, 92804510, 2424412143, 247735408, 4140139679, 82572647, 2009732824, 325159757, 3575518161, 370114248])
    }

    fn test_seed_range(seeds: &[u64], start: u64, length: u64) {
        for i in start..start + length {
            assert_eq!(seeds[(i - start) as usize], i);
        }
    }

    #[test]
    fn test_parse_2() {
        let input = "seeds: 950527520 85181200 546703948 123777711 63627802 279111951 1141059215 246466925 1655973293 98210926 3948361820 92804510 2424412143 247735408 4140139679 82572647 2009732824 325159757 3575518161 370114248";

        let seeds = Almanac::parse_seeds_2(input);

        test_seed_range(&seeds[0], 950527520, 85181200);
        test_seed_range(&seeds[1], 546703948, 123777711);
        test_seed_range(&seeds[2], 63627802, 279111951);
        test_seed_range(&seeds[3], 1141059215, 246466925);
        test_seed_range(&seeds[4], 1655973293, 98210926);
        test_seed_range(&seeds[5], 3948361820, 92804510);
        test_seed_range(&seeds[6], 2424412143, 247735408);
        test_seed_range(&seeds[7], 4140139679, 82572647);
        test_seed_range(&seeds[8], 2009732824, 325159757);
        test_seed_range(&seeds[9], 3575518161, 370114248);
    }

    fn test_mapping_parse(
        map: &str,
        dst_start: u64,
        src_start: u64,
        range: u64
    ) {
        let map = Mapping::parse(map);

        assert_eq!(map.dst_start, dst_start);
        assert_eq!(map.src_start, src_start);
        assert_eq!(map.range, range);
    }

    #[test]
    fn test_mappings() {
        test_mapping_parse("3642212803 1631134559 163560083",
            3642212803, 1631134559, 163560083);

        test_mapping_parse("0 198238182 157249059",
            0, 198238182, 157249059);

        test_mapping_parse("68473128 0 90342895", 68473128, 0, 90342895);
    }
}
