advent_of_code::solution!(12);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

struct Shape {
    size: usize,
}

struct Instruction {
    width: usize,
    height: usize,
    pieces: [usize; 6],
}

fn parse_data(input: &str) -> (Vec<Shape>, Vec<Instruction>) {
    let mut input_split = input.split("\n\n").collect::<Vec<_>>();
    let instructions_str = input_split.pop().unwrap();
    let shapes_str = input_split;

    let instructions = instructions_str
        .iter_unsigned()
        .chunk::<8>()
        .map(|[w, h, pieces @ ..]| Instruction { width: w, height: h, pieces })
        .collect();

    let shapes = shapes_str
        .iter()
        .map(|s| Shape { size: s.bytes().filter(|&c| c == b'#').count() })
        .collect();

    (shapes, instructions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (shapes, instructions) = parse_data(input);

    let result = instructions
        .into_iter()
        .filter(|instruction| {
            let n = instruction
                .pieces
                .iter()
                .enumerate()
                .fold(0, |acc, (i, p)| acc + p * shapes[i].size);
            n <= instruction.width * instruction.height
        })
        .count() as u32;

    Some(result)
}

pub fn part_two(_input: &str) -> Option<String> {
    // "Thank you Eric for another wonderful year of AoC!"
    Some(String::from("⭐️⭐️"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(String::from("⭐️⭐️")));
    }
}
