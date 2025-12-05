advent_of_code::solution!(5);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

struct Range {
    start: u64,
    end: u64,
}

fn parse_data(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (fresh_ingredient_ranges_str, available_ingredients_str) =
        input.split_once("\n\n").unwrap();

    let fresh_ingredient_ranges = fresh_ingredient_ranges_str
        .iter_unsigned()
        .chunk::<2>()
        .map(|[start, end]| Range { start, end })
        .collect();

    let available_ingredients = available_ingredients_str.iter_unsigned().collect();

    (fresh_ingredient_ranges, available_ingredients)
}

fn merge_all_ranges(fresh_ingredient_ranges: &mut Vec<Range>) {
    fresh_ingredient_ranges.sort_unstable_by_key(|r| r.start);

    let mut merged: Vec<Range> = Vec::with_capacity(fresh_ingredient_ranges.len());
    merged.push(Range {
        start: fresh_ingredient_ranges[0].start,
        end: fresh_ingredient_ranges[0].end,
    });

    for range in fresh_ingredient_ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if last.end >= range.start {
            last.end = u64::max(last.end, range.end);
        } else {
            merged.push(Range { start: range.start, end: range.end });
        }
    }

    *fresh_ingredient_ranges = merged;
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut fresh_ingredient_ranges, available_ingredients) = parse_data(input);
    merge_all_ranges(&mut fresh_ingredient_ranges);

    let result = available_ingredients
        .into_iter()
        .filter(|&ingredient| {
            let idx = fresh_ingredient_ranges.partition_point(|r| ingredient >= r.start);
            idx > 0 && ingredient <= fresh_ingredient_ranges[idx - 1].end
        })
        .count() as u32;

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
