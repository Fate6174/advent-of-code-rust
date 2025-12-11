use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut parsed: HashMap<&str, Vec<_>> = HashMap::new();
    for line in input.lines() {
        let mut words = line.split(' ');
        let word = words.next().unwrap();
        parsed.insert(word.trim_end_matches(':'), words.collect());
    }
    let out = count_paths(&parsed, "you", "out", "", &mut HashMap::new());
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut machines: HashMap<&str, Vec<_>> = HashMap::new();
    for line in input.lines() {
        let mut words = line.split(' ');
        let word = words.next().unwrap();
        machines.insert(word.trim_end_matches(':'), words.collect());
    }
    let mut num_pathes = HashMap::new();
    let svr_to_dac = count_paths(&machines, "svr", "dac", "fft", &mut num_pathes);
    let dac_to_fft = count_paths(&machines, "dac", "fft", "", &mut num_pathes);
    let fft_to_out = count_paths(&machines, "fft", "out", "", &mut num_pathes);
    let svr_to_fft = count_paths(&machines, "svr", "fft", "dac", &mut num_pathes);
    let fft_to_dac = count_paths(&machines, "fft", "dac", "", &mut num_pathes);
    let dac_to_out = count_paths(&machines, "dac", "out", "", &mut num_pathes);
    Some(svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out)
}

fn count_paths<'a>(
    machines: &'a HashMap<&str, Vec<&str>>,
    start: &'a str,
    end: &'a str,
    avoid: &'a str,
    num_pathes: &mut HashMap<(&'a str, &'a str, &'a str), u64>,
) -> u64 {
    if num_pathes.contains_key(&(start, end, avoid)) {
        return num_pathes[&(start, end, avoid)];
    } else {
        if start == end {
            num_pathes.insert((start, end, avoid), 1);
            return 1;
        } else if start == avoid || !machines.contains_key(start) {
            num_pathes.insert((start, end, avoid), 0);
            return 0;
        } else {
            let mut out = 0;
            for &next in &machines[start] {
                out += count_paths(&machines, next, end, avoid, num_pathes)
            }
            num_pathes.insert((start, end, avoid), out);
            return out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
