//! Global constants and configuration values for the Othello game.
//!
//! This module defines all the constant values used throughout the Othello
//! game implementation, including board dimensions, AI parameters, and
//! Q-Learning configuration. These constants ensure consistency across
//! the entire application and provide a central location for tuning
//! game behavior and AI performance.

/// The size of the Othello game board (8x8 grid).
///
/// This constant defines the dimensions of the standard Othello board.
/// The game is played on an 8x8 grid, which is the official board size
/// for Othello/Reversi. This value is used throughout the codebase for:
/// 
/// - Array dimensioning for the game board
/// - Boundary checking for move validation
/// - Loop iterations for board traversal
/// - UI layout calculations for grid rendering
///
/// # Standard Othello Rules
///
/// The 8x8 board size is required by official Othello rules and provides:
/// - 64 total positions for disc placement
/// - Standard starting position with 4 center discs
/// - Balanced gameplay with appropriate strategic depth
/// - Compatibility with standard Othello notation (A1-H8)
pub const SIZE: usize = 8;

/// Maximum search depth for AI tree-based algorithms.
///
/// This constant limits how many moves ahead the AI algorithms will
/// analyze when using tree-search methods like MinMax and Alpha-Beta
/// pruning. The depth directly affects:
///
/// - **AI strength**: Higher depth generally means stronger play
/// - **Computation time**: Exponential increase with each additional level
/// - **Memory usage**: Deeper searches require more memory for game trees
/// - **User experience**: Balance between AI quality and responsiveness
///
/// # Performance Considerations
///
/// A depth of 5 provides:
/// - Good strategic play without excessive computation time
/// - Reasonable response times on modern hardware
/// - Balanced difficulty for human players
/// - Manageable memory requirements for game tree storage
///
/// # Algorithm Compatibility
///
/// This depth limit applies to:
/// - MinMax algorithm implementations
/// - Alpha-Beta pruning optimizations
/// - Any tree-based search algorithms
pub const MAX_DEPTH: usize = 5; // Maximum depth for the AI search algorithms

// Qlearning

/// Learning rate parameter for Q-Learning algorithm.
///
/// This constant controls how quickly the Q-Learning AI adapts to new
/// information during training. The learning rate (alpha) determines
/// the weight given to new experiences versus existing knowledge.
///
/// # Value Significance
///
/// - **Range**: 0.0 to 1.0 (0.8 = high learning rate)
/// - **High values**: Rapid adaptation, may be unstable
/// - **Low values**: Slow learning, more stable convergence
/// - **0.8**: Aggressive learning with good convergence properties
///
/// # Learning Behavior
///
/// With LAMBDA_LEARN = 0.8:
/// - New experiences have significant impact on Q-values
/// - Relatively fast adaptation to changing strategies
/// - Good balance between learning speed and stability
/// - Suitable for training sessions with limited epochs
pub const LAMBDA_LEARN: f64 = 0.8; // Learning rate for Q-learning

/// Discount factor for future rewards in Q-Learning.
///
/// This constant determines how much importance the Q-Learning algorithm
/// places on future rewards compared to immediate rewards. It affects
/// the AI's strategic planning and long-term thinking.
///
/// # Value Significance
///
/// - **Range**: 0.0 to 1.0 (0.99 = high future value)
/// - **High values**: Long-term strategic planning
/// - **Low values**: Focus on immediate gains
/// - **0.99**: Strong emphasis on future position value
///
/// # Strategic Impact
///
/// With GAMMA = 0.99:
/// - AI considers long-term consequences of moves
/// - Encourages strategic positioning over immediate gains
/// - Promotes stable, thoughtful gameplay
/// - Suitable for complex games like Othello where position matters
pub const GAMMA: f64 = 0.99; // Discount factor for future rewards

/// Initial exploration rate for epsilon-greedy strategy in Q-Learning.
///
/// This constant sets the starting probability for random exploration
/// versus exploitation of learned knowledge. It balances learning new
/// strategies with using existing knowledge effectively.
///
/// # Value Significance
///
/// - **Range**: 0.0 to 1.0 (1.0 = 100% exploration initially)
/// - **High values**: More random exploration early in training
/// - **Low values**: More exploitation of current knowledge
/// - **1.0**: Maximum exploration at training start
///
/// # Training Strategy
///
/// With EPSILON = 1.0:
/// - Training begins with complete exploration
/// - AI tries many different strategies initially
/// - Epsilon typically decreases over time during training
/// - Ensures comprehensive exploration of the strategy space
pub const EPSILON: f64 = 1.0; // INITIAL Exploration rate for epsilon-greedy strategy

/// Maximum number of training epochs for Q-Learning sessions.
///
/// This constant defines the upper limit for Q-Learning training iterations.
/// Each epoch represents a complete training session or game played during
/// the learning process.
///
/// # Training Scope
///
/// With 10,000 epochs:
/// - Extensive training for thorough learning
/// - Sufficient iterations for strategy convergence
/// - Balanced between learning quality and training time
/// - Appropriate for developing competitive AI players
///
/// # Performance Considerations
///
/// - **Training time**: Each epoch requires game simulation time
/// - **Learning quality**: More epochs generally improve AI performance
/// - **Computational cost**: 10,000 epochs require significant processing
/// - **Convergence**: Most learning typically occurs in first few thousand epochs
pub const QLEARNING_MAX_EPOCHS: usize = 10000;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_even() {
        assert_eq!(SIZE % 2, 0);
    }
}
