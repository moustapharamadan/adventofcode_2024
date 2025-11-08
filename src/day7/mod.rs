use anyhow::{Ok, Result};
use std::{fs, u128};

fn parse(path: &str) -> Result<Vec<Vec<u128>>> {
    let content = fs::read_to_string(path)?;

    Ok(content
        .lines()
        .map(|line| {
            line.split_terminator([' ', ':'])
                .filter_map(|word| word.parse::<u128>().ok())
                .collect()
        })
        .collect())
}

pub fn part1(path: &str) -> Result<u128> {
    let equations = parse(path)?;

    let mut result = 0;
    for equation in equations {
        if equation.len() < 2 {
            continue;
        }

        let test_value = equation[0];
        let mut possibilities = vec![equation[1]];
        for num in &equation[2..] {
            let mut combinations = Vec::new();
            while let Some(value) = possibilities.pop() {
                combinations.push(value + num);
                combinations.push(value * num);
            }

            possibilities = combinations;
        }

        if let Some(_) = possibilities.iter().find(|&&value| value == test_value) {
            result += test_value;
        }
    }

    Ok(result)
}

pub fn part2(path: &str) -> Result<u128> {
    let equations = parse(path)?;

    let mut result = 0;
    for equation in equations {
        if equation.len() < 2 {
            continue;
        }

        let test_value = equation[0];
        let mut possibilities = vec![equation[1]];
        for num in &equation[2..] {
            let mut combinations = Vec::new();
            while let Some(value) = possibilities.pop() {
                combinations.push(value + num);
                combinations.push(value * num);
                combinations.push(format!("{value}{num}").parse::<u128>().unwrap());
            }

            possibilities = combinations;
        }

        if let Some(_) = possibilities.iter().find(|&&value| value == test_value) {
            result += test_value;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let result = part1("data/example/day7.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3749);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day7.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 11387);
    }
}
