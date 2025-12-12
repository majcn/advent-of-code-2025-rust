advent_of_code::solution!(11);

use advent_of_code_macros::memoize;

use advent_of_code::maneatingape::hash::*;

fn parse_data(input: &str) -> (Vec<Vec<usize>>, FastMap<String, usize>) {
    let result_with_str = input
        .lines()
        .map(|line| line.split(' ').map(|s| &s[..3]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut mapper = FastMap::with_capacity(result_with_str.len() + 1);
    for (i, line) in result_with_str.iter().enumerate() {
        mapper.insert(String::from(line[0]), i);
    }
    mapper.insert(String::from("out"), result_with_str.len());

    let mut result = vec![vec![]; mapper.len()];
    for line in result_with_str {
        let key = line[0];
        for s in &line[1..] {
            result[mapper[key]].push(mapper[*s]);
        }
    }

    (result, mapper)
}

struct PartXData {
    data: Vec<Vec<usize>>,
    goal: usize,
    path_validator: fn(bool, bool) -> bool,
    dac: usize,
    fft: usize,
}

#[derive(Eq, PartialEq, Hash)]
struct SolveCacheKey {
    pos: usize,
    has_dac: bool,
    has_fft: bool,
}

fn solve_cache_key(_: &PartXData, pos: usize, has_dac: bool, has_fft: bool) -> SolveCacheKey {
    SolveCacheKey { pos, has_dac, has_fft }
}

#[memoize(key_function = "solve_cache_key -> SolveCacheKey")]
fn part_x(data: &PartXData, pos: usize, has_dac: bool, has_fft: bool) -> u64 {
    if pos == data.goal {
        return if (data.path_validator)(has_dac, has_fft) { 1 } else { 0 };
    }

    data.data[pos]
        .iter()
        .map(|&next_pos| {
            if next_pos == data.dac {
                part_x(data, next_pos, true, has_fft)
            } else if next_pos == data.fft {
                part_x(data, next_pos, has_dac, true)
            } else {
                part_x(data, next_pos, has_dac, has_fft)
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (data, mapper) = parse_data(input);

    let start = mapper["you"];
    let goal = mapper["out"];
    let path_validator = |_, _| true;
    let dac = 0;
    let fft = 0;

    part_x_reset_memoize();
    let result = part_x(&PartXData { data, goal, path_validator, dac, fft }, start, false, false);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (data, mapper) = parse_data(input);

    let start = mapper["svr"];
    let goal = mapper["out"];
    let path_validator = |has_dac, has_fft| has_dac && has_fft;
    let dac = mapper["dac"];
    let fft = mapper["fft"];

    part_x_reset_memoize();
    let result = part_x(&PartXData { data, goal, path_validator, dac, fft }, start, false, false);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(&input);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, Some(2));
    }
}
