use anyhow::Result;
use std::{collections::HashMap, fs};

fn parse(path: &str) -> Result<Vec<Vec<char>>> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(|line| line.chars().collect()).collect())
}

fn is_in_bounds(x: i32, y: i32, rows: usize, cols: usize) -> bool {
    0 <= x && (x as usize) < rows && 0 <= y && (y as usize) < cols
}

pub fn part1(path: &str) -> Result<u32> {
    let mut map = parse(path)?;
    if map.is_empty() {
        return Ok(0);
    }
    let (rows, cols) = (map.len(), map[0].len());

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (row, line) in map.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_default()
                    .push((row as i32, col as i32));
            }
        }
    }

    let mut count = 0;
    for locations in antennas.values() {
        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let (x1, y1) = locations[i];
                let (x2, y2) = locations[j];
                let (dx, dy) = (x2 - x1, y2 - y1);

                for (nx, ny) in [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)] {
                    if is_in_bounds(nx, ny, rows, cols) && map[nx as usize][ny as usize] != '#' {
                        map[nx as usize][ny as usize] = '#';
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}

pub fn part2(path: &str) -> Result<u32> {
    let mut map = parse(path)?;
    if map.is_empty() {
        return Ok(0);
    }
    let (rows, cols) = (map.len(), map[0].len());

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (row, line) in map.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_default()
                    .push((row as i32, col as i32));
            }
        }
    }

    let mut count = 0;
    for locations in antennas.values() {
        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let (x1, y1) = locations[i];
                let (x2, y2) = locations[j];
                let (dx, dy) = (x2 - x1, y2 - y1);

                let mut x = x1;
                let mut y = y1;
                loop {
                    if !is_in_bounds(x, y, rows, cols) {
                        break;
                    }
                    if map[x as usize][y as usize] != '#' {
                        map[x as usize][y as usize] = '#';
                        count += 1;
                    }
                    x -= dx;
                    y -= dy;
                }

                let mut x = x2;
                let mut y = y2;
                loop {
                    if !is_in_bounds(x, y, rows, cols) {
                        break;
                    }
                    if map[x as usize][y as usize] != '#' {
                        map[x as usize][y as usize] = '#';
                        count += 1;
                    }
                    x += dx;
                    y += dy;
                }
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let result = part1("data/example/day8.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 14);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day8_part2.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 9);

        let result = part2("data/example/day8.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 34);
    }
}
