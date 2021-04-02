#[aoc_generator(day3)]
fn clean_input(contents: &str) -> Vec<String> {
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn find_trees(input: &[String], rs: &usize, cs: &usize) -> u64 {
    let rows = input.len();
    let mut trees_cnt = 0;
    let (mut row, mut col) = (0, 0);

    while row < rows {
        let cols = input[row].len();

        if input[row].as_bytes()[col % cols] == b'#' {
            trees_cnt += 1
        }

        row += rs;
        col += cs;
    }
    trees_cnt
}

#[aoc(day3, part1)]
pub fn day_03_pt_01(tree_map: &[String]) -> usize {
    let mut trees = 0;
    let mut col: Option<usize> = None;
    for l in tree_map {
        let up_pos = col.map_or(0, |i| i + 3);
        col = Some(up_pos);
        if l.as_bytes()[up_pos % l.len()] == b'#' {
            trees += 1
        }
    }
    trees
}

// #[aoc(day3, part1)]
// pub fn solve_part_01(input: &Vec<String>) -> u64 {
//     find_trees(input, &1, &3)
// }

#[aoc(day3, part2)]
pub fn day_03_pt_02(tree_map: &[String]) -> u64 {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes.iter().fold(1, |acc, (rows, cols)| {
        acc * find_trees(tree_map, rows, cols)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let data = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        assert_eq!(day_03_pt_01(&clean_input(data)), 7)
    }
}
