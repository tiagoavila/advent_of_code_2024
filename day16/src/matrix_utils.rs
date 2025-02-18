#[derive(Debug)]
pub struct MatrixUtils {
    rows: usize,
    cols: usize,
}

#[derive(Debug, PartialEq)]
pub enum Cell {
   Tile,
   Wall 
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl MatrixUtils {
    /// Creates a new MatrixUtils instance
    pub fn new(rows: usize, cols: usize) -> Self {
        MatrixUtils { rows, cols }
    }

    /// Converts a matrix to a flat array
    pub fn matrix_to_array(&self, matrix: Vec<Vec<char>>) -> Option<Vec<char>> {
        // Validate matrix dimensions
        if matrix.len() != self.rows || matrix.iter().any(|row| row.len() != self.cols) {
            return None;
        }

        let mut result = Vec::with_capacity(self.rows * self.cols);
        for row in matrix {
            result.extend(row);
        }

        Some(result)
    }

    /// Converts row and column indices to array index
    pub fn coords_to_index(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(row * self.cols + col)
    }

    /// Converts array index to row and column indices
    pub fn index_to_coords(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.rows * self.cols {
            return None;
        }
        let row = index / self.cols;
        let col = index % self.cols;
        Some((row, col))
    }

    /// Gets the dimensions of the matrix
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_to_array() {
        let utils = MatrixUtils::new(2, 3);
        let matrix = vec![vec!['1', '2', '3'], vec!['4', '5', '6']];

        assert_eq!(
            utils.matrix_to_array(matrix),
            Some(vec!['1', '2', '3', '4', '5', '6'])
        );
    }

    #[test]
    fn test_invalid_matrix() {
        let utils = MatrixUtils::new(2, 3);
        let invalid_matrix = vec![
            vec!['1', '2'], // Wrong number of columns
            vec!['4', '5', '6'],
        ];

        assert_eq!(utils.matrix_to_array(invalid_matrix), None);
    }

    #[test]
    fn test_coords_to_index() {
        let utils = MatrixUtils::new(3, 4);

        assert_eq!(utils.coords_to_index(0, 0), Some(0));
        assert_eq!(utils.coords_to_index(1, 2), Some(6));
        assert_eq!(utils.coords_to_index(2, 3), Some(11));
        assert_eq!(utils.coords_to_index(3, 0), None); // Invalid row
        assert_eq!(utils.coords_to_index(0, 4), None); // Invalid column
    }

    #[test]
    fn test_index_to_coords() {
        let utils = MatrixUtils::new(3, 4);

        assert_eq!(utils.index_to_coords(0), Some((0, 0)));
        assert_eq!(utils.index_to_coords(6), Some((1, 2)));
        assert_eq!(utils.index_to_coords(11), Some((2, 3)));
        assert_eq!(utils.index_to_coords(12), None); // Invalid index
    }
}
