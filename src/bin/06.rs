advent_of_code::solution!(6);

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> (Vec<&str>, Vec<u8>) {
    let mut lines = input.lines().collect::<Vec<_>>();
    let operators = lines.pop().unwrap().bytes().filter(|x| !x.is_ascii_whitespace()).collect();

    (lines, operators)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (lines, operators) = parse_data(input);

    let numbers = lines
        .into_iter()
        .map(|line| line.iter_unsigned::<u64>().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result = operators
        .into_iter()
        .enumerate()
        .map(|(i, c)| match c {
            b'+' => numbers.iter().fold(0, |acc, n| acc + n[i]),
            b'*' => numbers.iter().fold(1, |acc, n| acc * n[i]),
            _ => unreachable!(),
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (lines, operators) = parse_data(input);

    let lines = lines.into_iter().map(|line| line.bytes().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut numbers = vec![];
    let mut column_i = 0;
    let columns_len = lines[0].len();

    let mut result = 0;

    for operator in operators {
        while column_i < columns_len {
            let number = lines
                .iter()
                .map(|line| line[column_i])
                .filter(|x| x.is_ascii_alphanumeric())
                .fold(0, |acc, d| acc * 10 + (d - b'0') as u64);
            column_i += 1;

            if number != 0 {
                numbers.push(number);
            } else {
                break;
            }
        }

        result += match operator {
            b'+' => numbers.drain(..).sum::<u64>(),
            b'*' => numbers.drain(..).product::<u64>(),
            _ => unreachable!(),
        };
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(3263827));
    }
}
