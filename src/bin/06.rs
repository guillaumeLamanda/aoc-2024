use std::str::FromStr;

use geo::Point;
use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl TryFrom<&char> for Direction {
    type Error = char;
    fn try_from(s: &char) -> Result<Self, Self::Error> {
        match s {
            '<' => Ok(Direction::LEFT),
            '>' => Ok(Direction::RIGHT),
            '^' => Ok(Direction::UP),
            'v' => Ok(Direction::DOWN),
            c => Err(*c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    pub position: Point<usize>,
    direction: Direction,
}

impl Guard {
    fn rotate_90(&mut self) {
        self.direction = match self.direction {
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
        }
    }

    fn next_position(&self) -> Result<Point<usize>, &'static str> {
        match self.direction {
            Direction::LEFT if self.position.x() == 0 => {
                Err("Error: cannot move left, would go below 0")
            }
            Direction::LEFT => Ok(Point::new(self.position.x() - 1, self.position.y())),
            Direction::RIGHT => Ok(Point::new(self.position.x() + 1, self.position.y())),
            Direction::UP if self.position.y() == 0 => {
                Err("Error: cannot move up, would go below 0")
            }
            Direction::UP => Ok(Point::new(self.position.x(), self.position.y() - 1)),
            Direction::DOWN => Ok(Point::new(self.position.x(), self.position.y() + 1)),
        }
    }
}

impl TryFrom<(usize, usize, &char)> for Guard {
    type Error = ();
    fn try_from((x, y, c): (usize, usize, &char)) -> Result<Self, Self::Error> {
        let direction = Direction::try_from(c);
        if direction.is_err() {
            return Err(());
        }
        Ok(Guard {
            position: Point::new(x, y),
            direction: direction.unwrap(),
        })
    }
}

impl TryFrom<&str> for Guard {
    type Error = ();
    fn try_from(map_state: &str) -> Result<Self, Self::Error> {
        map_state
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, char)| Guard::try_from((x, y, &char)).ok())
            })
            .ok_or(())
    }
}

#[derive(Debug)]
struct Map {
    state: Vec<Vec<char>>,
    guard: Guard,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let guard = Guard::try_from(s).unwrap();
        Ok(Map {
            state: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            if Direction::try_from(&c).is_ok() {
                                '.'
                            } else {
                                c
                            }
                        })
                        .collect()
                })
                .collect(),
            guard,
        })
    }
}

fn walk(
    guard: &mut Guard,
    state: &[Vec<char>],
    visited: &mut Vec<(Point<usize>, Direction)>,
) -> Result<(), ()> {
    let next_position = guard.next_position();
    if next_position.is_err() {
        return Err(());
    }
    let next_position = next_position.unwrap();
    if next_position.y() == state.len() || next_position.x() == state[0].len() {
        return Err(());
    }

    match state
        .get(next_position.y())
        .and_then(|line| line.get(next_position.x()))
    {
        None => {
            guard.rotate_90();
            visited.push((guard.position, guard.direction));
            Ok(())
        }
        Some('.') => {
            guard.position = next_position;
            visited.push((guard.position, guard.direction));
            Ok(())
        }
        Some('#') => {
            guard.rotate_90();
            visited.push((guard.position, guard.direction));
            Ok(())
        }
        Some(c) => panic!("unknown char {}", c),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Map::from_str(input).ok().map(|mut map| {
        let mut visited = vec![(map.guard.position, map.guard.direction)];
        while walk(&mut map.guard, &map.state, &mut visited).is_ok() {
            // println!("{:?}", map.guard);
        }

        let x = visited
            .iter()
            .map(|(point, _)| point.x_y())
            .unique()
            .collect::<Vec<_>>();

        x.len() as u32
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    Map::from_str(input).ok().map(|mut map| {
        let mut visited = vec![(map.guard.position, map.guard.direction)];
        while walk(&mut map.guard, &map.state, &mut visited).is_ok() {
            // println!("{:?}", map.guard);
        }

        let x = visited[1..]
            .iter()
            .filter(|visit| is_front_point_candidate_for_block(&map.state, visit))
            .filter(|visit| is_obstruction_result_in_infinite_loop(visit, &map.state))
            .map(|(point, _)| point.x_y())
            .unique()
            .collect::<Vec<_>>();
        // println!("{:?}", x);

        x.len() as u32
    })
}

fn is_obstruction_result_in_infinite_loop(
    (position, direction): &(Point<usize>, Direction),
    map: &[Vec<char>],
) -> bool {
    let mut guard = Guard {
        position: *position,
        direction: *direction,
    };

    guard
        .next_position()
        .ok()
        .filter(|position| {
            map.get(position.y())
                .and_then(|line| line.get(position.x()))
                .map_or(false, |c| c == &'.')
        })
        .map(|obstruction_position| {
            let mut new_map = Vec::from(map);
            new_map[obstruction_position.y()][obstruction_position.x()] = '#';
            let mut visited = vec![(guard.position, guard.direction)];
            while walk(&mut guard, &new_map, &mut visited).is_ok() {
                if visited[..visited.len() - 1].contains(&(guard.position, guard.direction)) {
                    // println!("{:?}, {:?}", visited, guard);
                    return true;
                }
            }
            false
        })
        .unwrap_or(false)
}

fn is_front_point_candidate_for_block(
    map: &[Vec<char>],
    (point, direction): &(Point<usize>, Direction),
) -> bool {
    match &direction {
        Direction::UP => map[point.y()][point.x()..].contains(&'#'),
        Direction::RIGHT => map[point.y()..]
            .iter()
            .map(|line| line[point.x()])
            .contains(&'#'),
        Direction::DOWN => map[point.y()][..point.x()].contains(&'#'),
        Direction::LEFT => map[..point.y()]
            .iter()
            .map(|line| line[point.x()])
            .contains(&'#'),
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
