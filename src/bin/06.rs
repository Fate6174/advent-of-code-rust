advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .rev()
        .collect::<Vec<_>>();
    let mut out = 0;
    for i in 0..parsed[0].len() {
        let operator = parsed[0][i];
        if operator == "+" {
            let mut result = 0;
            for j in 1..parsed.len() {
                result += parsed[j][i]
                    .parse::<u64>()
                    .expect("Failed to parse number.");
            }
            out += result;
        } else {
            let mut result = 1;
            for j in 1..parsed.len() {
                result *= parsed[j][i]
                    .parse::<u64>()
                    .expect("Failed to parse number.");
            }
            out += result;
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let num_rows = parsed.len();
    let mut temp = Vec::new();
    let mut out = 0;
    for i in (0..parsed[0].len()).rev() {
        let mut num = 0;
        for j in 0..num_rows - 1 {
            if parsed[j][i] == b' ' {
                continue;
            }
            num = 10 * num + (parsed[j][i] - b'0') as u64;
        }
        if num > 0 {
            temp.push(num)
        }
        if parsed[num_rows - 1][i] != b' ' {
            let mut result: u64;
            if parsed[num_rows - 1][i] == b'+' {
                result = 0;
                for item in &temp {
                    result += item
                }
            } else {
                result = 1;
                for item in &temp {
                    result *= item
                }
            }
            out += result;
            temp = Vec::new();
        }
    }
    Some(out as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
