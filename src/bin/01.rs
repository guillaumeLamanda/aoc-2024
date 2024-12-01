use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip();

    first.sort();
    second.sort();

    Some(
        first
            .iter()
            .zip(second.iter())
            .map(|(a, &b)| a.abs_diff(b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| {
            (
                a.trim().parse::<u32>().unwrap(),
                b.trim().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    let mut map: HashMap<u32, u32> = HashMap::new();
    for &value in second.iter() {
        *map.entry(value).or_insert(0) += 1;
    }

    Some(
        first
            .iter()
            .map(|a| {
                let num_of_occurs = map.get(a).unwrap_or(&0);
                a * num_of_occurs
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
