use std::collections::{HashMap, HashSet};

use geo::Coord;
use itertools::Itertools;
use pathfinding::matrix::Matrix;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();

    let antenas = get_antenas(&matrix);

    fn antinodes(
        antenas: &HashMap<char, Vec<Coord<isize>>>,
        matrix: Matrix<char>,
    ) -> HashSet<Coord<isize>> {
        antenas
            .iter()
            .flat_map(|(_, ants)| {
                ants.iter().tuple_combinations().flat_map(|(a, b)| {
                    [
                        Coord {
                            x: 2 * a.x - b.x,
                            y: 2 * a.y - b.y,
                        },
                        Coord {
                            x: 2 * b.x - a.x,
                            y: 2 * b.y - a.y,
                        },
                    ]
                })
            })
            .filter(|point| matrix.get((point.x as usize, point.y as usize)).is_some())
            .collect()
    }

    Some(antinodes(&antenas, matrix).len())
}

fn get_antenas(matrix: &Matrix<char>) -> HashMap<char, Vec<Coord<isize>>> {
    let mut antenas: HashMap<char, Vec<Coord<isize>>> = HashMap::new();
    for (x, row) in matrix.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if *value != '.' {
                antenas
                    .entry(*value)
                    .or_default()
                    .push((x as isize, y as isize).into());
            }
        }
    }
    antenas
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();

    let antenas = get_antenas(&matrix);

    fn antinodes(
        antenas: &HashMap<char, Vec<Coord<isize>>>,
        matrix: Matrix<char>,
    ) -> HashSet<Coord<isize>> {
        antenas
            .iter()
            .flat_map(|(_, ants)| {
                ants.iter().tuple_combinations().flat_map(|(a, b)| {
                    (0..)
                        .map(|i| Coord {
                            x: b.x + i * (a.x - b.x),
                            y: b.y + i * (a.y - b.y),
                        })
                        .take_while(|point| {
                            matrix.get((point.x as usize, point.y as usize)).is_some()
                        })
                        .chain(
                            (2..)
                                .map(|i| Coord {
                                    x: a.x + i * (b.x - a.x),
                                    y: a.y + i * (b.y - a.y),
                                })
                                .take_while(|point| {
                                    matrix.get((point.x as usize, point.y as usize)).is_some()
                                }),
                        )
                })
            })
            .collect()
    }

    Some(antinodes(&antenas, matrix).len())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn simple_antenas() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        assert_eq!(part_one(&input), Some(2))
    }
}
