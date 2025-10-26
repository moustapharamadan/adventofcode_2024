use anyhow::Result;
use std::{collections::HashMap, fs};

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let content = fs::read_to_string(input)?;

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for row in content.lines() {
        let parts: Vec<i32> = row
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Expected integer"))
            .collect::<Vec<i32>>();
        if parts.len() == 2 {
            col1.push(parts[0]);
            col2.push(parts[1]);
        }
    }

    Ok((col1, col2))
}
pub fn part1(input: &str) -> Result<u32> {
    let (mut col1, mut col2) = parse_input(input)?;

    col1.sort();
    col2.sort();

    return Ok(col1
        .iter()
        .zip(col2.iter())
        .map(|pair| (pair.0 - pair.1).abs() as u32)
        .sum());
}

pub fn part2(input: &str) -> Result<i32> {
    let (col1, col2) = parse_input(input)?;

    let value_count_map = col2.iter().fold(HashMap::new(), |mut acc, value| {
        *acc.entry(value).or_insert(0) += 1;
        acc
    });

    return Ok(col1
        .iter()
        .map(|value| value_count_map.get(value).unwrap_or(&0) * value)
        .sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("data/example/day1.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day1.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 31);
    }
}
