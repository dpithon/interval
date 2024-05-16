//! ## Introduction
//!
//! This crate implement interval definition and operations
//! It manages Empty set, Infinity set, Singleton and Intervals in a unique structure `Interval`
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
//! Constants are provided for empty or infinity sets. An associated function is dedicated to
//! create singleton.
//!
//! ```
//! use interval::{Interval, EMPTY, INFINITY};
//!
//! let e = EMPTY;    // ∅, equivalent to Interval::new(Open(0.), Open(0.))
//! let f = INFINITY; // (-∞,+∞), equivalent to Interval::new(Unbound, Unbound)
//!
//! let s = Interval::singleton(42.); // {42}, equivalent to Interval::new(Closed(42.), Closed(42.))
//! ```
//!
//!

mod interval;

pub use interval::Bound::{Closed, Open, Unbound};
pub use interval::{Interval, EMPTY, INFINITY};
