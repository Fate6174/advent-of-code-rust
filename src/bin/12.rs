advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    // solves the private puzzle input, but not the public example
    let sizes = input
        .split("\n\n")
        .filter(|present| !present.contains("x"))
        .map(|present| present.as_bytes().iter().filter(|b| **b == b'#').count())
        .collect::<Vec<_>>();
    let areas = input.split("\n\n").last().unwrap().lines().map(|line| {
        line.split(":")
            .next()
            .unwrap()
            .split("x")
            .map(|num| num.parse::<usize>().unwrap())
            .product::<usize>()
    });
    let requirements: Vec<Vec<usize>> = input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    Some(
        areas
            .zip(requirements)
            .filter(|(area, requirement)| {
                requirement
                    .iter()
                    .enumerate()
                    .map(|(idx, num)| num * sizes[idx])
                    .sum::<usize>()
                    < *area
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
