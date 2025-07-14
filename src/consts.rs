pub const SIZE: usize = 8;
pub const MAX_DEPTH: usize = 5; // Maximum depth for the AI search algorithms

// Qlearning
pub const LAMBDA_LEARN: f64 = 0.8; // Learning rate for Q-learning
pub const GAMMA: f64 = 0.99; // Discount factor for future rewards
pub const EPSILON: f64 = 1.0; // INITIAL Exploration rate for epsilon-greedy strategy
pub const QLEARNING_MAX_EPOCHS: usize = 10000;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_even() {
        assert_eq!(SIZE % 2, 0);
    }
}
