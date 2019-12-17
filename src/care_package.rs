use super::intcode::Computer;

#[derive(Default)]
struct Game {
    ball: (i64, i64),
    paddle: (i64, i64),
    score: i64,
    computer: Computer,
}

impl Game {
    fn play(&mut self) -> i64 {
        loop {
            match (self.computer.next(), self.computer.next(), self.computer.next()) {
                (Some(-1), Some(0), Some(score)) => {
                    self.score = score
                }
                (Some(x), Some(y), Some(3)) => {
                    self.paddle = (x, y)
                }
                (Some(x), Some(y), Some(4)) => {
                    self.ball = (x, y)
                }
                (None, None, None) => break,
                (_, _, _) => (),
            }
            self.computer.set_default_input((self.ball.0 - self.paddle.0).signum())
        }
        self.score
    }
}


pub fn solve_part_one() -> usize {
    let computer = Computer::new(Computer::load_data(13), &[]);
    computer
        .collect::<Vec<i64>>()
        .chunks_exact(3)
        .filter(|&chunk| chunk[2] == 2)
        .count()
}

pub fn solve_part_two() -> i64 {
    let mut data = Computer::load_data(13);
    data[0] = 2;
    let computer = Computer::new(data, &[]);
    let mut game = Game {
        ball: (0, 0),
        paddle: (0, 0),
        score: 0,
        computer,
    };
    game.play()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 309);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 15410);
    }
}