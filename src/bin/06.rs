use itertools::Itertools;
use pathfinding::matrix::{directions, Matrix};

advent_of_code::solution!(6);

type Direction = (isize, isize);
type PositionAndDirection = ((usize, usize), Direction);

fn get_direction(value: &char) -> Direction {
    match value {
        '^' => directions::N,
        'v' => directions::S,
        '<' => directions::W,
        '>' => directions::E,
        c => panic!("value not handled: {}", c),
    }
}

fn get_starting_position(matrix: &Matrix<char>) -> Option<(usize, usize)> {
    matrix.iter().enumerate().find_map(|(x, row)| {
        row.iter()
            .enumerate()
            .find_map(|(y, c)| if *c == '^' { Some((x, y)) } else { None })
    })
}

fn build_path(
    matrix: &Matrix<char>,
    start_position: (usize, usize),
    mut direction: (isize, isize),
    already_visited: &[PositionAndDirection],
) -> Result<Vec<PositionAndDirection>, ()> {
    let mut position = start_position;
    let mut visited = Vec::from(already_visited);

    while let Some(new_position) = matrix.move_in_direction(position, direction) {
        let value = matrix.get(new_position);
        match value {
            Some('#') => direction = rotate_90(direction),
            Some('.') => {
                position = new_position;
                if visited.contains(&(position, direction)) {
                    return Err(());
                }
                visited.push((position, direction));
            }
            _ => panic!("value should not be there"),
        }
    }
    Ok(visited)
}

fn rotate_90(direction: (isize, isize)) -> (isize, isize) {
    match direction {
        directions::N => directions::E,
        directions::E => directions::S,
        directions::S => directions::W,
        directions::W => directions::N,
        _ => panic!("direction not handled"),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let start_position = get_starting_position(&matrix)?;
    let direction = get_direction(matrix.get(start_position).unwrap());

    *matrix.get_mut(start_position).unwrap() = '.';

    let visited = build_path(&matrix, start_position, direction, &[]).unwrap();
    let length = visited
        .iter()
        .map(|(position, _)| position)
        .unique()
        .collect::<Vec<&(usize, usize)>>()
        .len();

    Some(length)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let start_position = get_starting_position(&matrix)?;
    let direction = get_direction(matrix.get(start_position).unwrap());

    *matrix.get_mut(start_position).unwrap() = '.';

    let visited = build_path(&matrix, start_position, direction, &[]).unwrap();

    let length = visited
        .iter()
        .filter(|position_and_direction| {
            is_obstruction_result_in_infinite_loop(
                position_and_direction,
                &matrix,
                &visited
                    .iter()
                    .take_while(|x| x == position_and_direction)
                    .copied()
                    .collect::<Vec<PositionAndDirection>>(),
            )
        })
        .count();

    Some(length)
}

fn is_obstruction_result_in_infinite_loop(
    (position, direction): &PositionAndDirection,
    map: &Matrix<char>,
    already_visited: &[PositionAndDirection],
) -> bool {
    let mut new_matrix = map.clone();
    let obstruction_position = new_matrix.move_in_direction(*position, *direction);
    if obstruction_position.is_none() {
        return false;
    }
    *new_matrix.get_mut(obstruction_position.unwrap()).unwrap() = '#';
    match build_path(&new_matrix, *position, *direction, already_visited) {
        Err(()) => true,
        Ok(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn matrix() {
        let m = Matrix::new_square(8, '.');
        assert_eq!(m.move_in_direction((1, 1), (2, 1)), Some((3, 2)));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
