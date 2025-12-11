use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variables};
use std::cmp::min;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let mut out = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(']').unwrap();
        let target: u64 = a
            .chars()
            .skip(1)
            .enumerate()
            .filter_map(|(idx, c)| {
                if c == '#' {
                    Some(2u64.pow(idx as u32))
                } else {
                    None
                }
            })
            .sum();
        let (c, _) = b.split_once('{').unwrap();
        let buttons: Vec<u64> = c
            .trim()
            .split(' ')
            .map(|button| {
                button
                    .trim_start_matches("(")
                    .trim_end_matches(")")
                    .split(',')
                    .map(|num| 2u64.pow(num.parse().unwrap()))
                    .sum()
            })
            .collect();
        let mut num_presses = buttons.len();
        for i in 0..2u64.pow(buttons.len() as u32) {
            let mut num = 0;
            let mut j = i;
            for &button in &buttons {
                if j.rem_euclid(2) > 0 {
                    num = num ^ button
                }
                j = j / 2
            }
            if num == target {
                num_presses = min(num_presses, i.count_ones() as usize)
            }
        }
        out += num_presses;
    }
    Some(out as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut out = 0;
    for line in input.lines() {
        let (_, b) = line.split_once(']').unwrap();
        let (c, d) = b.split_once('{').unwrap();
        let buttons: Vec<Vec<u64>> = c
            .trim()
            .split(' ')
            .map(|button| {
                button
                    .trim_start_matches("(")
                    .trim_end_matches(")")
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();
        let joltage: Vec<u64> = d
            .trim_end_matches('}')
            .split(',')
            .map(|jolt| jolt.parse().unwrap())
            .collect();
        // set up mixed integer linear programming
        variables! {
        vars:
            0 <= x[buttons.len()] (integer);
        }
        let objective: Expression = x.iter().sum();
        let mut a = vec![vec![0; buttons.len()]; joltage.len()];
        for (i, button) in buttons.iter().enumerate() {
            for j in button {
                a[*j as usize][i] = 1
            }
        }
        let mut constraints = Vec::new();
        for (j, jolt) in joltage.iter().enumerate() {
            let z = a[j]
                .iter()
                .zip(x.iter())
                .map(|(aji, xi)| *aji * *xi)
                .sum::<Expression>();
            constraints.push(constraint!(z == *jolt as u32));
        }
        let solution = vars
            .minimise(objective)
            .using(default_solver)
            .with_all(constraints)
            .solve()
            .ok()?;
        out += solution.eval(x.iter().sum::<Expression>()) as u64;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
