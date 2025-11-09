use anyhow::Result;
use std::{fs, iter};

fn parse(path: &str) -> Result<Vec<Option<usize>>> {
    let content = fs::read_to_string(path)?;
    let mut disk: Vec<Option<usize>> = Vec::new();

    for (index, ch) in content.char_indices() {
        let count = ch.to_digit(10).unwrap() as usize;
        let value = if index % 2 == 0 {
            Some(index / 2)
        } else {
            None
        };
        disk.extend(iter::repeat(value).take(count));
    }

    Ok(disk)
}

pub fn part1(path: &str) -> Result<u64> {
    let mut disk = parse(path)?;
    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        if disk[left].is_some() {
            left += 1;
        } else if disk[right].is_none() {
            right -= 1;
        } else {
            disk.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    Ok(disk
        .iter()
        .enumerate()
        .map(|(index, &value)| {
            if let Some(value) = value {
                return (index * value) as u64;
            } else {
                0
            }
        })
        .sum::<u64>())
}

pub fn part2(path: &str) -> Result<u64> {
    let mut disk = parse(path)?;

    // Process files from highest ID to lowest
    let mut file_id = disk.iter().filter_map(|&x| x).max().unwrap_or(0);

    while file_id > 0 {
        // Find file position and size
        let start = disk.iter().position(|&x| x == Some(file_id)).unwrap();
        let size = disk
            .iter()
            .skip(start)
            .take_while(|&&x| x == Some(file_id))
            .count();

        // Find leftmost free space that fits
        let mut free_start = 0;
        let mut found = false;

        while free_start < start && !found {
            if disk[free_start].is_none() {
                let mut free_end = free_start;
                while free_end < disk.len() && disk[free_end].is_none() {
                    free_end += 1;
                }

                if free_end - free_start >= size {
                    // Move file
                    for i in 0..size {
                        disk[free_start + i] = Some(file_id);
                        disk[start + i] = None;
                    }
                    found = true;
                } else {
                    free_start = free_end;
                }
            } else {
                free_start += 1;
            }
        }

        file_id -= 1;
    }

    Ok(disk
        .iter()
        .enumerate()
        .map(|(index, &value)| {
            if let Some(value) = value {
                return (index * value) as u64;
            } else {
                0
            }
        })
        .sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let result = part1("data/example/day9.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1928);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day9.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2858);
    }
}
