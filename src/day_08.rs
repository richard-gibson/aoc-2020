use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Instruction {
    Jmp(isize),
    Acc(isize),
    Nop(isize),
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = s.splitn(2, ' ').collect_tuple::<(_, _)>().unwrap();
        let i = arg.parse::<isize>().unwrap();
        match op {
            "jmp" => Ok(Instruction::Jmp(i)),
            "acc" => Ok(Instruction::Acc(i)),
            "nop" => Ok(Instruction::Nop(i)),
            _ => unreachable!("Invalid value"),
        }
    }
}

#[aoc_generator(day8)]
fn format_input(contents: &str) -> Vec<Instruction> {
    contents
        .lines()
        .map(|l| l.trim().to_string().parse().unwrap())
        .collect()
}

fn next_loc(instruction: &Instruction, curr_location: &isize, acc: isize) -> (isize, isize) {
    match instruction {
        Instruction::Acc(u) => (*curr_location + 1, acc + u),
        Instruction::Jmp(i) => (*curr_location + i, acc),
        Instruction::Nop(_) => (*curr_location + 1, acc),
    }
}

fn find_loop(instructions: &[Instruction]) -> Option<(isize, isize)> {
    process_rec(&instructions, &0, &mut HashSet::new(), 0)
}

fn process_rec(
    instructions: &[Instruction],
    cur_ins_loc: &isize,
    history: &mut HashSet<isize>,
    acc: isize,
) -> Option<(isize, isize)> {
    if history.contains(cur_ins_loc) {
        return Some((*cur_ins_loc, acc));
    } else {
        history.insert(*cur_ins_loc);
    }
    let next_ins = instructions.get(*cur_ins_loc as usize)?;
    let (upd_loc, upd_acc) = next_loc(&next_ins, cur_ins_loc, acc);
    process_rec(instructions, &upd_loc, history, upd_acc)
}

#[aoc(day8, part1)]
pub fn day_08_pt_01(input: &[Instruction]) -> isize {
    find_loop(input).map_or(0, |(_, acc)| acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_day8_part1_answers_valid() {
        let data = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        assert_eq!(day_08_pt_01(&format_input(&data)), 5);
    }
}
