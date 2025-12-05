advent_of_code::solution!(5);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }

    fn contains(&self, x: &u64) -> bool {
        &self.start <= x && x <= &self.end
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start).then_with(|| self.end.cmp(&other.end))
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_data(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (fresh_ingredient_ranges_str, available_ingredients_str) =
        input.split_once("\n\n").unwrap();

    let fresh_ingredient_ranges = fresh_ingredient_ranges_str
        .iter_unsigned()
        .chunk::<2>()
        .map(|[start, end]| Range::new(start, end))
        .collect();

    let available_ingredients = available_ingredients_str.iter_unsigned().collect();

    (fresh_ingredient_ranges, available_ingredients)
}

fn merge_all_ranges(fresh_ingredient_ranges: &mut Vec<Range>) {
    fresh_ingredient_ranges.sort_unstable();

    for i in (1..fresh_ingredient_ranges.len()).rev() {
        let r1 = fresh_ingredient_ranges[i - 1];
        let r2 = fresh_ingredient_ranges[i];

        if let Some(merged) = merge_two_ranges(r1, r2) {
            fresh_ingredient_ranges.swap_remove(i);
            fresh_ingredient_ranges[i - 1] = merged;
        }
    }
}

fn merge_two_ranges(r1: Range, r2: Range) -> Option<Range> {
    if r1.end < r2.start {
        return None;
    }

    if r2.end < r1.end {
        return Some(r1);
    }

    if r1.end == r2.end && r1.start <= r2.start {
        return Some(r1);
    }

    if r1.start == r2.start && r1.end <= r2.end {
        return Some(r2);
    }

    Some(Range::new(r1.start, r2.end))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut fresh_ingredient_ranges, available_ingredients) = parse_data(input);
    merge_all_ranges(&mut fresh_ingredient_ranges);

    let result = available_ingredients
        .into_iter()
        .filter(|ingredient| fresh_ingredient_ranges.iter().any(|r| r.contains(ingredient)))
        .count() as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut fresh_ingredient_ranges, _) = parse_data(input);
    merge_all_ranges(&mut fresh_ingredient_ranges);

    let result = fresh_ingredient_ranges.into_iter().map(|r| r.end - r.start + 1).sum();

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
        assert_eq!(result, Some(14));
    }
}
