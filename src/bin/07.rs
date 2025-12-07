advent_of_code::solution!(7);

use advent_of_code_macros::memoize;

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> (Grid<u8>, Point) {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();

    (grid, start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, start) = parse_data(input);
    grid[start] = b'|';

    let mut result = 0;
    for y in 0..grid.height - 1 {
        for x in 0..grid.width {
            let loc = Point::new(x, y);
            if grid[loc] == b'|' {
                if grid[loc + DOWN] == b'^' {
                    result += 1;
                    grid[loc + DOWN + LEFT] = b'|';
                    grid[loc + DOWN + RIGHT] = b'|';
                } else {
                    grid[loc + DOWN] = b'|';
                }
            }
        }
    }

    Some(result)
}

fn solve_cache_key(_: &Grid<u8>, beam_location: Point) -> Point {
    beam_location
}

#[memoize(key_function = "solve_cache_key -> Point")]
fn part_two_recursion(grid: &Grid<u8>, beam_location: Point) -> u64 {
    if beam_location.y == grid.height - 1 {
        return 1;
    }

    let mut result = 0;

    if grid[beam_location + DOWN] == b'^' {
        result += part_two_recursion(grid, beam_location + DOWN + LEFT);
        result += part_two_recursion(grid, beam_location + DOWN + RIGHT);
    } else {
        result += part_two_recursion(grid, beam_location + DOWN);
    }

    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start) = parse_data(input);

    part_two_recursion_reset_memoize();
    let result = part_two_recursion(&grid, start);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(40));
    }
}
