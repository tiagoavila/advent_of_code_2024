pub fn gaussian_elimination(matrix: &mut Vec<Vec<f64>>) -> Option<(usize, usize)> {
    let pivot_row_1 = matrix[0][0];

    // Make the first pivot equals to 1 by dividing the row by the pivot, the pivot is the first element of the row
    if pivot_row_1 == 0.0 {
        return None;
    }

    for i in 0..3 {
        matrix[0][i] = matrix[0][i] / pivot_row_1;
    }

    // Eliminate the first entry of the second row by subtracting the first row multiplied by the first entry of the second row
    let multiplier = matrix[1][0];

    for i in 0..3 {
        matrix[1][i] = matrix[1][i] - multiplier * matrix[0][i];
    }

    // Make the second pivot equals to 1 by dividing the row by the pivot, the pivot is the second element of the second row
    let pivot_row_2 = matrix[1][1];

    if pivot_row_2 == 0.0 {
        return None;
    }

    for i in 0..3 {
        matrix[1][i] = matrix[1][i] / pivot_row_2;
    }

    // Eliminate the second entry of the first row by subtracting the second row multiplied by the second entry of the first row
    let multiplier = matrix[0][1];

    for i in 0..3 {
        matrix[0][i] = matrix[0][i] - multiplier * matrix[1][i];
    }

    let a = matrix[0][2];
    let b = matrix[1][2];
    let epsilon: f64 = 1e-12;

    if a.fract().abs() > epsilon || b.fract().abs() > epsilon {
        return None;
    }

    let a = a.trunc() as usize;
    let b = b.trunc() as usize;

    return Some((a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_elimination_on_example_1() {
        let mut matrix = vec![vec![94.0, 22.0, 8400.0], vec![34.0, 67.0, 5400.0]];
        let result: Option<(usize, usize)> = Some((80, 40));
        assert_eq!(gaussian_elimination(&mut matrix), result);
    }

    #[test]
    fn test_gaussian_elimination_on_example_2() {
        let mut matrix = vec![vec![26.0, 67.0, 12748.0], vec![66.0, 21.0, 12176.0]];
        let result: Option<(usize, usize)> = None;
        assert_eq!(gaussian_elimination(&mut matrix), result);
    }

    #[test]
    fn test_gaussian_elimination_on_example_3() {
        let mut matrix = vec![vec![17.0, 84.0, 7870.0], vec![86.0, 37.0, 6450.0]];
        let result: Option<(usize, usize)> = Some((38, 86));
        assert_eq!(gaussian_elimination(&mut matrix), result);
    }

    #[test]
    fn test_gaussian_elimination_on_example_4() {
        let mut matrix = vec![vec![69.0, 27.0, 18641.0], vec![23.0, 71.0, 10279.0]];
        let result: Option<(usize, usize)> = None;
        assert_eq!(gaussian_elimination(&mut matrix), result);
    }
}
