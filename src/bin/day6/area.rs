use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    x: i32,
    y: i32
}

#[derive(Clone)]
pub struct Guard {
    position: Position,
    direction: Direction,
    visited: HashMap<Position, HashSet<Direction>>,
    will_loop: bool
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction { Up, Down, Left, Right }

pub struct Area {
    pub width: i32,
    pub height: i32,
    pub guard: Guard,
    pub obstacles: HashSet<Position>,
    initial_guard: Guard
}

impl Area {
    pub fn new(width: i32, height: i32, initial_guard: Guard, obstacles: HashSet<Position>) -> Self {
        Self {
            width,
            height,
            guard: initial_guard.clone(),
            obstacles,
            initial_guard
        }
    }

    pub fn how_many_distinct_positions(&mut self) -> usize {
        self.traverse();
        let count = self.guard.visited.len();
        self.reset();
        count
    }

    pub fn how_many_unique_new_obstacles_cause_a_loop(&mut self) -> usize {
        self.traverse();
        let mut traveled_positions = self.guard.visited.clone();
        traveled_positions.remove(&self.initial_guard.position);
        self.reset();

        traveled_positions.into_iter()
            .map(|(position, _)| {
                self.obstacles.insert(position);
                let is_loop = self.traverse();
                self.obstacles.remove(&position);
                self.reset();
                is_loop
            })
            .filter(|is_loop| *is_loop)
            .count()
    }

    fn reset(&mut self) {
        self.guard = self.initial_guard.clone();
    }

    fn position_in_front(&self) -> Position {
        match self.guard.direction {
            Direction::Up    => Position { x: self.guard.position.x, y: self.guard.position.y - 1 },
            Direction::Down  => Position { x: self.guard.position.x, y: self.guard.position.y + 1 },
            Direction::Left  => Position { x: self.guard.position.x - 1, y: self.guard.position.y },
            Direction::Right => Position { x: self.guard.position.x + 1, y: self.guard.position.y },
        }
    }

    fn position_in_bounds(&self, position: &Position) -> bool {
        (0..self.width).contains(&position.x) && (0..self.height).contains(&position.y)
    }

    fn visit(&mut self) {
        match self.guard.visited.get_mut(&self.guard.position) {
            Some(directions) => {
                self.guard.will_loop = !directions.insert(self.guard.direction);
            }
            None => {
                let mut directions = HashSet::new();
                directions.insert(self.guard.direction);
                self.guard.visited.insert(self.guard.position, directions);
            }
        }
    }

    /**
        Traverse through the area and return true if traversal ends due to a loop
     */
    fn traverse(&mut self) -> bool {
        let mut new_position;
        while {
            new_position = self.position_in_front();
            self.position_in_bounds(&new_position)
        } {
            if self.obstacles.contains(&new_position) {
                self.guard.direction = match self.guard.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            } else {
                self.guard.position = new_position;
            }
            self.visit();
            if self.guard.will_loop {
                return true;
            }
        }
        false
    }
}

impl From<String> for Area {
    fn from(value: String) -> Self {
        let mut obstacles = HashSet::new();
        let mut guard = Guard { 
            position: Position { x: 0, y: 0 }, 
            direction: Direction::Up,
            visited: HashMap::<Position, HashSet<Direction>>::new(),
            will_loop: false
        };
        let mut x = 0;
        let mut y = 0;
        value.chars()
            .for_each(|char| {
                match char {
                    '#' => {
                        obstacles.insert(Position { x, y });
                        x += 1;
                    }
                    '\n' => {
                        x = 0;
                        y += 1;
                    }
                    '^' => {
                        guard.position = Position { x, y };
                        let mut directions = HashSet::new();
                        directions.insert(guard.direction);
                        guard.visited.insert(guard.position, directions);
                        x += 1;
                    }
                    _ => {
                        x += 1;
                    }
                }
            });
        Self::new(x, y + 1, guard, obstacles)
    }
}

#[cfg(test)]
mod test {
    // use super::*;

}