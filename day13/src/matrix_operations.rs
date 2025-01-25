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

    if a < 0.0 || b < 0.0 {
        return None;
    }

    if a.fract().abs() == 0.0 || b.fract().abs() == 0.0 {
        let a = a.trunc() as usize;
        let b = b.trunc() as usize;

        return Some((a, b));
    }

    return match round_if_zero_or_nine_pair(a, b) {
        Some((rounded_a, rounded_b)) => Some((rounded_a, rounded_b)),
        None => None,
    };
}

fn round_if_zero_or_nine_pair(a: f64, b: f64) -> Option<(usize, usize)> {
    let a_rounded = round_if_zero_or_nine(a);
    let b_rounded = round_if_zero_or_nine(b);

    match (a_rounded, b_rounded) {
        (Some(ra), Some(rb)) => Some((ra, rb)), // Both numbers rounded
        (Some(ra), _) => Some((ra, b as usize)), // Only `a` rounded
        (_, Some(rb)) => Some((a as usize, rb)), // Only `b` rounded
        _ => None,                              // Neither rounded
    }
}

fn round_if_zero_or_nine(number: f64) -> Option<usize> {
    let integer_part = number.trunc();
    let decimal_part = number - integer_part;

    // Check if the decimal part is close to zero or one (sequence of nines or zeros).
    if (decimal_part - 0.0).abs() < 1e-3 || (decimal_part - 1.0).abs() < 1e-3 {
        return Some(number.round() as usize);
    }

    None
}

// This worked for the solution of the challenge!!!
pub fn solve_by_substitution(eq1: (i64, i64, i64), eq2: (i64, i64, i64)) -> Option<(usize, usize)> {
    // eq1: (coefficient_a, coefficient_b, result)
    // eq2: (coefficient_a, coefficient_b, result)
    let (a1, b1, r1) = (eq1.0 as f64, eq1.1 as f64, eq1.2 as f64);
    let (a2, b2, r2) = (eq2.0 as f64, eq2.1 as f64, eq2.2 as f64);

    // From first equation: A = (r1 - b1*B) / a1
    // Substitute into second equation:
    // a2*((r1 - b1*B) / a1) + b2*B = r2

    // Check if first coefficient is zero
    if a1.abs() < 1e-10 {
        return None;
    }

    // Expanding the substitution:
    // (a2*r1 - a2*b1*B) / a1 + b2*B = r2
    // a2*r1 - a2*b1*B + a1*b2*B*a1 = r2*a1
    // a2*r1 + B*(a1*b2 - a2*b1) = r2*a1

    let b_coefficient = a1 * b2 - a2 * b1;

    // If B coefficient is zero and equations are inconsistent
    if b_coefficient.abs() < 1e-10 {
        if (a2 * r1 - r2 * a1).abs() < 1e-10 {
            // Dependent equations - technically infinite solutions
            return None;
        }
        return None;
    }

    // Solve for B
    let b = (r2 * a1 - a2 * r1) / b_coefficient;

    // Substitute back to find A
    let a = (r1 - b1 * b) / a1;

    if a.fract().abs() == 0.0 || b.fract().abs() == 0.0 {
        let a = a.trunc() as usize;
        let b = b.trunc() as usize;

        return Some((a, b));
    }

    return None;

    // Check if solution is reasonably close to integers
    // if (a.round() - a).abs() < 1e-10 && (b.round() - b).abs() < 1e-10 {
    //     return Some((a.round() as usize, b.round() as usize));
    // } else {
    //     return Some((a as usize, b as usize));
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitution_method_on_example_1() {
        // Example system: 94A + 22B = 8400, 34A + 67B = 5400
        let eq1 = (94, 22, 8400);
        let eq2 = (34, 67, 5400);

        match solve_by_substitution(eq1, eq2) {
            Some((a, b)) => {
                println!("Solution: A = {:.2}, B = {:.2}", a, b);
                assert_eq!(a * 94 + b * 22, 8400);
                assert_eq!(a * 34 + b * 67, 5400);
            }
            None => {
                println!("No unique solution exists.");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_substitution_method_on_example_2() {
        let eq1 = (26, 67, 12748);
        let eq2 = (66, 21, 12176);
        let result = solve_by_substitution(eq1, eq2);
        println!("{:?}", result);
        assert!(result.is_none());
    }

    #[test]
    fn test_substitution_method_on_example_3() {
        println!("Example 3");
        let eq1 = (17, 84, 7870);
        let eq2 = (86, 37, 6450);

        match solve_by_substitution(eq1, eq2) {
            Some((a, b)) => {
                println!("Solution: A = {:.2}, B = {:.2}", a, b);
                assert_eq!(a * 17 + b * 84, 7870);
                assert_eq!(a * 86 + b * 37, 6450);
            }
            None => {
                println!("No unique solution exists.");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_substitution_method_on_example_4() {
        let eq1 = (69, 27, 18641);
        let eq2 = (23, 71, 10279);
        assert!(solve_by_substitution(eq1, eq2).is_none());
    }

    #[test]
    fn test_substitution_method_on_example_1_part2() {
        // Example system: 94A + 22B = 8400, 34A + 67B = 5400
        let eq1 = (94, 22, 10000000008400);
        let eq2 = (34, 67, 10000000005400);
        assert!(solve_by_substitution(eq1, eq2).is_none());
    }

    #[test]
    fn test_substitution_method_on_example_2_part2() {
        let eq1 = (26, 67, 10000000012748);
        let eq2 = (66, 21, 10000000012176);
        
        match solve_by_substitution(eq1, eq2) {
            Some((a, b)) => {
                println!("Solution: A = {:.2}, B = {:.2}", a, b);
                assert_eq!(a * 26 + b * 67, 10000000012748);
                assert_eq!(a * 66 + b * 21, 10000000012176);
            }
            None => {
                println!("No unique solution exists.");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_substitution_method_on_example_3_part2() {
        let eq1 = (17, 84, 10000000007870);
        let eq2 = (86, 37, 10000000006450);
        assert!(solve_by_substitution(eq1, eq2).is_none());
    }

    #[test]
    fn test_substitution_method_on_example_4_part2() {
        let eq1 = (69, 27, 10000000018641);
        let eq2 = (23, 71, 10000000010279);
        match solve_by_substitution(eq1, eq2) {
            Some((a, b)) => {
                println!("Solution: A = {:.2}, B = {:.2}", a, b);
                assert_eq!(a * 69 + b * 27, 10000000018641);
                assert_eq!(a * 23 + b * 71, 10000000010279);
            }
            None => {
                println!("No unique solution exists.");
                assert!(false);
            }
        }
    }

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
