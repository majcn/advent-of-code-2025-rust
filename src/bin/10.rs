advent_of_code::solution!(10);

use std::collections::VecDeque;

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::parse::*;

struct Manual {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

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

fn part_one_manual(manual: &Manual) -> u32 {
    let start = 0;
    let goal = manual
        .indicator_lights
        .iter()
        .enumerate()
        .map(|(i, &on)| if on { 1 << i } else { 0 })
        .sum();

    let button_actions = manual
        .buttons
        .iter()
        .map(|button| button.iter().map(|i| 1 << i).sum::<u32>())
        .collect::<Vec<_>>();

    let mut seen = FastSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((state, cost)) = queue.pop_front() {
        for button_action in &button_actions {
            let new_state = state ^ button_action;

            if seen.contains(&new_state) {
                continue;
            }
            seen.insert(new_state);

            if new_state == goal {
                return cost + 1;
            }

            queue.push_back((new_state, cost + 1));
        }
    }

    unreachable!()
}

fn part_2_manual(manual: &Manual) -> u32 {
    let solver = z3::Optimize::new();
    let zero = z3::ast::Int::new_const("zero");
    solver.assert(&zero.eq(0));

    let button_vars = (0..manual.buttons.len())
        .map(|i| z3::ast::Int::new_const(format!("x_{i}")))
        .collect::<Vec<_>>();

    for v in &button_vars {
        solver.assert(&v.ge(0));
    }

    for i in 0..manual.joltages.len() {
        let sum = manual
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| button.contains(&i))
            .fold(zero.clone(), |acc, (button_i, _)| acc + &button_vars[button_i]);

        solver.assert(&sum.eq(manual.joltages[i]));
    }

    let total = button_vars.iter().fold(zero, |acc, v| acc + v);
    solver.minimize(&total);
    solver.check(&[]);

    solver.get_model().unwrap().eval(&total, false).unwrap().as_u64().unwrap() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data.iter().map(part_one_manual).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data.iter().map(part_2_manual).sum();

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
