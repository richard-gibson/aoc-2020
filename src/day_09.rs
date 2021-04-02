use std::collections::VecDeque;

#[aoc_generator(day9)]
fn format_input(contents: &str) -> Vec<u64> {
    contents
        .lines()
        .map(|l| l.trim().to_string().parse().unwrap())
        .collect()
}

fn contains_sum(sample: &[u64], sum: u64) -> bool {
    (0..sample.len()).into_iter().any(|i| {
        sample
            .iter()
            .any(|e| *e != sample[i] && sum > sample[i] && *e == sum - sample[i])
    })
}

fn find_invalid(search: &[u64], lookback: usize) -> Option<u64> {
    (lookback..search.len() - 1).into_iter().find_map(|i| {
        if !contains_sum(&search[i - lookback..i], search[i]) {
            Some(search[i])
        } else {
            None
        }
    })
}

#[aoc(day9, part1)]
pub fn day_09_pt_01(input: &[u64]) -> u64 {
    find_invalid(input, 25).unwrap()
}

fn contains_set(sample: &[u64], target: u64, upto: usize) -> Option<u64> {
    (0..=upto).into_iter().find_map(|i| {
        let rng = &sample[i..=upto];
        if rng.iter().sum::<u64>() == target {
            Some(rng.iter().min()? + rng.iter().max()?)
        } else {
            None
        }
    })
}

fn find_invalid_sum_set(search: &[u64], invalid_sum: u64) -> Option<u64> {
    (0..search.len())
        .into_iter()
        .find_map(|i| contains_set(search, invalid_sum, i))
}

// #[aoc(day9, part2)]
pub fn day_09_pt_02(input: &[u64]) -> u64 {
    find_invalid_sum_set(input, 1639024365).unwrap()
}

#[aoc(day9, part2)]
pub fn day_09_pt_02_opt(input: &[u64]) -> u64 {
    let target = 1639024365;
    let mut list = VecDeque::new();

    for n in input {
        while list.iter().sum::<u64>() > target {
            list.pop_front();
        }

        if list.iter().sum::<u64>() == target && list.len() > 1 {
            break;
        }

        list.push_back(*n);
    }
    list.iter().min().unwrap() + list.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_day9_part1_answers_valid() {
        let data = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        assert_eq!(find_invalid(&format_input(&data), 5), Some(127));
    }

    #[test]
    fn check_day9_part2_answers_valid() {
        let data = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        assert_eq!(find_invalid_sum_set(&format_input(&data), 127), Some(62));
    }
}
