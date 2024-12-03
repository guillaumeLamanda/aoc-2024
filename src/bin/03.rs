use core::str;

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    mul_of_str(input)
}

fn mul_of_str(input: &str) -> Option<u32> {
    let mut values: Vec<(u32, u32)> = vec![];
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [a, b]) in regex.captures_iter(input).map(|c| c.extract()) {
        values.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    Some(values.iter().map(|(l, r)| l * r).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    use regex::bytes::Regex;
    let reg = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do\(\)|don't\(\))").unwrap();
    Some(
        reg.captures_iter(input.as_bytes())
            .map(|c| c.extract())
            .fold((false, 0), |(skip, sum), (_, [a])| {
                match (str::from_utf8(a).unwrap(), skip) {
                    ("don't()", _) => (true, sum),
                    ("do()", _) => (false, sum),
                    (_, true) => (skip, sum),
                    (mul, false) => {
                        let (l, r) = mul
                            .split_once(',')
                            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
                            .unwrap();

                        (skip, sum + l * r)
                    }
                }
            })
            .1,
    )
}

#[allow(warnings)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
