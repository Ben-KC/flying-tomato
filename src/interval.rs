//! # Interval
//!
//! Contains an enum for representing the current timer interval

use std::mem;

/// Enum indicating the current interval
pub enum Interval {
    /// Work interval
    Work,
    /// Break interval
    Break,
}

impl Interval {
    /// Swap in place from a [Work](Interval::Work) to a [Break](Interval::Break) interval, or vice versa
    pub fn switch(&mut self) {
        match self {
            Self::Work => {
                let _ = mem::replace(self, Self::Break);
            }
            Self::Break => {
                let _ = mem::replace(self, Self::Work);
            }
        }
    }
}
