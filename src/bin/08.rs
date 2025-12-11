use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let num_connections = input.lines().next().unwrap().parse().ok();
    let (circuit_id_to_junkboxes, _) = parse_input(input, num_connections);
    let mut circuits = circuit_id_to_junkboxes
        .values()
        .map(|s| s.len() as u64)
        .collect::<Vec<_>>();
    circuits.sort();
    let out = circuits.iter().rev().take(3).product();
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, out) = parse_input(input, None);
    Some(out)
}

fn parse_input(input: &str, num_connections: Option<usize>) -> (HashMap<usize, Vec<usize>>, u64) {
    let parsed = input
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|word| word.parse::<i64>().expect("Failed to parse coordinates."))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut distances = parsed
        .iter()
        .enumerate()
        .map(|(idx1, coord1)| {
            parsed
                .iter()
                .enumerate()
                .skip(idx1 + 1)
                .map(move |(idx2, coord2)| {
                    (
                        (coord1[0] - coord2[0]).pow(2)
                            + (coord1[1] - coord2[1]).pow(2)
                            + (coord1[2] - coord2[2]).pow(2),
                        idx1,
                        idx2,
                    )
                })
        })
        .flatten()
        .collect::<Vec<_>>();
    distances.sort_by_key(|tuple| tuple.0);
    // let mut junkbox_to_circuit_id = HashMap::new();
    let mut junkbox_to_circuit_id: HashMap<usize, usize> =
        HashMap::from_iter((0..parsed.len()).map(|idx| (idx, idx)));
    let mut circuit_id_to_junkboxes =
        HashMap::from_iter((0..parsed.len()).map(|idx| (idx, vec![idx])));
    let mut out = 0;
    for &(_, idx1, idx2) in &distances[0..num_connections.unwrap_or(distances.len())] {
        let circuit_id1 = junkbox_to_circuit_id[&idx1];
        let circuit_id2 = junkbox_to_circuit_id[&idx2];
        if circuit_id1 == circuit_id2 {
            continue;
        }
        let mut circuit2 = circuit_id_to_junkboxes
            .remove(&circuit_id2)
            .unwrap_or(vec![]);
        circuit_id_to_junkboxes
            .get_mut(&circuit_id1)
            .unwrap()
            .append(&mut circuit2);
        for &junkbox in &circuit_id_to_junkboxes[&circuit_id1] {
            junkbox_to_circuit_id.insert(junkbox, circuit_id1);
        }
        if circuit_id_to_junkboxes.keys().len() == 1 {
            out = parsed[idx1][0] * parsed[idx2][0]
        }
    }
    (circuit_id_to_junkboxes, out as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
