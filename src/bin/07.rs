advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, _, splits) = fill_manifold(input);
    Some(splits as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (manifold, beam_starts, _) = fill_manifold(input);
    let (m, n) = (manifold.len(), manifold[0].len());
    let mut cache = vec![vec![1; n]; m];
    for i in (1..m - 1).rev() {
        for j in 0..n {
            match manifold[i + 1][j] {
                '|' => {
                    cache[i][j] = cache[i + 1][j];
                }
                '^' => {
                    let mut value = 0;
                    value += cache[i + 1].get(j - 1).copied().unwrap_or(0);
                    value += cache[i + 1].get(j + 1).copied().unwrap_or(0);
                    cache[i][j] = value;
                }
                _ => {}
            }
        }
    }
    Some(beam_starts.iter().map(|&(i, j)| cache[i + 1][j]).sum())
}

fn fill_manifold(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>, u64) {
    let mut manifold = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let beam_starts = manifold
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, &item)| {
                    if item == 'S' { Some((i, j)) } else { None }
                },
            )
        })
        .collect::<Vec<_>>();
    let mut beam_positions = beam_starts.clone();
    let mut splits = 0;
    loop {
        let Some((a, b)) = beam_positions.pop() else {
            break;
        };
        match manifold.get(a + 1).and_then(|row| row.get(b)) {
            None => {}
            Some('.') => {
                manifold[a + 1][b] = '|';
                beam_positions.push((a + 1, b))
            }
            Some('^') => {
                splits += 1;
                beam_positions.push((a, b - 1));
                beam_positions.push((a, b + 1));
            }
            _ => {}
        }
    }
    (manifold, beam_starts, splits)
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
