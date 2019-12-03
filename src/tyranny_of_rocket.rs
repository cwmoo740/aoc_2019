fn parse_input(x: &String) -> Vec<isize> {
    x
        .split_whitespace()
        .map(|z| z.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn calc_fuel(weight: isize) -> isize {
    (weight / 3) - 2
}

fn calc_fuel_complicated(weight: isize) -> isize {
    let mut fuel_to_add = calc_fuel(weight);
    let mut total: isize = 0;
    while fuel_to_add > 0 {
        total += fuel_to_add;
        fuel_to_add = calc_fuel(fuel_to_add);
    }
    total
}

pub fn solve_part_one() -> isize {
    let input = super::get_input::main(1);
    parse_input(&input)
        .into_iter()
        .map(calc_fuel)
        .sum()
}

pub fn solve_part_two() -> isize {
    let input = super::get_input::main(1);
    parse_input(&input)
        .into_iter()
        .map(calc_fuel_complicated)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input_str = "12
        14
        1969
        100756".to_string();
        assert_eq!(parse_input(&input_str), vec![12, 14, 1969, 100756]);
    }

    #[test]
    fn test_calc_fuel() {
        let values: Vec<(isize, isize)> = vec![(12, 2), (14, 2), (1969, 654), (100756, 33583)];
        for (x, y) in values {
            assert_eq!(calc_fuel(x), y, "weight={}", x);
        }
    }

    #[test]
    fn test_calc_fuel_complicated() {
        let values: Vec<(isize, isize)> = vec![(14, 2), (1969, 966), (100756, 50346)];
        for (x, y) in values {
            assert_eq!(calc_fuel_complicated(x), y, "weight={}", x);
        }
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 3376997);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 5062623);
    }
}