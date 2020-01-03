use super::intcode::Computer;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Space,
    Scaffold,
    Newline,
    RobotUp,
    RobotRight,
    RobotDown,
    RobotLeft,
}

impl From<i64> for Pixel {
    fn from(x: i64) -> Pixel {
        match x {
            35 => Pixel::Scaffold,
            46 => Pixel::Space,
            10 => Pixel::Newline,
            60 => Pixel::RobotLeft,
            62 => Pixel::RobotRight,
            94 => Pixel::RobotUp,
            118 => Pixel::RobotDown,
            _ => panic!("pixel not recognized: {}", x),
        }
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Space => ".",
                Pixel::Scaffold => "#",
                Pixel::Newline => "\n",
                Pixel::RobotUp => "^",
                Pixel::RobotRight => ">",
                Pixel::RobotDown => "v",
                Pixel::RobotLeft => "<",
            }
        )
    }
}

impl Pixel {
    fn is_scaffold(&self) -> bool {
        match self {
            Pixel::RobotRight
            | Pixel::RobotDown
            | Pixel::RobotLeft
            | Pixel::RobotUp
            | Pixel::Scaffold => true,
            _ => false,
        }
    }
}

fn print_image(image: &Vec<Vec<Pixel>>) {
    println!(
        "{}",
        image
            .iter()
            .map(|row| row
                .iter()
                .map(|px| format!("{}", px))
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn get_image(computer: &mut Computer) -> Vec<Vec<Pixel>> {
    computer
        .map(Pixel::from)
        .collect::<Vec<Pixel>>()
        .split(|&x| x == Pixel::Newline)
        .filter(|z| z.len() > 0)
        .map(|z| z.into_iter().cloned().collect::<Vec<Pixel>>())
        .collect()
}

fn get_intersections(image: &Vec<Vec<Pixel>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for x in 1..(image.len() - 1) {
        for y in 1..(image[x].len() - 1) {
            if [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .all(|(a, b)| image[*a][*b].is_scaffold())
            {
                result.push((x, y));
            }
        }
    }
    result
}

pub fn solve_part_one() -> usize {
    let mut computer = Computer::new(&Computer::load_data(17), &[]);
    let image = get_image(&mut computer);
    print_image(&image);
    let intersections = get_intersections(&image);
    intersections.into_iter().map(|(x, y)| x * y).sum()
}

/*
        (12, 10)
 R12 -> (24, 10)
 L10 -> (24,  0)
 L10 -> (14,  0)
 L 6 -> (14,  6)
 L12 -> (26,  6)
 R12 -> (26, 18)
 L 4 -> (30, 18)
 R12 -> (30, 30)
 L10 -> (40, 30)
 L10 -> (40, 20)
 L 6 -> (34, 20)
 L12 -> (34, 32)
 R12 -> (22, 32)
 L 4 -> (22, 36)
 L12 -> (34, 36)
 R12 -> (34, 48)
 L 6 -> (40, 48)
 L 6 -> (40, 42)
 L12 -> (28, 42)
 R12 -> (28, 30)
 L 4 -> (24, 30)
 L12 -> (24, 42)
 R12 -> (12, 42)
 L 6 -> (12, 48)
 R12 -> ( 0, 48)
 L10 -> ( 0, 58)
 L10 -> (10, 58)
 L12 -> (10, 46)
 R12 -> (22, 46)
 L 6 -> (22, 40)
 L12 -> (10, 40)
 R12 -> (10, 28)
 L 6 -> ( 4, 28)

 R12 L10 L10 L6 L12 R12 L4 R12 L10 L10 L6 L12 R12 L4 L12 R12 L6 L6 L12 R12 L4 L12 R12 L6 R12 L10 L10 L12 R12 L6 L12 R12 L6

 A B A B C B C A C C
 A: R12 L10 L10
 B: L6 L12 R12 L4
 C: L12 R12 L6
*/
pub fn solve_part_two() -> i64 {
    let input: Vec<i64> = concat!(
    "A,B,A,B,C,B,C,A,C,C\n",
    "R,12,L,10,L,10\n",
    "L,6,L,12,R,12,L,4\n",
    "L,12,R,12,L,6\n",
    "n\n"
    )
        .chars()
        .map(|x| x as i64)
        .collect();

    let mut data = Computer::load_data(17);
    data[0] = 2;
    Computer::new(&data, &input).last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_intersections() {
        let pixels: Vec<Vec<Pixel>> = vec![
            vec![
                Pixel::Space,
                Pixel::Scaffold,
                Pixel::Scaffold,
                Pixel::Scaffold,
            ],
            vec![
                Pixel::Scaffold,
                Pixel::Scaffold,
                Pixel::Scaffold,
                Pixel::Scaffold,
            ],
            vec![Pixel::Space, Pixel::Space, Pixel::RobotUp, Pixel::Space],
            vec![Pixel::Scaffold, Pixel::Scaffold, Pixel::Space, Pixel::Space],
        ];
        assert_eq!(get_intersections(&pixels), vec![(1, 2)])
    }

    #[test]
    fn test_wtf() {
        assert_eq!('a' as usize, 97);
        let move_routine = itertools::join("ABABCBCACC".chars(), ",")
            .chars()
            .map(|z| z as i64)
            .collect::<Vec<i64>>();
        assert_eq!(
            move_routine,
            vec![65, 44, 66, 44, 65, 44, 66, 44, 67, 44, 66, 44, 67, 44, 65, 44, 67, 44, 67]
        );
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 8084);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 1119775);
    }
}
