use std::collections::HashMap;
use std::iter::Zip;
use std::ops::Range;
use std::str::Chars;

fn to_number(x: &char) -> usize {
    x.to_digit(10).unwrap() as usize
}

fn iterate_pairs(x: &str) -> Zip<Chars, Chars> {
    x.chars().zip(x[1..].chars())
}

fn at_least_one_double(x: &str) -> bool {
    iterate_pairs(x).any(|(a, b)| a == b)
}

fn at_least_one_double_strict(x: &str) -> bool {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for ch in x.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts.iter().any(|(_, v)| *v == 2)
}

fn always_increase(x: &str) -> bool {
    iterate_pairs(x)
        .map(|(a, b)| (to_number(&a), to_number(&b)))
        .all(|(a, b)| a <= b)
}

fn meets_requirements(x: &String) -> bool {
    x.len() == 6 && at_least_one_double(x) && always_increase(x)
}

fn meets_requirements_part_two(x: &String) -> bool {
    x.len() == 6 && at_least_one_double_strict(x) && always_increase(x)
}

pub fn solve_part_one(range: Range<usize>) -> usize {
    range
        .map(|x| x.to_string())
        .filter(meets_requirements)
        .count()
}

pub fn solve_part_two(range: Range<usize>) -> usize {
    range
        .map(|x| x.to_string())
        .filter(meets_requirements_part_two)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_least_one_double() {
        let values: Vec<(&str, bool)> =
            vec![("11", true), ("12345", false), ("943325344445", true)];
        for (val, expected) in values {
            assert_eq!(at_least_one_double(val), expected);
        }
    }

    #[test]
    fn test_always_increase() {
        let values: Vec<(&str, bool)> =
            vec![("11", true), ("12345", true), ("943325344445", false)];
        for (val, expected) in values {
            assert_eq!(always_increase(val), expected);
        }
    }

    #[test]
    fn test_at_least_one_double_strict() {
        let values: Vec<(&str, bool)> = vec![
            ("112233", true),
            ("123444", false),
            ("111122", true),
            ("111111", false),
        ];
        for (val, expected) in values {
            assert_eq!(at_least_one_double_strict(val), expected);
        }
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(130254..678275), 2090);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(130254..678275), 1419);
    }
}
