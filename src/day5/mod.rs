use anyhow::{Ok, Result};
use std::{cmp::Ordering, collections::HashMap, fs};

pub fn part1(path: &str) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    let content: Vec<&str> = content.split_whitespace().collect();

    let mut dependencies: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut result = 0;
    for line in content {
        if line.is_empty() {
            continue;
        }

        if line.contains("|") {
            let mut pages = line.split('|');
            let page1 = pages
                .next()
                .expect("Expect str")
                .parse::<u32>()
                .expect("Expect integer");
            let page2 = pages
                .next()
                .expect("Expect str")
                .parse::<u32>()
                .expect("Expect integer");

            if let Some(dep) = dependencies.get_mut(&page2) {
                dep.push(page1);
            } else {
                dependencies.insert(page2, vec![page1]);
            }
        } else {
            let pages: Vec<u32> = line
                .split(',')
                .map(|p| p.parse::<u32>().expect("Expect integer"))
                .collect();
            let mut is_correct = true;
            let mut printed: Vec<u32> = Vec::new();
            for page in &pages {
                if let Some(dep) = dependencies.get(&page) {
                    let all_dep_printed = dep
                        .iter()
                        .all(|p| printed.contains(&p) || !pages.contains(&p));
                    if all_dep_printed {
                        printed.push(page.clone());
                    } else {
                        is_correct = false;
                        break;
                    }
                } else {
                    printed.push(page.clone());
                }
            }

            if is_correct {
                result += pages[pages.len() / 2];
            }
        }
    }
    Ok(result)
}

fn parse(path: &str) -> Result<(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)> {
    let content = fs::read_to_string(path)?;
    let mut line_iter = content.lines();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }

        let mut pages = line.split('|');
        let x = pages
            .next()
            .expect("Expect str")
            .parse::<u32>()
            .expect("Expect integer");
        let y = pages
            .next()
            .expect("Expect str")
            .parse::<u32>()
            .expect("Expect integer");

        if let Some(rule) = rules.get_mut(&x) {
            rule.push(y);
        } else {
            rules.insert(x, vec![y]);
        }
    }

    let mut updates: Vec<Vec<u32>> = Vec::new();
    for line in &mut line_iter {
        let pages: Vec<u32> = line
            .split(',')
            .map(|p| p.parse::<u32>().expect("Expect integer"))
            .collect();
        updates.push(pages);
    }

    Ok((rules, updates))
}

fn check(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> Option<u32> {
    for i in 0..update.len() - 1 {
        if let Some(x_rules) = rules.get(&update[i]) {
            if !x_rules.contains(&update[i + 1]) {
                return None;
            }
        } else {
            return None;
        }
    }
    Some(update[update.len() / 2])
}

fn sort(rules: &HashMap<u32, Vec<u32>>, update: &mut Vec<u32>) -> u32 {
    update.sort_by(|x, y| {
        if let Some(x_rules) = rules.get(x) {
            if x_rules.contains(y) {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        } else {
            return Ordering::Greater;
        }
    });
    update[update.len() / 2]
}

pub fn part1_new(path: &str) -> Result<u32> {
    let (rules, updates) = parse(path)?;

    let result = updates
        .iter()
        .filter_map(|update| check(&rules, update))
        .sum::<u32>();

    Ok(result)
}

pub fn part2(path: &str) -> Result<u32> {
    let (rules, mut updates) = parse(path)?;

    let result = updates
        .iter_mut()
        .filter(|update| check(&rules, update).is_none())
        .map(|update| sort(&rules, update))
        .sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let result = part1("data/example/day5.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 143);
    }

    #[test]
    fn test_part1_new() {
        let result = part1_new("data/example/day5.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 143);
    }

    #[test]
    fn test_part2() {
        let result = part2("data/example/day5.txt");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123);
    }
}
