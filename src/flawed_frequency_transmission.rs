lazy_static! {
    static ref BASE_PATTERN: Vec<isize> = vec![0, 1, 0, -1];
}
fn make_repeated_pattern(count: usize) -> impl Iterator<Item = isize> {
    BASE_PATTERN
        .iter()
        .flat_map(move |&x| std::iter::repeat(x).take(count))
        .cycle()
        .skip(1)
}

fn fft(signal: &[isize], cycles: usize) -> Vec<isize> {
    let mut result: Vec<isize> = signal.iter().cloned().collect();

    for _cycle in 0..cycles {
        for i in 0..result.len() {
            let repeated_pattern = make_repeated_pattern(i + 1);
            result[i] = result
                .iter()
                .zip(repeated_pattern)
                .map(|(&x, y)| x * y)
                .sum::<isize>()
                .abs()
                % 10;
        }
    }

    result
}

fn repeat_signal(signal: &[isize], repeats: usize) -> Vec<isize> {
    signal
        .iter()
        .cycle()
        .take(signal.len() * repeats)
        .cloned()
        .collect()
}

fn fft_with_repeats_and_offset(
    signal: &[isize],
    repeats: usize,
    offset_digits: usize,
    cycles: usize,
) -> Vec<isize> {
    let mut output: Vec<isize> = repeat_signal(signal, repeats);
    let start_index: usize = signal
        .iter()
        .take(offset_digits)
        .map(|&z| format!("{}", z))
        .collect::<String>()
        .parse()
        .unwrap();
    for _ in 0..cycles {
        let mut partial_sum: isize = output.iter().skip(start_index).sum();
        for j in start_index..output.len() {
            let t = partial_sum;
            partial_sum -= output[j];
            if t >= 0 {
                output[j] = t % 10;
            } else {
                output[j] = (-t) % 10;
            }
        }
    }
    output.into_iter().skip(start_index).take(8).collect()
}

fn get_input() -> Vec<isize> {
    super::get_input::main(16)
        .trim()
        .chars()
        .map(|z| z.to_digit(10).unwrap() as isize)
        .collect()
}

pub fn solve_part_one() -> Vec<isize> {
    fft(&get_input(), 100).into_iter().take(8).collect()
}

pub fn solve_part_two() -> Vec<isize> {
    let repeats: usize = 10_000;
    let offset_digits: usize = 7;
    let cycles: usize = 100;
    fft_with_repeats_and_offset(&get_input(), repeats, offset_digits, cycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_multiple_rounds() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let values: Vec<Vec<isize>> = vec![
            vec![4, 8, 2, 2, 6, 1, 5, 8],
            vec![3, 4, 0, 4, 0, 4, 3, 8],
            vec![0, 3, 4, 1, 5, 5, 1, 8],
            vec![0, 1, 0, 2, 9, 4, 9, 8],
        ];
        for (i, expected) in values.into_iter().enumerate() {
            assert_eq!(fft(&input, i + 1), expected);
        }
    }

    #[test]
    fn test_fft_big() {
        let values: Vec<(Vec<isize>, Vec<isize>)> = vec![
            (
                vec![
                    8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8,
                    6, 4, 5, 5, 9, 5,
                ],
                vec![2, 4, 1, 7, 6, 1, 7, 6],
            ),
            (
                vec![
                    1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4,
                    1, 8, 9, 9, 1, 7,
                ],
                vec![7, 3, 7, 4, 5, 4, 1, 8],
            ),
            (
                vec![
                    6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4,
                    3, 1, 9, 8, 7, 3,
                ],
                vec![5, 2, 4, 3, 2, 1, 3, 3],
            ),
        ];
        for (input, ans) in values {
            assert_eq!(
                fft(&input, 100).into_iter().take(8).collect::<Vec<isize>>(),
                ans
            );
        }
    }

    #[test]
    fn test_fft_with_repeats_and_offset() {
        let values: Vec<(Vec<isize>, Vec<isize>)> = vec![
            (
                vec![
                    0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5,
                    4, 7, 4, 6, 6, 4,
                ],
                vec![8, 4, 4, 6, 2, 0, 2, 6],
            ),
            (
                vec![
                    0, 2, 9, 3, 5, 1, 0, 9, 6, 9, 9, 9, 4, 0, 8, 0, 7, 4, 0, 7, 5, 8, 5, 4, 4, 7,
                    0, 3, 4, 3, 2, 3,
                ],
                vec![7, 8, 7, 2, 5, 2, 7, 0],
            ),
            (
                vec![
                    0, 3, 0, 8, 1, 7, 7, 0, 8, 8, 4, 9, 2, 1, 9, 5, 9, 7, 3, 1, 1, 6, 5, 4, 4, 6,
                    8, 5, 0, 5, 1, 7,
                ],
                vec![5, 3, 5, 5, 3, 7, 3, 1],
            ),
        ];
        for (input, output) in values {
            assert_eq!(fft_with_repeats_and_offset(&input, 10_000, 7, 100), output);
        }
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), vec![2, 5, 1, 3, 1, 1, 2, 8]);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), vec![5, 3, 2, 0, 1, 6, 0, 2]);
    }
}
