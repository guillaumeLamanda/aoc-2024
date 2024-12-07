advent_of_code::solution!(7);

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(result, nums)| {
                    (
                        result.parse::<i64>().unwrap(),
                        nums.trim()
                            .split(' ')
                            .map(|num| num.parse().unwrap())
                            .collect::<Vec<i64>>(),
                    )
                })
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        parse(input)
            .iter()
            .filter(|(r, n)| check(*r, n, 0))
            .map(|(r, _)| r)
            .sum::<i64>(),
    )
}

fn check(val: i64, math: &[i64], acc: i64) -> bool {
    if math.is_empty() {
        return val == acc;
    }
    check(val, &math[1..], acc + math[0]) || check(val, &math[1..], acc * math[0])
}

fn check2(val: i64, math: &[i64], acc: i64) -> bool {
    if math.is_empty() {
        return val == acc;
    }
    let conc = concatenate_numbers(acc, math[0]);
    check2(val, &math[1..], acc + math[0])
        || check2(val, &math[1..], acc * math[0])
        || check2(val, &math[1..], conc)
}

fn concatenate_numbers(a: i64, b: i64) -> i64 {
    let concatenated = format!("{}{}", a, b);
    concatenated.parse::<i64>().unwrap()
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        parse(input)
            .iter()
            .filter(|(r, n)| check2(*r, n, 0))
            .map(|(r, _)| *r)
            .sum::<i64>(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
