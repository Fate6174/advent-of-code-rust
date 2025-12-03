advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum_invalid_ids = 0;
    let id_ranges = input.split(',');
    for id_range in id_ranges {
        let (start, stop) = id_range.split_once('-').expect("Failed to parse id_range.");
        let start: u64 = start.trim().parse().expect("Failed to parse id");
        let stop: u64 = stop.trim().parse().expect("Failed to parse id");
        for num in start..=stop {
            let num_str = num.to_string();
            let num_str_len = num_str.len();
            if num_str_len.rem_euclid(2) == 0 {
                let half_length = num_str_len / 2;
                let (first, second) = num_str.split_at(half_length);
                if first == second {
                    sum_invalid_ids += num;
                }
            }
        }
    }
    println!("{sum_invalid_ids}");
    Some(sum_invalid_ids)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum_invalid_ids = 0;
    let id_ranges = input.split(',');
    for id_range in id_ranges {
        let (start, stop) = id_range.split_once('-').expect("Failed to parse id_range.");
        let start: u64 = start.trim().parse().expect("Failed to parse id");
        let stop: u64 = stop.trim().parse().expect("Failed to parse id");
        for num in start..=stop {
            let num_str = num.to_string();
            let num_str_len = num_str.len();
            for num_parts in 2..=num_str_len {
                if num_str_len.rem_euclid(num_parts) == 0 {
                    let part_length = num_str_len / num_parts;
                    let (first, rest) = num_str.split_at(part_length);
                    if (0..num_parts - 1)
                        .all(|idx| *first == rest[idx * part_length..(idx + 1) * part_length])
                    {
                        sum_invalid_ids += num;
                        break;
                    }
                }
            }
        }
    }
    println!("{sum_invalid_ids}");
    Some(sum_invalid_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
