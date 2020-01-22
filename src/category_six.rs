use super::intcode::Computer;
use std::collections::{VecDeque, HashMap, HashSet};

fn run(mut computers: Vec<Computer>, with_looping: bool) -> i64 {
    let mut last_packet: Option<(i64, i64)> = None;
    let mut delivered_last_packet: Option<i64> = None;
    let mut idle: HashSet<usize> = HashSet::new();
    loop {
        if (0..computers.len()).all(|i| idle.contains(&i)) {
            idle.clear();
            if let Some((x, y)) = last_packet {
                match delivered_last_packet {
                    Some(last_y) if last_y == y => return y,
                    _ => (),
                };
                delivered_last_packet = Some(y);
                computers[0].add_input(&[x, y]);
                last_packet = None
            } else {
                panic!("computers are all idle but NAT has no packet");
            }
        }
        for i in 0..computers.len() {
            let computer = &mut computers[i];
            let queue_empty = computer.input_queue.is_empty();
            if queue_empty {
                computer.input_queue.push_back(-1);
            }
            if let Some(dest) = computer.next() {
                idle.remove(&i);
                let (x, y) = (computer.next().unwrap(), computer.next().unwrap());
                if dest == 255 && with_looping {
                    last_packet = Some((x, y));
                } else if dest == 255 {
                    return y;
                } else {
                    let dest_computer = &mut computers[dest as usize];
                    dest_computer.add_input(&[x, y]);
                }
            } else if queue_empty {
                idle.insert(i);
            }
        }
    }
}

fn prepare_computers() -> Vec<Computer> {
    (0..50)
        .map(|i| {
            let mut computer = Computer::new(&Computer::load_data(23), &[i]);
            computer.yield_on_empty = true;
            computer
        })
        .collect()
}

pub fn solve_part_one() -> i64 {
    run(prepare_computers(), false)
}

pub fn solve_part_two() -> i64 {
    run(prepare_computers(), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 19937);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 13758);
    }
}
