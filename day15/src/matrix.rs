use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Robot {
    pub row: usize,
    pub col: usize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Direction {
    pub row: isize,
    pub col: isize,
}

impl Robot {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn move_robot(
        &mut self,
        warehouse_map: &mut HashMap<(usize, usize), char>,
        direction: Direction,
    ) -> Self {
        let new_robot_position = Self {
            row: (self.row as isize + direction.row) as usize,
            col: (self.col as isize + direction.col) as usize,
        };

        let new_position_value = warehouse_map
            .get_mut(&(new_robot_position.row, new_robot_position.col))
            .unwrap();

        match *new_position_value {
            '.' => {
                *new_position_value = '@';
                warehouse_map
                    .entry((self.row, self.col))
                    .and_modify(|v| *v = '.');
                *self = new_robot_position;
            }
            'O' => {
                let mut empty_or_wall_position = Self {
                    row: (new_robot_position.row as isize + direction.row) as usize,
                    col: (new_robot_position.col as isize + direction.col) as usize,
                };
                while warehouse_map
                    .get(&(empty_or_wall_position.row, empty_or_wall_position.col))
                    .unwrap()
                    == &'O'
                {
                    empty_or_wall_position.row =
                        (empty_or_wall_position.row as isize + direction.row) as usize;
                    empty_or_wall_position.col =
                        (empty_or_wall_position.col as isize + direction.col) as usize;
                }

                match warehouse_map
                    .get_mut(&(empty_or_wall_position.row, empty_or_wall_position.col))
                    .unwrap()
                {
                    '.' => {
                        *warehouse_map
                            .get_mut(&(empty_or_wall_position.row, empty_or_wall_position.col))
                            .unwrap() = 'O';
                        *warehouse_map
                            .get_mut(&(new_robot_position.row, new_robot_position.col))
                            .unwrap() = '@';
                        *warehouse_map.get_mut(&(self.row, self.col)).unwrap() = '.';

                        *self = new_robot_position;
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        *self
    }

    pub fn move_robot_part2(
        &mut self,
        warehouse_map: &mut HashMap<(usize, usize), char>,
        direction: Direction,
    ) -> Self {
        let new_robot_position = Self {
            row: (self.row as isize + direction.row) as usize,
            col: (self.col as isize + direction.col) as usize,
        };

        let new_position_value = warehouse_map
            .get_mut(&(new_robot_position.row, new_robot_position.col))
            .unwrap();

        match *new_position_value {
            '.' => {
                *new_position_value = '@';
                warehouse_map
                    .entry((self.row, self.col))
                    .and_modify(|v| *v = '.');
                *self = new_robot_position;
            }
            '[' | ']' => match direction {
                Direction::LEFT | Direction::RIGHT => {
                    let mut empty_or_wall_position = Self {
                        row: (new_robot_position.row as isize + direction.row) as usize,
                        col: (new_robot_position.col as isize + direction.col) as usize,
                    };

                    let mut positions_to_move_stack: Vec<(usize, usize)> = Vec::new();
                    positions_to_move_stack.push((empty_or_wall_position.row, empty_or_wall_position.col));

                    while let Some(&ch) = warehouse_map.get(&(empty_or_wall_position.row, empty_or_wall_position.col)) {
                        if ch == '[' || ch == ']' {
                            empty_or_wall_position.row =
                                (empty_or_wall_position.row as isize + direction.row) as usize;
                            empty_or_wall_position.col =
                                (empty_or_wall_position.col as isize + direction.col) as usize;
                            positions_to_move_stack.push((empty_or_wall_position.row, empty_or_wall_position.col));
                        } else {
                            break;
                        }
                    }

                    if let Some(ch) = warehouse_map.get_mut(&(empty_or_wall_position.row, empty_or_wall_position.col)) {
                        if *ch == '.' {
                            while !positions_to_move_stack.is_empty() {
                                let (row, col) = positions_to_move_stack.pop().unwrap();
                                let (previous_row, previous_col) = positions_to_move_stack.last().unwrap();
                                *warehouse_map.get_mut(&(row, col)).unwrap() = *warehouse_map.get(&(*previous_row, *previous_col)).unwrap();
                            }
                        }
                    }
                }
                Direction::TOP | Direction::BOTTOM => {}
                _ => (),
            },
            _ => (),
        }

        *self
    }
}

impl Direction {
    pub const TOP: Self = Self { row: -1, col: 0 };
    pub const BOTTOM: Self = Self { row: 1, col: 0 };
    pub const LEFT: Self = Self { row: 0, col: -1 };
    pub const RIGHT: Self = Self { row: 0, col: 1 };
}
