use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

// use itertools::Itertools;

use lazy_static::lazy_static;
use regex::Regex;
// use itertools::join;
// use itertools::Itertools;

lazy_static! {
    static ref HGT_RE: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#([a-z0-9]{6})$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^(\d{9})$").unwrap();
    static ref MANDATORY_FIELDS: HashSet<String> =
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(ToString::to_string)
            .collect();
}

// #[aoc_generator(day4)]
// fn parse_lines(input: &str) -> Vec<HashMap<String, String>> {
//     let data: Vec<&str> = input.lines().collect();
//     data.iter()
//         .group_by(|&&l| l.is_empty())
//         .into_iter()
//         .map(|(_, g)| join(g, " "))
//         .filter(|l| !l.is_empty())
//         .map(|s| {
//             s.split(' ')
//                 .filter_map(|elem| {
//                     let kv: Vec<&str> = elem.split(':').collect();
//                     let k = kv.get(0)?.to_string();
//                     let v = kv.get(1)?.to_string();
//                     Some((k, v))
//                 })
//                 .collect::<HashMap<String, String>>()
//         })
//         .collect()
// }

#[aoc_generator(day4)]
fn format_input(contents: &str) -> Vec<HashMap<String, String>> {
    let line_separator = '\n';
    contents
        .split(&*format!("{}{}", line_separator, line_separator))
        .map(|s| {
            s.replace(line_separator, " ")
                .split(' ') // or split_whitespace()
                .filter_map(|elem| {
                    let kv: Vec<&str> = elem.splitn(2, ':').collect();
                    let k = kv.get(0)?.to_string();
                    let v = kv.get(1)?.to_string();
                    Some((k, v))
                }) // should be able to use `elem.splitn(2, ":").collect_tuple::<(_, _)>()`
                .collect()
        })
        .collect()
}

fn validate_fields(passport: &HashMap<String, String>) -> bool {
    let passport_keys: HashSet<String> = passport.keys().cloned().collect();
    passport_keys.intersection(&MANDATORY_FIELDS).count() == MANDATORY_FIELDS.len()
}

#[aoc(day4, part1)]
pub fn day_04_pt_01(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .filter(|passport| validate_fields(passport))
        .count()
}

fn in_range<T: PartialOrd + FromStr>(str_val: Option<&String>, lower: T, upper: T) -> Option<T> {
    let t_val = str_val?.as_str().parse::<T>().ok();
    t_val.filter(|yr| *yr >= lower && *yr <= upper)
}

fn validate_values(passport: &HashMap<String, String>) -> Option<()> {
    let _byr: u32 = in_range(passport.get("byr"), 1920, 2002)?;
    let _iyr: u32 = in_range(passport.get("iyr"), 2010, 2020)?;
    let _eyr: u32 = in_range(passport.get("eyr"), 2020, 2030)?;

    let hgt_captures = HGT_RE.captures(passport.get("hgt")?)?;
    let hgt_val = hgt_captures.get(1)?.as_str().parse::<u32>().ok();
    let hgt_unit = hgt_captures.get(2)?.as_str();
    let hgt_val_in_range = hgt_val.filter(|h| {
        (hgt_unit == "cm" && (h >= &150 && h <= &193))
            || (hgt_unit == "in" && (h >= &59 && h <= &76))
    })?;
    let _hgt = (hgt_val_in_range, hgt_unit.to_string());
    HCL_RE.captures(passport.get("hcl")?)?;
    ECL_RE.captures(passport.get("ecl")?)?;
    PID_RE.captures(passport.get("pid")?)?;

    Some(())
}

#[aoc(day4, part2)]
pub fn day_04_pt_02(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .filter(|passport| validate_fields(passport) && validate_values(passport).is_some())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part2_answers_valid() {
        let data = "pid:087499704 hgt:74in ecl:grn iyr:2p12 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(day_04_pt_02(&format_input(&data)), 3);
    }

    #[test]
    fn check_part2_answers_file() {
        let data = std::include_str!("../input/2020/day4.txt");
        assert_eq!(day_04_pt_02(&format_input(&data)), 172);
    }
}
