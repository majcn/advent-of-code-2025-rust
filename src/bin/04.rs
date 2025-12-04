advent_of_code::solution!(4);

use advent_of_code::majcn::grid::*;
use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn part_x(grid: &mut Grid<u8>, remove_after_find: bool) -> u32 {
    let mut result = 0;

    for point in grid.points() {
        if grid[point] == b'@' {
            let valid_neighburs = DIAGONAL
                .map(|d| d + point)
                .into_iter()
                .filter(|&n| grid.contains(n) && grid[n] == b'@')
                .count();

            if valid_neighburs < 4 {
                result += 1;
                if remove_after_find {
                    grid[point] = b'.';
                }
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_data(input);

    let result = part_x(&mut grid, false);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_data(input);

    let result = std::iter::from_fn(move || {
        let res = part_x(&mut grid, true);
        (res > 0).then_some(res)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(43));
    }
}
