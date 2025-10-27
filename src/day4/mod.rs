use anyhow::Result;
use std::fs;

pub fn part1(path: &str) -> Result<u32> {
    const XMAS: &str = "XMAS";
    const SAMX: &str = "SAMX";

    let content = fs::read_to_string(path)?;
    let content: Vec<&str> = content.split_whitespace().collect();

    if content.is_empty() {
        return Ok(0);
    }

    let row_len = content.len();
    let col_len = content[0].len();

    let mut result = 0;
    for row in 0..row_len {
        for col in 0..col_len {
            let row_end = row + XMAS.len();
            if row_end <= row_len {
                let sub_str: String = content[row..row_end]
                    .iter()
                    .map(|s| s.chars().nth(col).expect("Expected char"))
                    .collect();

                if sub_str == XMAS || sub_str == SAMX {
                    // println!("[Vertical] Row: {row}, Col: {col}, sub_str: {sub_str}");
                    result += 1;
                }
            }

            let col_end = col + XMAS.len();
            if col_end <= col_len {
                if &content[row][col..col_end] == XMAS || &content[row][col..col_end] == SAMX {
                    // println!("[Horizontal] Row: {row}, Col: {col}, sub_str: {:?}", &content[row][col..col_end]);
                    result += 1;
                }
            }

            if row_end <= row_len && col_end <= col_len {
                let sub_str: String = content[row..row_end]
                    .iter()
                    .enumerate()
                    .map(|(i, s)| s.chars().nth(col + i).expect("Expected char"))
                    .collect();

                if sub_str == XMAS || sub_str == SAMX {
                    // println!("[Diagonal Right] Row: {row}, Col: {col}, sub_str: {sub_str}");
                    result += 1;
                }
            }

            let col_start = col as i32 - XMAS.len() as i32 + 1;
            if row_end <= row_len && 0 <= col_start {
                let sub_str: String = content[row..row_end]
                    .iter()
                    .enumerate()
                    .map(|(i, s)| s.chars().nth(col - i).expect("Expected char"))
                    .collect();

                if sub_str == XMAS || sub_str == SAMX {
                    // println!("[Diagonal Left] Row: {row}, Col: {col}, sub_str: {sub_str}");
                    result += 1;
                }
            }
        }
    }
    Ok(result)
}

pub fn part2(path: &str) -> Result<u32> {
    const MAS: &str = "MAS";
    const SAM: &str = "SAM";

    let content = fs::read_to_string(path)?;
    let content: Vec<&str> = content.split_whitespace().collect();

    if content.is_empty() {
        return Ok(0);
    }

    let row_len = content.len();
    let col_len = content[0].len();

    let mut result = 0;
    for row in 0..row_len {
        for col in 0..col_len {
            let row_end = row + MAS.len();
            let col_end = col + MAS.len();
            if row_end <= row_len && col_end <= col_len {
                let sub_str: String = content[row..row_end]
                    .iter()
                    .enumerate()
                    .map(|(i, s)| s.chars().nth(col + i).expect("Expected char"))
                    .collect();

                let mut is_x_mas = sub_str == MAS || sub_str == SAM;

                let sub_str: String = content[row..row_end]
                    .iter()
                    .enumerate()
                    .map(|(i, s)| s.chars().nth(col_end - i - 1).expect("Expected char"))
                    .collect();
                is_x_mas &= sub_str == MAS || sub_str == SAM;

                if is_x_mas {
                    result += 1;
                }
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_test1() {
        let result = part1("data/example/day4_part1_test1.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_part1_test2() {
        let result = part1("data/example/day4_part1_test2.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 18);
    }

    #[test]
    fn test_part2_test1() {
        let result = part2("data/example/day4_part2_test1.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_part2_test2() {
        let result = part2("data/example/day4_part2_test2.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 9);
    }
}
