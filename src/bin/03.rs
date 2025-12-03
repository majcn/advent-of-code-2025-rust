advent_of_code::solution!(3);

fn parse_data(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().map(|x| x - b'0').collect()).collect()
}

fn part_x<const COUNT: usize>(bank: &[u8]) -> u64 {
    let mut result = 0;
    let mut index = 0;
    for c in (0..COUNT).rev() {
        let (max_i, &max_v) = bank[index..bank.len() - c]
            .iter()
            .enumerate()
            .max_by_key(|&(i, v)| (v, usize::MAX - i))
            .unwrap();

        index += max_i + 1;
        result = result * 10 + max_v as u64;
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = data.iter().map(|bank| part_x::<2>(bank)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = data.iter().map(|bank| part_x::<12>(bank)).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(3121910778619));
    }
}
