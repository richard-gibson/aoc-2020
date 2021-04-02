use std::collections::HashSet;

#[aoc_generator(day5)]
fn format_input(contents: &str) -> Vec<String> {
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

#[aoc(day5, part1)]
pub fn day_05_pt_01(encoded_seats: &[String]) -> Option<u32> {
    encoded_seats.iter().map(|enc_seat| seat_id(enc_seat)).max()
}

#[aoc(day5, part2)]
pub fn day_05_pt_02(encoded_seats: &[String]) -> u32 {
    let occ_seats: HashSet<u32> = encoded_seats
        .iter()
        .map(|enc_seat| seat_id(enc_seat))
        .collect();

    let start_seat = *occ_seats.iter().min().unwrap();
    let end_seat = *occ_seats.iter().max().unwrap() + 1;
    let all_seats: HashSet<u32> = (start_seat..(end_seat)).into_iter().collect();
    *all_seats.difference(&occ_seats).next().unwrap()
}

fn seat_id(encoded_seat: &str) -> u32 {
    let (enc_row, enc_col) = encoded_seat.split_at(7);
    decode_row(enc_row) * 8 + decode_col(enc_col)
}
fn decode(bsp_row: &str, zero: char, one: char) -> u32 {
    bsp_row.chars().fold(0, |acc, elem| {
        if elem == zero {
            acc << 1
        } else if elem == one {
            acc << 1 | 1
        } else {
            acc
        }
    })
}

fn decode_row(bsp_row: &str) -> u32 {
    decode(bsp_row, 'F', 'B')
}

fn decode_col(bsp_row: &str) -> u32 {
    decode(bsp_row, 'L', 'R')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_test() {
        let bin_idx = "FBFBBFF".replace('F', "0").replace('B', "1");
        let intval = isize::from_str_radix(&bin_idx, 2).unwrap();
        assert_eq!(intval, 44);
    }

    #[test]
    fn check_part1_answers_valid() {
        let data = "FBFBBFFRLR
                    BFFFBBFRRR
                    FFFBBBFRRR
                    BBFFBBFRLL";
        assert_eq!(day_05_pt_01(&format_input(&data)), Some(820));
    }

    #[test]
    fn check_bens_row() {
        assert_eq!(decode_col("RLR"), 7);
    }
}
