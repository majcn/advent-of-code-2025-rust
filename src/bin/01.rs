advent_of_code::solution!(1);

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<i32> {
    input.replace('L', "-").iter_signed().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .scan(50, |pos, v| {
            let new_pos = *pos + v;
            *pos = new_pos;

            if new_pos % 100 == 0 { Some(1) } else { Some(0) }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .scan(50, |pos, v| {
            let curr_pos = *pos;
            let new_pos = curr_pos + v;
            *pos = new_pos.rem_euclid(100);

            if new_pos <= 0 && curr_pos > 0 {
                Some(new_pos.unsigned_abs() / 100 + 1)
            } else {
                Some(new_pos.unsigned_abs() / 100)
            }
        })
        .sum();

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
