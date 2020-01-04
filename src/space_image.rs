struct Dimensions {
    width: usize,
    height: usize,
}

fn count_zeros(chars: &Vec<char>) -> usize {
    chars.into_iter().filter(|&ch| *ch == '0').count()
}

fn layer_with_fewest_zeros(layers: Vec<Vec<char>>) -> Vec<char> {
    layers
        .into_iter()
        .min_by(|a, b| count_zeros(a).cmp(&count_zeros(b)))
        .unwrap()
}

fn to_layers(input: String, dimensions: &Dimensions) -> Vec<Vec<char>> {
    input
        .chars()
        .collect::<Vec<char>>()
        .chunks_exact(dimensions.width * dimensions.height)
        .map(|a| Vec::from(a))
        .collect()
}

fn count_digit(layer: &[char], ch: char) -> usize {
    layer.into_iter().filter(|&&x| x == ch).count()
}

fn create_composite(layers: Vec<Vec<char>>) -> Vec<char> {
    let mut result = vec!['2'; layers[0].len()];
    for j in 0..result.len() {
        result[j] = (0..layers.len())
            .into_iter()
            .map(|i| layers[i][j])
            .filter(|x| *x != '2')
            .nth(0)
            .unwrap_or('2');
    }
    result
}

fn print_image(img: &Vec<char>, dimensions: &Dimensions) -> bool {
    let output = img
        .iter()
        .map(|&ch| match ch {
            '0' | '2' => ' ',
            '1' => 'x',
            _ => panic!("value not allowed in image: {}", ch),
        })
        .collect::<Vec<char>>()
        .chunks_exact(dimensions.width)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>();

    println!("8.2 output: ");
    for line in output {
        println!("{}", line);
    }
    true
}

pub fn solve_part_one() -> usize {
    let input = super::get_input::main(8);
    let layers = to_layers(
        input,
        &Dimensions {
            width: 25,
            height: 6,
        },
    );
    let fewest_zeros = layer_with_fewest_zeros(layers);
    count_digit(&fewest_zeros, '1') * count_digit(&fewest_zeros, '2')
}

pub fn solve_part_two() -> Vec<char> {
    let input = super::get_input::main(8);
    let dimensions = Dimensions {
        width: 25,
        height: 6,
    };
    let layers = to_layers(input, &dimensions);
    let composite = create_composite(layers);
    print_image(&composite, &dimensions);
    composite
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_with_fewest_zeros() {
        assert_eq!(
            layer_with_fewest_zeros(to_layers(
                "123456789012".to_string(),
                &Dimensions {
                    width: 3,
                    height: 2,
                },
            ))
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
            123456,
        );
    }

    #[test]
    fn test_create_composite() {
        assert_eq!(
            create_composite(to_layers(
                "0222112222120000".to_string(),
                &Dimensions {
                    width: 2,
                    height: 2,
                },
            )),
            vec!['0', '1', '1', '0'],
        );
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 1215);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two();
        let expected: Vec<char> = vec![
            '1', '0', '0', '0', '0', '1', '0', '0', '1', '0', '0', '1', '1', '0', '0', '1', '1',
            '1', '0', '0', '1', '0', '0', '1', '0', '1', '0', '0', '0', '0', '1', '0', '0', '1',
            '0', '1', '0', '0', '1', '0', '1', '0', '0', '1', '0', '1', '0', '0', '1', '0', '1',
            '0', '0', '0', '0', '1', '1', '1', '1', '0', '1', '0', '0', '0', '0', '1', '0', '0',
            '1', '0', '1', '1', '1', '1', '0', '1', '0', '0', '0', '0', '1', '0', '0', '1', '0',
            '1', '0', '0', '0', '0', '1', '1', '1', '0', '0', '1', '0', '0', '1', '0', '1', '0',
            '0', '0', '0', '1', '0', '0', '1', '0', '1', '0', '0', '1', '0', '1', '0', '0', '0',
            '0', '1', '0', '0', '1', '0', '1', '1', '1', '1', '0', '1', '0', '0', '1', '0', '0',
            '1', '1', '0', '0', '1', '0', '0', '0', '0', '1', '0', '0', '1', '0',
        ];
        assert_eq!(result, expected);
    }
}
