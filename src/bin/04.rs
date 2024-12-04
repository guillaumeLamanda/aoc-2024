advent_of_code::solution!(4);

fn count_xmas(row: Vec<char>) -> i32 {
    if row.len() < 4 {
        return 0;
    }
    row.windows(4)
        .filter(|&w| w.iter().collect::<String>() == "XMAS")
        .count() as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            // Count columns
            if row <= map.len() - 4 {
                let column: Vec<char> = (0..4).map(|offset| map[row + offset][col]).collect();
                count += count_xmas(column.clone());
                count += count_xmas(column.into_iter().rev().collect());
            }

            // Count rows
            if col <= map[row].len() - 4 {
                let row_segment: Vec<char> = (0..4).map(|offset| map[row][col + offset]).collect();
                count += count_xmas(row_segment.clone());
                count += count_xmas(row_segment.into_iter().rev().collect());
            }

            // Count diagonals
            if row <= map.len() - 4 && col <= map[row].len() - 4 {
                let diagonal_1: Vec<char> = (0..4)
                    .map(|offset| map[row + offset][col + offset])
                    .collect();
                count += count_xmas(diagonal_1.clone());
                count += count_xmas(diagonal_1.into_iter().rev().collect());

                let diagonal_2: Vec<char> = (0..4)
                    .map(|offset| map[row + offset][col + 3 - offset])
                    .collect();
                count += count_xmas(diagonal_2.clone());
                count += count_xmas(diagonal_2.into_iter().rev().collect());
            }
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = vec![];
    for (y, line) in input.lines().enumerate() {
        for c in line.chars() {
            if map.len() <= y {
                map.push(vec![]);
            }
            map[y].push(c);
        }
    }

    Some(
        map.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == 'A')
                    .map(move |(x, _)| (x, y))
            })
            .filter(|(x, y)| {
                if x == &0 || y == &0 || x == &(map[0].len() - 1) || y == &(map.len() - 1) {
                    return false;
                }

                matches!(
                    (
                        map[y - 1][x - 1],
                        map[y - 1][x + 1],
                        map[y + 1][x - 1],
                        map[y + 1][x + 1],
                    ),
                    ('S', 'S', 'M', 'M')
                        | ('M', 'S', 'M', 'S')
                        | ('M', 'M', 'S', 'S')
                        | ('S', 'M', 'S', 'M')
                )
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
