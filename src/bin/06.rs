use std::cmp::PartialEq;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position(isize, isize);
#[derive(Debug, PartialEq)]
enum Occupant {
    None,
    Guard,
    Obstacle
}

#[derive(Debug)]
struct Cell {
    index: isize,
    position: Position,
    visited: bool,
    occupant: Occupant
}

#[derive(Debug)]
struct Map {
    grid: Vec<Cell>
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Map {
    fn from_string(input: &str) -> Map {
        let mut cells = Vec::new();
        let mut index = 0;

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let position = Position(i as isize, j as isize);
                let occupant = match c {
                    '#' => Occupant::Obstacle,
                    '^' => Occupant::Guard,
                    _ => Occupant::None
                };

                cells.push(Cell {
                    index,
                    position,
                    occupant,
                    visited: false
                });
                index += 1;
            }
        }
        Map {
            grid: cells
        }
    }

    fn determine_next_position(current_position: Position, direction: &Direction) -> Position {
        match direction {
            Direction::Up => {
                Position(current_position.0 - 1, current_position.1)
            }
            Direction::Down => {
                Position(current_position.0 + 1, current_position.1)
            }
            Direction::Left => {
                Position(current_position.0, current_position.1 - 1)
            }
            Direction::Right => {
                Position(current_position.0, current_position.1 + 1)
            }
        }
    }

    fn rotate_direction(direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }

    fn get_visited_cells(&self) -> Vec<&Cell> {
        self.grid.iter().filter(|c| c.visited).collect()
    }

    fn find_cell(&mut self, position: &Position) -> Option<&mut Cell> {
        match self.grid.iter_mut().find(|c| c.position == *position) {
            Some(c) => Some(c),
            _ => None
        }
    }

    fn find_guard_position(&self) -> Option<Position> {
        match self.grid.iter().find(|c| c.occupant == Occupant::Guard) {
            Some(c) => Some(c.position.clone()),
            None => None
        }
    }

    fn simulate_guard_movement(&mut self) {
        let mut current_position = self.find_guard_position().unwrap();
        let mut current_direction = Direction::Up;

         loop {
            let Some(current_cell) = self.find_cell(&current_position) else {
                break
            };

            current_cell.visited = true;
            current_cell.occupant = Occupant::Guard;

            let mut possible_position = Map::determine_next_position(current_position, &current_direction);
            let possible_cell = self.find_cell(&possible_position);

            if possible_cell.is_some() && possible_cell.unwrap().occupant == Occupant::Obstacle {
                current_direction =  Map::rotate_direction(current_direction);
                possible_position = Map::determine_next_position(current_position, &current_direction);
            }

            current_position = possible_position;

        }
    }


}
pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_string(input);
    map.simulate_guard_movement();
    let visited_cells = map.get_visited_cells();

    Some(visited_cells.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_determine_next_position() {
        let position = Position(1, 1);
        assert_eq!(Map::determine_next_position(Position(1, 1), &Direction::Up), Position(0, 1));
        assert_eq!(Map::determine_next_position(Position(1, 1), &Direction::Down), Position(2, 1));
        assert_eq!(Map::determine_next_position(Position(1, 1), &Direction::Left), Position(1, 0));
        assert_eq!(Map::determine_next_position(Position(1, 1), &Direction::Right), Position(1, 2));
    }
}
