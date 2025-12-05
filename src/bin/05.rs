use std::cmp::{max, min};
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse_input(input);
    Some(
        ingredients
            .iter()
            .filter(|ingredient| {
                ranges
                    .iter()
                    .any(|(a, b)| *ingredient >= a && *ingredient <= b)
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<_> = parse_input(input).0.iter().map(|tup| Some(*tup)).collect();
    for idx in 0..ranges.len() {
        if let Some((mut a, mut b)) = ranges[idx] {
            for idx2 in 0..idx {
                if let Some((c, d)) = ranges[idx2] {
                    if d < a || b < c {
                        continue;
                    } else {
                        a = min(a, c);
                        b = max(b, d);
                        ranges[idx2] = None;
                    }
                }
            }
            ranges[idx] = Some((a, b));
        }
    }
    Some(
        ranges
            .iter()
            .flatten()
            .map(|(a, b)| b - a + 1)
            .sum::<usize>() as u64,
    )
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges: Vec<_> = ranges
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let ingredients: Vec<usize> = ingredients
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
