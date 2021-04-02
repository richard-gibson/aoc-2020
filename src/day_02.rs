use std::ops::RangeInclusive;

struct Policy {
    occurances: RangeInclusive<usize>,
    pswd: String,
    test_char: char,
}

#[aoc_generator(day2)]
fn read_input_policy(contents: &str) -> Vec<Policy> {
    contents
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter_map(|l| parse_policy(l))
        .collect()
}

fn parse_range(range_str: &str) -> Option<RangeInclusive<usize>> {
    let v: Vec<usize> = range_str.split('-').map(|s| s.parse().unwrap()).collect();
    if v.len() == 2 {
        Some(v.get(0)?.clone()..=v.get(1)?.clone())
    } else {
        None
    }
}

fn trim_test_char(t: &str) -> Option<char> {
    if t.len() == 1 || (t.ends_with(":") && t.len() == 2) {
        t.chars().next()
    } else {
        None
    }
}

fn parse_policy(policy_line: &str) -> Option<Policy> {
    let v: Vec<&str> = policy_line.split(' ').collect();
    let range = parse_range(v.get(0)?)?;
    let test_char = trim_test_char(v.get(1)?)?;
    Some(Policy {
        occurances: range,
        pswd: v.get(2)?.to_string(),
        test_char: test_char,
    })
}

fn valid_policy(p: &Policy) -> bool {
    let t_char_cnt = p.pswd.matches(p.test_char).count();
    p.occurances.contains(&t_char_cnt)
}

#[aoc(day2, part1)]
fn day_02_pt_01(input: &Vec<Policy>) -> usize {
    input.iter().filter(|p| valid_policy(p)).count()
}

#[aoc(day2, part2)]
fn day_02_pt_02(input: &Vec<Policy>) -> usize {
    input
        .iter()
        .filter(|p| {
            let pswd_chars = p.pswd.as_bytes();
            let pos1 = p.occurances.start();
            let pos2 = p.occurances.end();
            let f = pswd_chars[*pos1 - 1] as char;
            let l = pswd_chars[*pos2 - 1] as char;
            (l == p.test_char) ^ (f == p.test_char)
        })
        .count()
}
