use num::complex::Complex;
use std::collections::HashSet;

const OFFSETS: [Complex<isize>; 8] = [
    Complex::new(1, 0),
    Complex::new(-1, 0),
    Complex::new(0, 1),
    Complex::new(0, -1),
    Complex::new(1, 1),
    Complex::new(1, -1),
    Complex::new(-1, 1),
    Complex::new(-1, -1),
];

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid_set = parse_grid(input);
    Some(find_removables(&grid_set).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid_set = parse_grid(input);
    let mut out = 0;
    loop {
        let removables = find_removables(&grid_set);
        if removables.is_empty() {
            break;
        }
        for coordinate in &removables {
            grid_set.remove(coordinate);
        }
        out += removables.len();
    }
    Some(out as u64)
}

fn parse_grid(input: &str) -> HashSet<Complex<isize>> {
    let mut out = HashSet::new();
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            if char == '@' {
                out.insert(Complex::new(row_idx as isize, col_idx as isize));
            }
        }
    }
    out
}

fn find_removables(grid_set: &HashSet<Complex<isize>>) -> Vec<Complex<isize>> {
    let mut out = vec![];
    for coordinate in grid_set {
        let mut adjacent_rolls = 0;
        for dz in OFFSETS {
            if grid_set.contains(&(coordinate + dz)) {
                adjacent_rolls += 1
            }
        }
        if adjacent_rolls < 4 {
            out.push(*coordinate);
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
