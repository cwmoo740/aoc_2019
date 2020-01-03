use std::collections::{HashMap, HashSet};

use super::intcode::Computer;

pub fn solve_part_one() -> usize {
    let mut computer = Computer::new(&Computer::load_data(19), &[]);
    let mut affected: HashSet<(i64, i64)> = HashSet::new();
    for y in 0..50 {
        let mut found_beam = false;
        for x in 0..50 {
            computer.reset();
            computer.add_input(&[x, y]);
            match computer.next() {
                Some(z) if z == 0 && !found_beam => (),
                Some(z) if z == 1 => {
                    found_beam = true;
                    affected.insert((x, y));
                }
                Some(z) if z == 0 && found_beam => break,
                _ => panic!("this should not happen"),
            }
        }
    }
    affected.len()
}

fn is_in_tractor_beam(computer: &mut Computer, cache: &mut HashMap<(i64, i64), i64>, &(x, y): &(i64, i64)) -> bool {
    if *cache.get(&(x, y)).unwrap_or(&0) == 1 {
        true
    } else {
        computer.reset();
        computer.add_input(&[x, y]);
        computer.next().unwrap() == 1
    }
}

pub fn solve_part_two() -> i64 {
    let mut computer = Computer::new(&Computer::load_data(19), &[]);
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
    let mut x = 0;
    let mut y = 100;
    loop {
        while !is_in_tractor_beam(&mut computer, &mut cache, &(x, y)) {
            x += 1;
        }

        if is_in_tractor_beam(&mut computer, &mut cache, &(x + 99, y - 99)) {
            return x * 10_000 + y - 99;
        }

        y += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 154);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 9791328)
    }
}