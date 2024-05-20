//! ## Introduction
//!
//! This crate implement interval definition and operations
//! It manages Empty set, Infinity set, Intervals through `Interval` enum
//!
//! ## Interval creation
//!
//! Variants `Closed`, `Open` and `Unbound` are used to define endpoints of interval.
//!
//! ```
//! use interval::{Interval, Closed, Open, Unbound};
//!
//! let a = Interval::new(Closed(0.), Closed(42.)); // [0, 42]
//! let b = Interval::new(Open(-42.), Open(42.)); // (-42, 42)
//! let c = Interval::new(Unbound, Closed(42.)); // (-∞, 42]
//! ```
//! ## Singleton, Empty or Infinity set
//!
//! Variants are provided for empty or infinity sets. An associated function is dedicated to
//! create singleton.
//!
//! ```
//! use interval::{Interval, EMPTY, INFINITY};
//!
//! let e = EMPTY;    // ∅
//! let f = INFINITY; // (-∞,+∞)
//!
//! let s = Interval::singleton(42.); // {42}, equivalent to Interval::new(Closed(42.), Closed(42.))
//! ```
//!
//!

mod interval;
mod interval_set;

pub use interval::{Closed, Interval, Open, Unbound, EMPTY, INFINITY};
pub use interval_set::IntervalSet;
