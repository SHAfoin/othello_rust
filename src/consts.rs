pub const SIZE: usize = 8;
pub const MAX_DEPTH: usize = 5; // Maximum depth for the AI search algorithms
pub const ULTRA_THREADING: bool = false; // Enable or disable ultra threading for AI calculations

pub const MATRIX_A: [[isize; SIZE]; SIZE] = [
    [100, -20, 10, 5, 5, 10, -20, 100],
    [-20, -50, -2, -2, -2, -2, -50, -20],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [-20, -50, -2, -2, -2, -2, -50, -20],
    [100, -20, 10, 5, 5, 10, -20, 100],
];

pub const MATRIX_B: [[isize; SIZE]; SIZE] = [
    [500, -150, 30, 10, 10, 30, -150, 500],
    [-150, -250, 0, 0, 0, 0, -250, -150],
    [30, 0, 1, 2, 2, 1, 0, 30],
    [10, 0, 2, 16, 16, 2, 0, 10],
    [10, 0, 2, 16, 16, 2, 0, 10],
    [30, 0, 1, 2, 2, 1, 0, 30],
    [-150, -250, 0, 0, 0, 0, -250, -150],
    [500, -150, 30, 10, 10, 30, -150, 500],
];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_even() {
        assert_eq!(SIZE % 2, 0);
    }
}
