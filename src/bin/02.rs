use itertools::Itertools;

advent_of_code::solution!(2);

fn is_safe_report(report: Vec<i32>) -> bool {
    let report = report.into_iter();
    (report.clone().tuple_windows().all(|(l, r)| l < r)
        || report.clone().tuple_windows().all(|(l, r)| l > r))
        && report
            .clone()
            .tuple_windows()
            .all(|(l, r)| (l - r).abs() <= 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|level| is_safe_report(level.clone()))
        .collect();

    Some(reports.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|levels| {
            let valid = is_safe_report(levels.clone());
            if valid {
                return true;
            }
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if is_safe_report(new_levels) {
                    return true;
                }
            }
            false
        })
        .collect();

    Some(reports.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
