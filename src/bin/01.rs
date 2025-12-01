advent_of_code::solution!(1);

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<i32> {
    input.replace('L', "-").iter_signed().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let (mut pos, mut result) = (50, 0);
    for v in data {
        pos += v;
        result += u32::from(pos % 100 == 0)
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let (mut pos, mut result) = (50, 0);
    for v in data {
        let tmp = pos + v;
        result += tmp.unsigned_abs() / 100 + u32::from(tmp <= 0 && pos > 0);
        pos = tmp.rem_euclid(100);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(6));
    }
}
