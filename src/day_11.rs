static AJACENTs: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Space {
    Empty,
    Floor,
    Occupied,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            'L' => Space::Empty,
            '#' => Space::Occupied,
            '.' => Space::Floor,
            _ => unreachable!(format!("unknown char {}", c)),
        }
    }
}

#[aoc_generator(day11)]
fn format_input(contents: &str) -> Vec<Vec<Space>> {
    contents
        .lines()
        .map(|l| l.trim()
        .chars()
        .map(From::from)
        .collect()
        )
        .collect()
}

#[aoc(day11, part1)]
pub fn day_11_pt_01(input: &[Vec<Space>]) -> usize {
   println!("{:?}", input);
   0
}
