#![allow(clippy::manual_inspect)]
use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

enum OrderingRule {
    Before(i32),
    After(i32),
}

type RulesHashMap = HashMap<i32, Vec<OrderingRule>>;

fn ordering(a: &i32, b: &i32, rules: &RulesHashMap) -> Ordering {
    rules.get(a).map_or(Ordering::Equal, |rules| {
        rules
            .iter()
            .find_map(|rule| match rule {
                OrderingRule::Before(n) if b == n => Some(Ordering::Less),
                OrderingRule::After(n) if b == n => Some(Ordering::Greater),
                _ => None,
            })
            .unwrap_or(Ordering::Equal)
    })
}

fn rules_check(rules: &RulesHashMap, update: Vec<i32>) -> i32 {
    if update.is_sorted_by(|a, b| ordering(a, b, rules).is_le()) {
        return update[update.len() / 2];
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(|(dependancies, updates)| {
        updates
            .iter()
            .map(|update| rules_check(&dependancies, update.clone()))
            .sum::<i32>() as u32
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    parse(input).map(|(dependancies, mut updates)| {
        updates
            .iter_mut()
            .filter(|update| !update.is_sorted_by(|a, b| ordering(a, b, &dependancies).is_le()))
            .map(|update| {
                update.sort_by(|a, b| ordering(a, b, &dependancies));
                update
            })
            .map(|update| rules_check(&dependancies, update.clone()))
            .sum::<i32>() as u32
    })
}

fn parse(input: &str) -> Option<(RulesHashMap, Vec<Vec<i32>>)> {
    input.split_once("\n\n").map(|(deps, pages)| {
        (
            deps.lines()
                .fold(HashMap::<i32, Vec<OrderingRule>>::new(), |mut map, line| {
                    line.split_once('|')
                        .map(|(before, after)| {
                            let before = before.parse().unwrap();
                            let after = after.parse().unwrap();
                            map.entry(before)
                                .or_default()
                                .push(OrderingRule::Before(after));
                            map.entry(after)
                                .or_default()
                                .push(OrderingRule::After(before));
                        })
                        .unwrap();
                    map
                }),
            pages
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|page| page.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
