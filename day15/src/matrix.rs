use std::collections::HashMap;

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

    pub fn push(&self, warehouse_map: &HashMap<(usize, usize), char>, direction: Direction) -> Self {
        let new_position = Self {
            row: (self.row as isize + direction.row) as usize,
            col: (self.col as isize + direction.col) as usize,
        };

        if warehouse_map.contains_key(&(new_position.row, new_position.col)) {
            new_position
        } else {
            self
        }
    }
}

impl Direction {
    pub const TOP: Self = Self { row: -1, col: 0 };
    pub const BOTTOM: Self = Self { row: 1, col: 0 };
    pub const LEFT: Self = Self { row: 0, col: -1 };
    pub const RIGHT: Self = Self { row: 0, col: 1 };
}

pub static DIRECTION_MAP: [(char, Direction); 4] = [
    ('^', Direction::TOP),
    ('v', Direction::BOTTOM),
    ('>', Direction::RIGHT),
    ('<', Direction::LEFT),
];