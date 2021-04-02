// use trees::{tr,Tree,Forest};
// use trees::Tree;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use itertools::Itertools;
static BAG_COLOUR: &str = "shiny gold";

lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"(\d{1}) (\w+ \w+) bag").unwrap();
// //3-4 k: bkksggqbtwkkkzqcz
//     static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
//     // static ref BAR_RE: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
//     // static ref HCL_RE: Regex = Regex::new(r"^#([a-z0-9]{6})$").unwrap();
//     // static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
//     // static ref PID_RE: Regex = Regex::new(r"^(\d{9})$").unwrap();
}

#[aoc_generator(day7)]
fn format_input(contents: &str) -> Vec<String> {
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn parse_bag_colours(line: &str) -> Option<(&str, Vec<&str>)> {
    let (outer, inners) = line.splitn(2, " bags contain ").collect_tuple::<(_, _)>()?;
    let inners_fmted = inners
        .split(", ")
        .filter_map(|inner| match inner {
            "no other bags." => None,
            _ => {
                let bag_colour = BAG_RE.captures(inner)?.get(2)?.as_str();
                Some(bag_colour)
            }
        })
        .collect();
    Some((outer, inners_fmted))
}

fn contains_bag(item_map: &HashMap<&str, Vec<&str>>, bag_colour: &str) -> bool {
    if bag_colour == BAG_COLOUR {
        return true;
    }

    item_map.get(bag_colour).map_or(false, |colours| {
        colours
            .iter()
            .map(|colour| contains_bag(item_map, colour))
            .any(|b| b)
    })
}

pub fn find_my_color(rules: &HashMap<&str, Vec<&str>>, bag: &str) -> bool {
    if bag == BAG_COLOUR {
        return true;
    }

    match rules.get(bag) {
        Some(contents) => contents
            .iter()
            .map(|color| find_my_color(rules, color))
            .any(|x| x),
        None => false,
    }
}

#[aoc(day7, part1)]
pub fn day_07_pt_01(input: &[String]) -> usize {
    let item_map: HashMap<&str, Vec<&str>> =
        input.iter().filter_map(|l| parse_bag_colours(l)).collect();

    item_map
        .iter()
        .filter(|(k, _)| **k != BAG_COLOUR && find_my_color(&item_map, BAG_COLOUR))
        .count()
}

// shiny purple bags contain 2 pale blue bags, 1 wavy fuchsia bag, 5 pale salmon bags.
// fn parse_line(line: &str) -> Option<(&str, Vec<(u16, &str)>)> {
fn parse_bags(line: &str) -> Option<(&str, HashMap<&str, u16>)> {
    let (outer, inners) = line.splitn(2, " bags contain ").collect_tuple::<(_, _)>()?;
    let inners_fmted = inners
        .split(", ")
        .filter_map(|inner| match inner {
            "no other bags." => None,
            _ => {
                let bag_captures = BAG_RE.captures(inner)?;
                let bag_cnt = bag_captures.get(1)?.as_str().parse().ok()?;
                let bag_colour = bag_captures.get(2)?.as_str();
                Some((bag_colour, bag_cnt))
            }
        })
        .collect();
    Some((outer, inners_fmted))
}
#[aoc(day7, part2)]
pub fn day_07_pt_02(input: &[String]) -> usize {
    let item_map: HashMap<&str, HashMap<&str, u16>> =
        input.iter().filter_map(|l| parse_bags(l)).collect();
    count_bags(&item_map, "shiny gold") - 1
}

fn count_bags(item_map: &HashMap<&str, HashMap<&str, u16>>, bag_colour: &str) -> usize {
    1 + item_map.get(bag_colour).map_or(0, |bags| {
        bags.iter()
            .map(|(k, v)| *v as usize * count_bags(item_map, k))
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part1_answers_valid() {
        let data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(day_07_pt_01(&format_input(&data)), 4);
    }
    #[test]
    fn check_part2_answers_valid() {
        let data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";
        assert_eq!(day_07_pt_02(&format_input(&data)), 32);
    }
}
