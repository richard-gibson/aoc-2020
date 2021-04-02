use std::collections::HashSet;

#[aoc_generator(day1)]
fn read_input(contents: &str) -> HashSet<u32> {
    contents
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn day_01_pt_01(input: &HashSet<u32>) -> u32 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn day_01_pt_02(input: &HashSet<u32>) -> u32 {
    for x in input {
        for y in input {
            if (x + y) < 2020 {
                let z = 2020 - (x + y);
                if input.contains(&z) {
                    return x * y * z;
                }
            }
        }
    }
    0
}
