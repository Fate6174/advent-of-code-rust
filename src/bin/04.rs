use ndarray::Array2;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    Some(find_removables(&grid).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut out = 0;
    loop {
        let removables = find_removables(&grid);
        if removables.is_empty() {
            break;
        }
        for (i, j) in &removables {
            grid[[*i, *j]] = '.';
        }
        out += removables.len();
    }
    Some(out as u64)
}

fn parse_grid(input: &str) -> Array2<char> {
    let rows: Vec<&str> = input.lines().collect();
    let m = rows.len();
    let n = rows[0].len();
    let flat: Vec<char> = rows.iter().flat_map(|row| row.chars()).collect();
    Array2::from_shape_vec((m, n), flat).expect("grid is not rectangular")
}

fn find_removables(grid: &Array2<char>) -> Vec<(usize, usize)> {
    let (m, n) = grid.dim();
    let mut out = vec![];
    for row_idx in 0..m {
        for col_idx in 0..n {
            if grid[[row_idx, col_idx]] == '@' {
                // count adjacent paper rolls (@)
                let mut adjacent_rolls = 0;
                for row_idx_temp in [row_idx.wrapping_sub(1), row_idx, row_idx + 1] {
                    if row_idx_temp == usize::MAX || row_idx_temp == m {
                        continue;
                    }
                    for col_idx_temp in [col_idx.wrapping_sub(1), col_idx, col_idx + 1] {
                        if col_idx_temp == usize::MAX || col_idx_temp == n {
                            continue;
                        }
                        if (row_idx_temp, col_idx_temp) != (row_idx, col_idx)
                            && grid[[row_idx_temp, col_idx_temp]] == '@'
                        {
                            adjacent_rolls += 1
                        }
                    }
                }
                if adjacent_rolls < 4 {
                    out.push((row_idx, col_idx));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
