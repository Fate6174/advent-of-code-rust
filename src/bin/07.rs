use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, _, out) = fill_manifold(input);
    Some(out as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (manifold, beam_starts, _) = fill_manifold(input);
    let (m, n) = (manifold.len(), manifold[0].len());
    let mut cache = HashMap::new();
    for j in 0..n {
        if manifold[m - 1][j] == '|' {
            cache.insert((m - 1, j), 1 as u64);
        }
    }
    for i in (1..m - 1).rev() {
        for j in 0..n {
            if manifold[i][j] == '|' {
                match manifold[i + 1][j] {
                    '|' => {
                        cache.insert((i, j), cache.get(&(i + 1, j)).copied().unwrap());
                    }
                    '^' => {
                        let mut value = 0;
                        if j > 0 {
                            value += cache.get(&(i + 1, j - 1)).copied().unwrap();
                        }
                        if j < n - 1 {
                            value += cache.get(&(i + 1, j + 1)).copied().unwrap();
                        }
                        cache.insert((i, j), value);
                    }
                    _ => panic!(),
                }
            }
        }
    }
    let mut out = 0;
    for (i, j) in beam_starts {
        out += cache.get(&(i + 1, j)).copied().unwrap();
    }
    Some(out)
}

fn fill_manifold(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>, u64) {
    let mut manifold = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (m, n) = (manifold.len(), manifold[0].len());
    let beam_starts = manifold
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, &item)| {
                    if item == 'S' { Some((i, j)) } else { None }
                },
            )
        })
        .flatten()
        .collect::<Vec<_>>();
    let mut beam_positions = beam_starts.clone();
    let mut out = 0;
    while !beam_positions.is_empty() {
        let (a, b) = beam_positions.pop().unwrap();
        if a == m - 1 {
            continue;
        }
        match manifold[a + 1][b] {
            '.' => {
                manifold[a + 1][b] = '|';
                beam_positions.push((a + 1, b))
            }
            '^' => {
                out += 1;
                if b > 0 && manifold[a + 1][b - 1] == '.' {
                    beam_positions.push((a, b - 1))
                }
                if b < n - 1 && manifold[a + 1][b + 1] == '.' {
                    beam_positions.push((a, b + 1))
                }
            }
            _ => continue,
        }
    }
    (manifold, beam_starts, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
