use anyhow::Result;
use std::fs;

fn parse_input(path: &str) -> Result<Vec<Vec<i32>>> {
    let content = fs::read_to_string(path)?;

    let mut result = Vec::new();
    for row in content.lines() {
        result.push(
            row.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Expected integer"))
                .collect::<Vec<i32>>(),
        );
    }

    Ok(result)
}

fn is_safe(row: &Vec<i32>, recursive: bool) -> bool {
    if row.len() < 2 {
        return true;
    }

    let coef = if row[0] < row[1] {
        -1
    } else if row[0] > row[1] {
        1
    } else {
        0
    };

    for i in 0..row.len() - 1 {
        let diff = (row[i] - row[i + 1]) * coef;
        let safe = 1 <= diff && diff <= 3;
        if !safe && !recursive {
            return false;
        } else if !safe {
            let mut row_without_i = row.clone();
            row_without_i.remove(i);

            let mut row_without_i_plus_1 = row.clone();
            row_without_i_plus_1.remove(i + 1);

            if is_safe(&row_without_i, false) || is_safe(&row_without_i_plus_1, false) {
                return true;
            }

            if i >= 1 {
                let mut row_without_i_min_1 = row.clone();
                row_without_i_min_1.remove(i - 1);
                if is_safe(&row_without_i_min_1, false) {
                    return true;
                }
            }

            return false;
        }
    }
    true
}

pub fn part1(path: &str) -> Result<u32> {
    let input = parse_input(path)?;
    Ok(input
        .iter()
        .map(|row| if is_safe(&row, false) { 1 } else { 0 })
        .sum())
}

pub fn part2(path: &str) -> Result<u32> {
    let input = parse_input(path)?;
    Ok(input
        .iter()
        .map(|row| if is_safe(&row, true) { 1 } else { 0 })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("data/example/day2.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day2.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }
}
