use std::cmp::{max, min};

use num::abs;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();
    let mut out = 0;
    for (idx, (a, b)) in parsed.iter().enumerate() {
        for (c, d) in parsed[idx + 1..].iter() {
            out = max(out, (abs(a - c) + 1) * (abs(b - d) + 1))
        }
    }
    Some(out as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();
    let mut out = 0;
    for (idx, (a, b)) in parsed.iter().enumerate() {
        for (c, d) in parsed[idx + 1..].iter() {
            // rectangle A = [a,b] -> [c,b] -> [c,d] -> [a,d]
            let area = (abs(a - c) + 1) * (abs(b - d) + 1);
            // only do tests if the area is larger than the current maximum
            if area <= out {
                continue;
            }
            // test if the other corners are inside
            if !is_inside(&parsed, (*c, *b)) || !is_inside(&parsed, (*a, *d)) {
                continue;
            }
            // test if the rectangle sides cross any polygon edge
            let mut inside = true;
            for ((u, v), (w, x)) in parsed.iter().zip(parsed.iter().cycle().skip(1)) {
                if u == w {
                    if min(a, c) < u && u < max(a, c) {
                        if min(b, d) >= min(v, x) && min(b, d) < max(v, x) {
                            inside = false;
                            break;
                        }
                        if max(b, d) > min(v, x) && max(b, d) <= max(v, x) {
                            inside = false;
                            break;
                        }
                    }
                } else {
                    if v > min(b, d) && v < max(b, d) {
                        if min(a, c) >= min(u, w) && min(a, c) < max(u, w) {
                            inside = false;
                            break;
                        }
                        if max(a, c) > min(u, w) && max(a, c) <= max(u, w) {
                            inside = false;
                            break;
                        }
                    }
                }
            }
            if inside {
                out = area
            }
        }
    }
    Some(out as u64)
}

fn is_inside(polygon: &Vec<(i64, i64)>, point: (i64, i64)) -> bool {
    let mut out = false;
    let (x0, y0) = point;
    for i in 0..polygon.len() {
        let j = (i + 1).rem_euclid(polygon.len());
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[j];
        if x1 != x2 {
            if x0 >= min(x1, x2) && x0 <= max(x2, x2) && y0 == y1 {
                return true;
            }
        }
        if y0 >= min(y1, y2) && y0 <= max(y1, y2) {
            if x0 == x1 {
                return true;
            }
            if x0 <= x1 {
                out = !out
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
