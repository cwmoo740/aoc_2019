enum Command {
    Reverse,
    Cut(i128),
    Increment(i128),
}

impl From<&str> for Command {
    fn from(x: &str) -> Command {
        if x.starts_with("cut") {
            x.split_whitespace()
                .last()
                .and_then(|z| z.parse::<i128>().ok())
                .map(Command::Cut)
                .unwrap()
        } else if x.starts_with("deal with increment") {
            x.split_whitespace()
                .last()
                .and_then(|z| z.parse::<i128>().ok())
                .map(Command::Increment)
                .unwrap()
        } else {
            Command::Reverse
        }
    }
}

fn reverse(mut x: Vec<usize>) -> Vec<usize> {
    x.reverse();
    x
}

fn cut(mut x: Vec<usize>, count: i128) -> Vec<usize> {
    if count >= 0 {
        x.rotate_left(count as usize);
    } else {
        x.rotate_right(count.abs() as usize)
    }
    x
}

fn increment(x: Vec<usize>, count: usize) -> Vec<usize> {
    let length = x.len();
    let mut result = vec![0; length];
    for (i, val) in x.into_iter().enumerate() {
        result[i * count % length] = val;
    }
    result
}

fn shuffle(mut deck: Vec<usize>, instructions: &str) -> Vec<usize> {
    for command in instructions.lines().map(Command::from) {
        match command {
            Command::Reverse => deck = reverse(deck),
            Command::Increment(z) => deck = increment(deck, z as usize),
            Command::Cut(z) => deck = cut(deck, z),
        }
    }
    deck
}

pub fn solve_part_one() -> usize {
    let deck = (0..10007).collect();
    let instructions = super::get_input::main(22);
    shuffle(deck, &instructions)
        .into_iter()
        .position(|z| z == 2019)
        .unwrap()
}

// taken from https://github.com/enjmusic/aoc_2019/blob/master/aoc_22/src/main.rs
fn mod_pow(base: i128, exp: i128, modulus: i128) -> i128 {
    let mut powers_of_two = vec![base % modulus];
    let mut curr_power = 2;
    while curr_power < exp {
        let last_power = *powers_of_two.last().unwrap();
        powers_of_two.push((last_power * last_power) % modulus);
        curr_power <<= 1;
    }
    let mut out = 1;
    let (mut exponent_process, mut power_idx) = (exp, 0);
    while exponent_process != 0 {
        if (exponent_process & 1) == 1 {
            out = (out * powers_of_two[power_idx]) % modulus;
        }
        exponent_process >>= 1;
        power_idx += 1;
    }
    out
}

/*
need to use some number theory to solve this because the numbers are so big
*/
pub fn solve_part_two() -> i128 {
    let num_cards: i128 = 119_315_717_514_047;
    let num_repeats: i128 = 101_741_582_076_661;

    let (a, b) = super::get_input::main(22)
        .lines()
        .map(Command::from)
        .rev()
        .fold((1, 0), |(a, b), cmd| {
            let (a_new, b_new) = match cmd {
                Command::Reverse => (-a, -b - 1),
                Command::Cut(n) => (a, b + n),
                Command::Increment(n) => {
                    let n = mod_pow(n, num_cards - 2, num_cards);
                    (a * n, b * n)
                }
            };
            (a_new % num_cards, b_new % num_cards)
        });
    let term1 = 2020 * mod_pow(a, num_repeats, num_cards) % num_cards;
    let tmp = (mod_pow(a, num_repeats, num_cards) - 1) * mod_pow(a - 1, num_cards - 2, num_cards)
        % num_cards;
    let term2 = b * tmp % num_cards;
    (term1 + term2) % num_cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let deck = (0..10).collect();
        assert_eq!(increment(deck, 3), vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_shuffle() {
        let values = vec![
            (
                vec![
                    "deal with increment 7",
                    "deal into new stack",
                    "deal into new stack",
                ],
                vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            ),
            (
                vec!["cut 6", "deal with increment 7", "deal into new stack"],
                vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
            ),
            (
                vec![
                    "deal into new stack",
                    "cut -2",
                    "deal with increment 7",
                    "cut 8",
                    "cut -4",
                    "deal with increment 7",
                    "cut 3",
                    "deal with increment 9",
                    "deal with increment 3",
                    "cut -1",
                ],
                vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
            ),
        ];
        for (instructions, result) in values {
            let deck = shuffle((0..10).collect(), &instructions.join("\n"));
            assert_eq!(deck, result);
        }
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 5540);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 6821410630991);
    }
}
