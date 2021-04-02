#[aoc_generator(day10)]
fn format_input(contents: &str) -> Vec<usize> {
    contents
        .lines()
        .map(|l| l.trim().to_string().parse().unwrap())
        .collect()
}

#[aoc(day10, part1)]
pub fn day_10_pt_01(input: &[usize]) -> usize {
    let mut adapters = input.to_vec();
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let (acc_ones, acc_threes, _) =
        adapters
            .iter()
            .fold((0, 0, 0), |(ones, threes, cur_jolt), elem| {
                match elem - cur_jolt {
                    1 => (ones + 1, threes, *elem),
                    3 => (ones, threes + 1, *elem),
                    _ => (ones, threes, *elem),
                }
            });
    acc_ones * acc_threes
}
#[derive(Debug, Clone)]
struct Window {
    third_prev: usize,
    second_prev: usize,
    prev: usize,
}
impl Window {
    fn empty() -> Window {
        Window::with_prev(0)
    }
    fn with_prev(prev: usize) -> Window {
        Window {
            third_prev: 0,
            second_prev: 0,
            prev,
        }
    }
}

#[aoc(day10, part2)]
pub fn day_10_pt_02(input: &[usize]) -> usize {
    let mut adapters = input.to_vec();

    adapters.push(0);
    adapters.sort_unstable();

    //add device j adapter
    adapters.push(adapters.last().unwrap() + 3);

    let (path_window, _) = adapters.iter().enumerate().skip(1).fold(
        (Window::with_prev(1), Window::empty()),
        |(path_window, adapter_window), (idx, elem)| {
            let Window {
                third_prev: third_prev_path,
                second_prev: second_prev_path,
                prev: prev_path,
            } = path_window;

            let Window {
                third_prev: third_prev_adp,
                second_prev: second_prev_adp,
                prev: prev_adp,
            } = init_window_values(&adapter_window, (idx, *elem)).unwrap_or(adapter_window);

            let third_prev_inc = if third_prev_adp != 0 && elem - third_prev_adp <= 3 {
                third_prev_path
            } else {
                0
            };
            let sec_prev_inc = if second_prev_adp != 0 && elem - second_prev_adp <= 3 {
                second_prev_path
            } else {
                0
            };
            let prev_inc = if prev_adp != 0 && elem - prev_adp <= 3 {
                prev_path
            } else {
                0
            };
            (
                Window {
                    third_prev: second_prev_path,
                    second_prev: prev_path,
                    prev: third_prev_inc + sec_prev_inc + prev_inc,
                },
                Window {
                    third_prev: second_prev_adp,
                    second_prev: prev_adp,
                    prev: *elem,
                },
            )
        },
    );
    path_window.prev
}

fn init_window_values(window: &Window, elem: (usize, usize)) -> Option<Window> {
    let (idx, elem_val) = elem;
    if idx > 3 {
        None
    } else {
        let Window {
            third_prev,
            second_prev,
            prev,
        } = window;
        Some(Window {
            third_prev: if *third_prev == 0 && idx == 3 {
                elem_val
            } else {
                *third_prev
            },
            second_prev: if *second_prev == 0 && idx == 2 {
                elem_val
            } else {
                *second_prev
            },
            prev: if *prev == 0 && idx == 1 {
                elem_val
            } else {
                *prev
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_day10_part1_answers_valid() {
        let data = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        assert_eq!(day_10_pt_01(&format_input(&data)), 220);
    }

    #[test]
    fn check_day10_part2_sample_answers_valid() {
        let data = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        assert_eq!(day_10_pt_02(&format_input(&data)), 19208);
    }
}
