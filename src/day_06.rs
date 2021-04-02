use std::collections::HashSet;
use std::hash::Hash;

#[aoc_generator(day6)]
fn format_input(contents: &str) -> Vec<Vec<String>> {
    let line_separator = '\n';
    contents
        .split(&*format!("{}{}", line_separator, line_separator))
        .map(|s| s.split(line_separator).map(|sr| sr.to_string()).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn day_06_pt_01(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group_answers| {
            group_answers
                .iter()
                .flat_map(|passenger_answers| passenger_answers.chars().collect::<Vec<char>>())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn day_06_pt_02(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group_answers| {
            intersect(
                &group_answers
                    .iter()
                    .map(|passenger_answers| passenger_answers.chars().collect::<HashSet<char>>())
                    .collect::<Vec<HashSet<char>>>(),
            )
            .map_or(0, |i| i.len())
        })
        .sum()
}

fn intersect<T: Eq + Hash + Clone>(sets: &[HashSet<T>]) -> Option<HashSet<T>> {
    sets.iter().fold(None, |acc, elem| {
        Some(acc.map_or_else(
            || elem.iter().cloned().collect::<HashSet<_>>(),
            |a| a.intersection(&elem).cloned().collect(),
        ))
    })
}

// #[aoc(day6, part2)]
// pub fn day_06_pt_02(input: &[Vec<String>]) -> usize {
//     input
//         .iter()
//         .map(|group_answers|
//                 group_answers
//                     .iter()
//                     .flat_map(|passenger_answers| passenger_answers.chars().collect::<HashSet<char>>())
//                     .collect::<Vec<char>>()
//         )
//         .
// }
// fn intersect_chars(sets: &[String]) -> Vec<HashSet<char>> {
//     let i = sets.iter().map(|ans| ans.chars().collect::<Vec<char>>())
//     .collect::<Vec<_>>();

//     Vec::new()
// }
// fn intersect<T: Eq + Hash + Clone>(sets: &[Vec<T>]) -> Option<HashSet<T>> {
//     sets.iter().fold(None, |acc, elem| {
//         Some(acc.map_or_else(
//             || elem.iter().cloned().collect::<HashSet<_>>(),
//             |a| a.intersection(&elem).cloned().collect(),
//         ))
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part1_answers_valid() {
        let data = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(day_06_pt_01(&format_input(&data)), 11);
    }

    #[test]
    fn check_part2_answers_valid() {
        let data = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(day_06_pt_02(&format_input(&data)), 6);
    }
}
