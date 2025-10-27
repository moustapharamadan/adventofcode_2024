use anyhow::Result;
use std::fs;
use regex::Regex;

pub fn part1(path: &str) -> Result<i32> {
    let content = fs::read_to_string(path)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

    Ok(re.captures_iter(&content).map(|cap| {
        let num1: i32 = cap[1].parse().expect("Expected integer");
        let num2: i32 = cap[2].parse().expect("Expected integer");
        return num1*num2;
    }).sum())
}

pub fn part2(path: &str) -> Result<i32> {
    let content = fs::read_to_string(path)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")?;
    let mut enabled = true;
    let mut total = 0;

    for cap in re.captures_iter(&content) {
        match &cap[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if cap[0].starts_with("mul") && enabled {
                    let num1: i32 = cap[1].parse().expect("Expected integer");
                    let num2: i32 = cap[2].parse().expect("Expected integer");
                    total += num1 * num2;
                }
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = part1("data/example/day3_part1.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 161);
    }

    #[test]
    fn test_part2(){
        let result = part2("data/example/day3_part2.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 48);
    }
}