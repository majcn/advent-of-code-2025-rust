advent_of_code::solution!(8);

use advent_of_code::maneatingape::heap::*;
use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

struct Location {
    x: u64,
    y: u64,
    z: u64,
}

enum Day8Result {
    Part1(Vec<Vec<usize>>),
    Part2(usize, usize),
}

fn parse_data(input: &str) -> (Vec<Location>, usize) {
    const DEFAULT_LIMIT: usize = 1000;

    let default_right = format!("{DEFAULT_LIMIT}",);
    let (left, right) = input.split_once("\n\n").unwrap_or((input, &default_right));

    (
        left.iter_unsigned().chunk::<3>().map(|[x, y, z]| Location { x, y, z }).collect(),
        right.unsigned(),
    )
}

fn part_x(data: &[Location], limit: usize) -> Day8Result {
    let mut groups = MinHeap::with_capacity(data.len() * (data.len() - 1) / 2);
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let dx = u64::abs_diff(data[i].x, data[j].x);
            let dy = u64::abs_diff(data[i].y, data[j].y);
            let dz = u64::abs_diff(data[i].z, data[j].z);
            groups.push(dx * dx + dy * dy + dz * dz, (i, j));
        }
    }

    let generate_new_group = |l1, l2| {
        let mut new_group = Vec::with_capacity(data.len());
        new_group.push(l1);
        new_group.push(l2);
        new_group
    };

    let mut final_groups: Vec<Vec<usize>> = Vec::with_capacity(data.len());
    for _ in 0..limit {
        let (_, (loc1, loc2)) = groups.pop().unwrap();
        let loc1_group = final_groups.iter().position(|g| g.contains(&loc1));
        let loc2_group = final_groups.iter().position(|g| g.contains(&loc2));

        match (loc1_group, loc2_group) {
            (Some(i), None) => final_groups[i].push(loc2),
            (None, Some(j)) => final_groups[j].push(loc1),
            (None, None) => final_groups.push(generate_new_group(loc1, loc2)),
            (Some(i), Some(j)) if i != j => {
                let (min_i, max_i) = if i < j { (i, j) } else { (j, i) };
                let values = final_groups.swap_remove(max_i);
                final_groups[min_i].extend(values);
            }
            _ => { /* do nothing */ }
        }

        if final_groups.len() == 1 && final_groups[0].len() == data.len() {
            return Day8Result::Part2(loc1, loc2);
        }
    }

    Day8Result::Part1(final_groups)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (data, limit) = parse_data(input);

    let Day8Result::Part1(mut final_groups) = part_x(&data, limit) else { unreachable!() };
    final_groups.sort_unstable_by_key(|x| x.len());
    let result = final_groups.iter().rev().take(3).map(|x| x.len() as u64).product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (data, _) = parse_data(input);

    let Day8Result::Part2(loc1, loc2) = part_x(&data, usize::MAX) else { unreachable!() };
    let result = data[loc1].x * data[loc2].x;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(25272));
    }
}
