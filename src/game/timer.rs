//! Game timer implementation for tracking elapsed time.
//!
//! This module provides a simple timer utility for measuring and tracking
//! elapsed time during gameplay, AI calculations, or other time-sensitive
//! operations in the Othello game.

use std::time::{Duration, Instant};

/// A simple timer for tracking elapsed time with start/stop functionality.
///
/// The `Timer` struct provides basic stopwatch functionality, allowing you to
/// measure time intervals with the ability to start, stop, reset, and query
/// elapsed time. It's useful for timing AI moves, game sessions, or performance
/// measurements.
///
/// # State Management
///
/// The timer maintains internal state to track:
/// - Start time reference point
/// - Accumulated elapsed time when stopped
/// - Whether the timer is currently running or stopped
///
/// # Usage Patterns
///
/// - **Single Measurement**: Create, start, stop, read elapsed time
/// - **Multiple Intervals**: Reset and restart for new measurements
/// - **Real-time Monitoring**: Read elapsed time while timer is running
///
/// # Examples
///
/// ```rust
/// let mut timer = Timer::new();
/// timer.start();
/// // ... do some work ...
/// timer.stop();
/// println!("Operation took: {:?}", timer.elapsed());
/// ```
pub struct Timer {
    /// Reference point for when the timer was started
    start_time: Instant,
    /// Accumulated elapsed time when timer is stopped
    elapsed: Duration,
    /// Whether the timer is currently stopped (true) or running (false)
    stopped: bool,
}

impl Timer {
    /// Creates a new timer instance.
    ///
    /// This constructor initializes a new timer with the current time as
    /// the reference point. The timer starts in a stopped state with
    /// zero elapsed time accumulated.
    ///
    /// # Initial State
    ///
    /// - `start_time`: Set to current instant
    /// - `elapsed`: Zero duration
    /// - `stopped`: true (timer not running)
    ///
    /// # Usage Note
    ///
    /// After creation, call `start()` to begin timing or use the timer
    /// for immediate elapsed time measurement from the creation point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let timer = Timer::new();
    /// assert!(timer.elapsed().as_secs() == 0);
    /// ```
    pub fn new() -> Self {
        Timer {
            start_time: Instant::now(),
            elapsed: Duration::new(0, 0),
            stopped: false,
        }
    }

    /// Resets the timer to zero and stops it.
    ///
    /// This method resets all timer state to initial conditions:
    /// sets a new start time reference, clears accumulated elapsed time,
    /// and puts the timer in stopped state.
    ///
    /// # State Changes
    ///
    /// - `start_time`: Updated to current instant
    /// - `elapsed`: Reset to zero duration  
    /// - `stopped`: Set to true
    ///
    /// # Use Cases
    ///
    /// - Preparing for a new timing measurement
    /// - Clearing previous timing data
    /// - Resetting between different operations
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut timer = Timer::new();
    /// timer.start();
    /// // ... some time passes ...
    /// timer.reset();
    /// assert!(timer.elapsed().as_secs() == 0);
    /// ```
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.elapsed = Duration::new(0, 0);
        self.stopped = true;
    }

    /// Starts the timer from zero elapsed time.
    ///
    /// This method begins a new timing session by setting a fresh start
    /// time reference, clearing any accumulated elapsed time, and putting
    /// the timer in running state.
    ///
    /// # State Changes
    ///
    /// - `start_time`: Updated to current instant
    /// - `elapsed`: Reset to zero duration
    /// - `stopped`: Set to false (timer running)
    ///
    /// # Behavior
    ///
    /// Starting an already running timer will reset the elapsed time
    /// to zero and restart from the current moment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut timer = Timer::new();
    /// timer.start();
    /// // Timer is now running and measuring elapsed time
    /// ```
    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.elapsed = Duration::new(0, 0);
        self.stopped = false;
    }

    /// Stops the timer and preserves the elapsed time.
    ///
    /// This method stops the timer and captures the total elapsed time
    /// since the last start. The accumulated time is preserved and can
    /// be queried with `elapsed()`.
    ///
    /// # State Changes
    ///
    /// - `start_time`: Unchanged
    /// - `elapsed`: Updated to total time since start
    /// - `stopped`: Set to true
    ///
    /// # Behavior
    ///
    /// - If already stopped: No effect
    /// - If running: Captures elapsed time and stops
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut timer = Timer::new();
    /// timer.start();
    /// // ... do some work ...
    /// timer.stop();
    /// let duration = timer.elapsed(); // Get the measured time
    /// ```
    pub fn stop(&mut self) {
        self.elapsed = self.start_time.elapsed();
        self.stopped = true;
    }

    /// Returns the current elapsed time.
    ///
    /// This method returns the elapsed time based on the timer's current state:
    /// - If stopped: returns the preserved elapsed time from when stopped
    /// - If running: returns the live elapsed time since start
    ///
    /// # Returns
    ///
    /// `Duration` representing the elapsed time since the timer was started
    ///
    /// # Real-time vs Frozen
    ///
    /// - **Running timer**: Returns current elapsed time (updates continuously)
    /// - **Stopped timer**: Returns frozen elapsed time from when stopped
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut timer = Timer::new();
    /// timer.start();
    /// std::thread::sleep(std::time::Duration::from_millis(100));
    /// let elapsed = timer.elapsed();
    /// assert!(elapsed.as_millis() >= 100);
    /// ```
    pub fn elapsed(&self) -> Duration {
        if self.stopped {
            self.elapsed
        } else {
            self.start_time.elapsed()
        }
    }
}
