advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial_state = 50;
    let mut num_at_zero = 0;
    for rotation in input.lines() {
        // parse rotation
        let (direction, distance) = rotation
            .split_at_checked(1)
            .expect("Failed to parse direction.");
        let distance: i64 = distance
            .parse()
            .expect("Distance must be an integer value.");
        assert!(direction == "L" || direction == "R");
        let distance = if direction == "L" {
            -distance
        } else {
            distance
        };
        let old_dial_state = dial_state;
        dial_state = (dial_state + distance) % 100;
        let ended_at_zero_str = if dial_state == 0 {
            num_at_zero += 1;
            " Ended at zero."
        } else {
            ""
        };
        println!("{old_dial_state} rotated by {rotation} -> {dial_state}.{ended_at_zero_str}");
    }
    println!("The dial was {num_at_zero} times at the position 0 at the end of an rotation.");
    Some(num_at_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial_state = 50;
    let mut num_at_zero = 0;
    for rotation in input.lines() {
        // parse rotation
        let (direction, distance) = rotation
            .split_at_checked(1)
            .expect("Failed to parse direction.");
        let distance: i64 = distance
            .parse()
            .expect("Distance must be an integer value.");
        assert!(direction == "L" || direction == "R");
        let distance = if direction == "L" {
            -distance
        } else {
            distance
        };
        let old_dial_state = dial_state;
        let old_num_at_zero = num_at_zero;

        dial_state = (old_dial_state + distance).rem_euclid(100);
        let total_revolutions = distance.abs() as u64 / 100;
        num_at_zero += total_revolutions;
        if direction == "R" && dial_state < old_dial_state
            || direction == "L" && old_dial_state != 0 && dial_state > old_dial_state
            || direction == "L" && dial_state == 0
        {
            num_at_zero += 1;
        }
        let traverse_zero_str = if num_at_zero > old_num_at_zero {
            let traversals = num_at_zero - old_num_at_zero;
            if traversals == 1 {
                " Traversed zero once.".to_string()
            } else {
                format!(" Traversed zero {traversals} times.")
            }
        } else {
            "".to_string()
        };
        println!("{old_dial_state} rotated by {rotation} -> {dial_state}.{traverse_zero_str}");
    }
    println!(
        "The dial was {num_at_zero} times at the position 0 during or at the end of an rotation."
    );
    Some(num_at_zero)
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
        assert_eq!(result, Some(6));
    }
}
