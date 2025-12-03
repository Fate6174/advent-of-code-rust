advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|bank| compute_batteries(bank, 2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|bank| compute_batteries(bank, 12)).sum())
}

pub fn compute_batteries(bank: &str, num_batteries: usize) -> u64 {
    let mut out = 0;
    let bank: Vec<u64> = bank.as_bytes().iter().map(|b| (b - b'0') as u64).collect();
    let bank_length = bank.len();
    let mut current_idx = 0;
    for i in 0..num_batteries {
        let mut max_jolt = bank[current_idx];
        for (idx, jolt) in
            (current_idx + 1..).zip(&bank[current_idx + 1..bank_length - num_batteries + 1 + i])
        {
            if *jolt > max_jolt {
                current_idx = idx;
                max_jolt = *jolt
            }
        }
        out = 10 * out + max_jolt;
        current_idx += 1
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
