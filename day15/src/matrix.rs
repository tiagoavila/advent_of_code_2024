use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Direction {
    pub row: isize,
    pub col: isize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Robot {
    pub row: usize,
    pub col: usize,
    pub is_stuck: bool,
    pub current_direction: Direction,
}

impl Robot {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            is_stuck: false,
            current_direction: Direction::RIGHT,
        }
    }

    pub fn move_robot(
        &mut self,
        warehouse_map: &mut HashMap<(usize, usize), char>,
        direction: Direction,
    ) -> Self {
        let new_robot_position = Self {
            row: (self.row as isize + direction.row) as usize,
            col: (self.col as isize + direction.col) as usize,
            is_stuck: false,
            current_direction: direction,
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
                let mut neighbor_position = Self {
                    row: (new_robot_position.row as isize + direction.row) as usize,
                    col: (new_robot_position.col as isize + direction.col) as usize,
                    is_stuck: false,
                    current_direction: direction,
                };
                while warehouse_map
                    .get(&(neighbor_position.row, neighbor_position.col))
                    .unwrap()
                    == &'O'
                {
                    neighbor_position.row =
                        (neighbor_position.row as isize + direction.row) as usize;
                    neighbor_position.col =
                        (neighbor_position.col as isize + direction.col) as usize;
                }

                match warehouse_map
                    .get_mut(&(neighbor_position.row, neighbor_position.col))
                    .unwrap()
                {
                    '.' => {
                        *warehouse_map
                            .get_mut(&(neighbor_position.row, neighbor_position.col))
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
        if self.is_stuck && direction == self.current_direction {
            return *self;
        }

        self.current_direction = direction;

        let new_robot_position = Self {
            row: (self.row as isize + direction.row) as usize,
            col: (self.col as isize + direction.col) as usize,
            is_stuck: false,
            current_direction: direction,
        };

        let new_position_value = warehouse_map
            .get(&(new_robot_position.row, new_robot_position.col))
            .unwrap();

        match *new_position_value {
            '.' => {
                self.update_position(warehouse_map, new_robot_position);
            }
            '[' | ']' => match direction {
                Direction::LEFT | Direction::RIGHT => {
                    self.process_horizontal_movement_part2(
                        warehouse_map,
                        direction,
                        new_robot_position,
                    );
                }
                Direction::TOP | Direction::BOTTOM => {
                    self.process_vertical_movement_part2(
                        warehouse_map,
                        direction,
                        new_robot_position,
                    );
                }
                _ => (),
            },
            _ => (),
        }

        *self
    }

    fn update_position(
        &mut self,
        warehouse_map: &mut HashMap<(usize, usize), char>,
        new_robot_position: Robot,
    ) {
        warehouse_map
            .entry((new_robot_position.row, new_robot_position.col))
            .and_modify(|v| *v = '@');

        warehouse_map
            .entry((self.row, self.col))
            .and_modify(|v| *v = '.');

        *self = new_robot_position;
    }

    fn process_horizontal_movement_part2(
        &mut self,
        warehouse_map: &mut HashMap<(usize, usize), char>,
        direction: Direction,
        new_robot_position: Robot,
    ) {
        let mut neighbor_position = Self {
            row: (new_robot_position.row as isize + direction.row) as usize,
            col: (new_robot_position.col as isize + direction.col) as usize,
            is_stuck: false,
            current_direction: direction,
        };

        let mut positions_to_move_stack: Vec<(usize, usize)> = Vec::new();
        positions_to_move_stack.push((new_robot_position.row, new_robot_position.col));

        while let Some(&ch) = warehouse_map.get(&(neighbor_position.row, neighbor_position.col)) {
            if ch == '[' || ch == ']' {
                positions_to_move_stack.push((neighbor_position.row, neighbor_position.col));

                neighbor_position.row = (neighbor_position.row as isize + direction.row) as usize;
                neighbor_position.col = (neighbor_position.col as isize + direction.col) as usize;
            } else {
                break;
            }
        }

        if let Some(ch) = warehouse_map.get(&(neighbor_position.row, neighbor_position.col)) {
            if *ch == '.' {
                positions_to_move_stack.push((neighbor_position.row, neighbor_position.col));

                while !positions_to_move_stack.is_empty() {
                    let (row, col) = positions_to_move_stack.pop().unwrap();
                    match positions_to_move_stack.last() {
                        Some((previous_row, previous_col)) => {
                            *warehouse_map.get_mut(&(row, col)).unwrap() =
                                *warehouse_map.get(&(*previous_row, *previous_col)).unwrap();
                        }
                        None => {
                            *warehouse_map.get_mut(&(row, col)).unwrap() = '@';
                        }
                    }
                }

                warehouse_map
                    .entry((self.row, self.col))
                    .and_modify(|v| *v = '.');
                *self = new_robot_position;
            }
        }
    }

    fn process_vertical_movement_part2(
        &mut self,
        warehouse_map: &mut HashMap<(usize, usize), char>,
        direction: Direction,
        new_robot_position: Robot,
    ) {
        let mut positions_to_move_stack: Vec<(usize, usize)> = Vec::new();

        positions_to_move_stack.push((new_robot_position.row, new_robot_position.col));

        if warehouse_map
            .get(&(new_robot_position.row, new_robot_position.col))
            .unwrap()
            == &'['
        {
            positions_to_move_stack.push((new_robot_position.row, new_robot_position.col + 1));
        } else {
            positions_to_move_stack.push((new_robot_position.row, new_robot_position.col - 1));
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from(positions_to_move_stack.clone());

        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();

            let neighbor_position = Self {
                row: (row as isize + direction.row) as usize,
                col: (col as isize + direction.col) as usize,
                is_stuck: false,
                current_direction: direction,
            };

            if let Some(&ch) = warehouse_map.get(&(neighbor_position.row, neighbor_position.col)) {
                match ch {
                    '[' => {
                        positions_to_move_stack
                            .push((neighbor_position.row, neighbor_position.col));
                        queue.push_back((neighbor_position.row, neighbor_position.col));

                        let other_side_of_box = neighbor_position.col + 1;
                        positions_to_move_stack.push((neighbor_position.row, other_side_of_box));
                        queue.push_back((neighbor_position.row, other_side_of_box));
                    }
                    ']' => {
                        positions_to_move_stack
                            .push((neighbor_position.row, neighbor_position.col));
                        queue.push_back((neighbor_position.row, neighbor_position.col));

                        let other_side_of_box = neighbor_position.col - 1;
                        positions_to_move_stack.push((neighbor_position.row, other_side_of_box));
                        queue.push_back((neighbor_position.row, other_side_of_box));
                    }
                    '#' => {
                        self.is_stuck = true;

                        break;
                    }
                    _ => (),
                }
            }
        }

        if self.is_stuck {
            return;
        }

        while !positions_to_move_stack.is_empty() {
            let (box_row, box_col) = positions_to_move_stack.pop().unwrap();
            let box_value = warehouse_map.get(&(box_row, box_col)).unwrap().clone();
            let (new_row, new_col) = (box_row as isize + direction.row, box_col as isize + direction.col);

            warehouse_map
                .entry((new_row as usize, new_col as usize))
                .and_modify(|v| *v = box_value);

            warehouse_map
                .entry((box_row, box_col))
                .and_modify(|v| *v = '.');

        }

        self.update_position(warehouse_map, new_robot_position);
    }
}

impl Direction {
    pub const TOP: Self = Self { row: -1, col: 0 };
    pub const BOTTOM: Self = Self { row: 1, col: 0 };
    pub const LEFT: Self = Self { row: 0, col: -1 };
    pub const RIGHT: Self = Self { row: 0, col: 1 };
}
