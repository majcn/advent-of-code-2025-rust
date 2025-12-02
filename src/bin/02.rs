advent_of_code::solution!(2);

use advent_of_code::majcn::math::*;
use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<[u64; 2]> {
    input.iter_unsigned().chunk::<2>().collect()
}

fn generate_invalid_ids<const LIMIT: usize>(min_id: u64, max_id: u64) -> Vec<u64> {
    let mut result = vec![];

    let min_id_len = min_id.count_digits();
    let max_id_len = max_id.count_digits();

    for repeat in 2..=usize::min(max_id_len, LIMIT) {
        let min_i = (min_id_len / repeat) as u64;
        let max_i = u64::pow(10, (max_id_len / repeat) as u32);

        for i in min_i..max_i {
            let i_len = i.count_digits();

            let mut x = 0;
            for _ in 0..repeat {
                x = x * u64::pow(10, i_len as u32) + i;
            }

            if x >= min_id && x <= max_id {
                result.push(x);
            }
        }
    }

    result
}

fn part_x<const LIMIT: usize>(data: Vec<[u64; 2]>) -> u64 {
    let mut result = data
        .into_iter()
        .flat_map(|[min_id, max_id]| generate_invalid_ids::<LIMIT>(min_id, max_id))
        .collect::<Vec<_>>();

    result.sort_unstable();
    result.dedup();

    result.into_iter().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<2>(data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<20>(data);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(4174379265));
    }
}
