use anyhow::{Ok, Result};
use std::{collections::HashSet, fs};

fn parse(path: &str) -> Result<Vec<Vec<u8>>> {
    let content = fs::read_to_string(path)?;

    Ok(content
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect())
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_str(c: u8) -> Option<Self> {
        if c == b'^' {
            Some(Direction::Up)
        } else if c == b'v' {
            Some(Direction::Down)
        } else if c == b'<' {
            Some(Direction::Left)
        } else if c == b'>' {
            Some(Direction::Right)
        } else {
            None
        }
    }
    fn advance(&self, x: i32, y: i32) -> (i32, i32) {
        let (dx, dy) = match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };

        let x = x as i32 + dx;
        let y = y as i32 + dy;

        (x, y)
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn find_start_position(graph: &Vec<Vec<u8>>) -> (i32, i32, Direction) {
    graph
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter().enumerate().find_map(|(column, &c)| {
                if let Some(direction) = Direction::from_str(c) {
                    Some((row as i32, column as i32, direction))
                } else {
                    None
                }
            })
        })
        .expect("Start position not found")
}

pub fn part1(path: &str) -> Result<u32> {
    let mut graph = parse(path)?;

    let row_len = graph.len();
    if row_len == 0 {
        return Ok(0);
    }

    let column_len = graph[0].len();

    let (mut pos_x, mut pos_y, mut dir) = find_start_position(&graph);

    let mut visited_cells = 1;
    graph[pos_x as usize][pos_y as usize] = b'X';
    loop {
        let (new_x, new_y) = dir.advance(pos_x, pos_y);
        if new_x == -1 || new_x == row_len as i32 || new_y == -1 || new_y == column_len as i32 {
            break;
        }
        if graph[new_x as usize][new_y as usize] == b'#' {
            dir = dir.rotate_right();
            continue;
        }
        (pos_x, pos_y) = (new_x, new_y);

        let cell = &mut graph[pos_x as usize][pos_y as usize];
        if *cell != b'X' {
            *cell = b'X';
            visited_cells += 1;
        }
    }

    Ok(visited_cells)
}

pub fn has_loop(graph: Vec<Vec<u8>>) -> Result<bool> {
    let row_len = graph.len();
    if row_len == 0 {
        return Ok(false);
    }

    let column_len = graph[0].len();

    let (mut pos_x, mut pos_y, mut dir) = find_start_position(&graph);
    let mut visisted = HashSet::new();
    loop {
        if !visisted.insert((pos_x, pos_y, dir.clone())) {
            return Ok(true);
        }
        let (new_x, new_y) = dir.advance(pos_x, pos_y);
        if new_x == -1 || new_x == row_len as i32 || new_y == -1 || new_y == column_len as i32 {
            return Ok(false);
        }
        if graph[new_x as usize][new_y as usize] == b'#' {
            dir = dir.rotate_right();
            continue;
        }
        (pos_x, pos_y) = (new_x, new_y);
    }
}

pub fn part2(path: &str) -> Result<u32> {
    let graph = parse(path)?;

    let row_len = graph.len();
    if row_len == 0 {
        return Ok(0);
    }

    let column_len = graph[0].len();

    let mut count = 0;
    for row in 0..row_len {
        for column in 0..column_len {
            if graph[row][column] != b'.' {
                continue;
            }
            let mut graph = graph.clone();
            graph[row][column] = b'#';

            if has_loop(graph).unwrap_or(false) {
                count += 1;
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
        let result = part1("data/example/day6.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day6.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 6);
    }
}
