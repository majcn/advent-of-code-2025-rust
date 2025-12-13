advent_of_code::solution!(10);

use advent_of_code_macros::memoize;

use advent_of_code::maneatingape::parse::*;

struct Manual {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

type Target = [u32; 10];

fn parse_data(input: &str) -> Vec<Manual> {
    input
        .lines()
        .map(|line| {
            let (indicator_lights_str, other) = line.split_once(']').unwrap();
            let (buttons_str, joltages_str) = other.split_once('{').unwrap();

            let indicator_lights = indicator_lights_str[1..].bytes().map(|c| c == b'#').collect();

            let buttons = buttons_str
                .split(")")
                .map(|s| s.iter_unsigned().collect::<Vec<_>>())
                .filter(|v| !v.is_empty())
                .collect();

            let joltages = joltages_str.iter_unsigned().collect();

            Manual { indicator_lights, buttons, joltages }
        })
        .collect()
}

fn combinations_cache_key(_: &[u32], target: u32) -> u32 {
    target
}

/// Constructs all combinations of pressing a button 1 or 0 times to achieve the given
/// target mask, with toggling (i.e. pressing twice turns the state back off).
///
/// Uses Gray Code iteration order so we only need a single update per iteration
/// rather than reconstructing the whole result pattern each time.
#[memoize(key_function = "combinations_cache_key -> u32")]
fn combinations(button_masks: &[u32], target: u32) -> Vec<u32> {
    let end = 1u32 << button_masks.len();
    let mut c = 0u32;
    let mut pat = 0u32;

    let mut result = Vec::with_capacity(end as usize);
    if target == 0 {
        result.push(0);
    }

    for i in 1..end {
        let i_zeros = i.trailing_zeros();
        c ^= 1 << i_zeros;
        pat ^= button_masks[i_zeros as usize];
        if pat == target {
            result.push(c);
        }
    }

    result.sort_unstable_by_key(|x| x.count_ones());
    result
}

fn find_minimum_presses_cache_key(_: &[u32], _: &[u32], target: Target) -> Target {
    target
}

/// Returns the minimum number of button presses needed to achieve the specified target
/// values by pressing any combination of the given buttons.
///
/// Based on the algorithm proposed [here](https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/).
///
/// # Arguments
///
/// * `button_masks` - A bitmask per button, where the i-th bit indicates whether the button increases the i-th value.
/// * `joltages_masks` - For each joltage index, a bitmask where bit i is set if button i affects that joltage.
/// * `target` - The target values to achieve.
///
/// # Returns
///
/// The minimum number of button presses needed to achieve the given pattern,
/// or `u32::MAX` if no solution exists.
#[memoize(key_function = "find_minimum_presses_cache_key -> Target")]
fn find_minimum_presses(button_masks: &[u32], joltages_masks: &[u32], target: Target) -> u32 {
    let par = target.iter().enumerate().fold(0, |acc, (i, &v)| acc | (v & 1) << i);
    let combos = combinations(button_masks, par);

    let mut result = u32::MAX;
    let mut remaining = Target::default();

    'combos: for c in combos {
        let press_count = c.count_ones();
        if press_count >= result {
            break;
        }

        let mut remaining_or = 0;
        for j in 0..joltages_masks.len() {
            let presses = (c & joltages_masks[j]).count_ones();
            if presses > target[j] {
                continue 'combos;
            }
            remaining[j] = (target[j] - presses) / 2;
            remaining_or |= remaining[j];
        }

        let additional = match remaining_or {
            0 => 0,
            _ => find_minimum_presses(button_masks, joltages_masks, remaining),
        };

        if additional < u32::MAX {
            let total = press_count + 2 * additional;
            if total < result {
                result = total;
            }
        }
    }

    result
}

fn part_one_manual(manual: &Manual) -> u32 {
    let button_masks = manual
        .buttons
        .iter()
        .map(|button| button.iter().fold(0, |mask, i| mask | (1 << i)))
        .collect::<Vec<_>>();

    let target = manual
        .indicator_lights
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &on)| acc | if on { 1 << i } else { 0 });

    combinations_reset_memoize();
    combinations(&button_masks, target).into_iter().map(|c| c.count_ones()).min().unwrap()
}

fn part_two_manual(manual: &Manual) -> u32 {
    let button_masks = manual
        .buttons
        .iter()
        .map(|button| button.iter().fold(0, |mask, i| mask | (1 << i)))
        .collect::<Vec<_>>();

    let mut joltages_masks = vec![0; manual.joltages.len()];
    for (i, button) in manual.buttons.iter().enumerate() {
        for &j in button {
            joltages_masks[j] |= 1 << i;
        }
    }

    let mut target = Target::default();
    target[..manual.joltages.len()].copy_from_slice(&manual.joltages);

    combinations_reset_memoize();
    find_minimum_presses_reset_memoize();
    find_minimum_presses(&button_masks, &joltages_masks, target)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data.iter().map(part_one_manual).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data.iter().map(part_two_manual).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(33));
    }
}
