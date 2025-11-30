use anyhow::Result;
use std::{collections::HashSet, fs};

fn parse(path: &str) -> Result<Vec<Vec<u32>>> {
    let content = fs::read_to_string(path)?;
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in content.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            let value = ch.to_digit(10).unwrap();
            row.push(value);
        }
        map.push(row);
    }

    Ok(map)
}

fn neighbours(x: i32, y: i32) -> [(i32, i32); 4] {
    [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
}

fn count_trail_heads(start_row: usize, start_col: usize, map: &Vec<Vec<u32>>, unique: bool) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((start_row, start_col));

    let mut result = 0;
    while let Some((x, y)) = queue.pop() {
        if unique && visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if map[x as usize][y as usize] == 9 {
            result += 1;
            continue;
        }

        let target_value = map[x][y] + 1;
        for (n_x, n_y) in neighbours(x as i32, y as i32) {
            if 0 <= n_x && n_x < map.len() as i32 && 0 <= n_y && n_y < map[0].len() as i32 {
                if map[n_x as usize][n_y as usize] == target_value {
                    queue.push((n_x as usize, n_y as usize));
                }
            }
        }
    }
    result
}

fn solve(path: &str, unique: bool) -> Result<u32> {
    let map = parse(path)?;

    let mut result = 0;
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if col == &0 {
                result += count_trail_heads(row_index, col_index, &map, unique);
            }
        }
    }
    Ok(result)
}

pub fn part1(path: &str) -> Result<u32> {
    solve(path, true)
}

pub fn part2(path: &str) -> Result<u32> {
    solve(path, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_1() {
        let result = part1("data/example/day10_part1_1.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_part1_2() {
        let result = part1("data/example/day10.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 36);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day10.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 81);
    }
}
