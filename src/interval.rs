mod bound;
mod left;
mod right;

use bound::Bound;
use left::Left;
use right::Right;

pub use Bound::{Closed, Open, Unbound};

//use bounds::I{self, Closed, LeftOpen, Bound::Unbound, Bound::Unbound, RightOpen};

use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Empty,
    Infinity,
    Ival(Left, Right),
}

use Interval::*;

pub enum Union {
    Single(Interval),
    Couple(Interval, Interval),
}

impl Display for Union {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Union::Single(i) => write!(f, "{i}"),
            Union::Couple(a, b) => write!(f, "{a} U {b}"),
        }
    }
}

pub const INFINITY: Interval = Interval::Infinity;
pub const EMPTY: Interval = Interval::Empty;

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Empty => write!(f, "∅"),
            Infinity => write!(f, "]-∞,+∞["),
            Ival(Left(Closed(a)), Right(Closed(b))) if a == b => write!(f, "{{{a:5.2}}}"),
            Ival(a, b) => write!(f, "{a},{b}"),
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Empty, Empty) => true,
            (Infinity, Infinity) => true,
            (Ival(a1, a2), Ival(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
}

impl Interval {
    /// Build interval from given bounds
    ///
    /// # Returns
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use interval::{Interval, Open, Closed, Unbound};
    ///
    /// let a = Interval::new(Open(42.), Closed(43.));
    /// let b = Interval::new(Unbound, Unbound);
    /// let c = Interval::singleton(42.);
    ///
    /// assert_eq!(format!("{a}"), "]42.00,43.00]");
    /// assert_eq!(format!("{b}"), "]-∞,+∞[");
    /// assert_eq!(format!("{c}"), "{42.00}");
    /// ```
    ///
    pub fn new(b1: Bound, b2: Bound) -> Self {
        let b1 = Left(b1);
        let b2 = Right(b2);

        if b2 < b1 {
            Empty
        } else if (b1, b2) == (Left(Unbound), Right(Unbound)) {
            Infinity
        } else {
            Ival(b1, b2)
        }
    }

    pub fn singleton(k: f64) -> Self {
        Ival(Left(Closed(k)), Right(Closed(k)))
    }

    pub fn is_singleton(&self) -> bool {
        match self {
            Ival(Left(Closed(k1)), Right(Closed(k2))) => k1 == k2,
            _ => false,
        }
    }

    pub fn is_empty(self) -> bool {
        self == Empty
    }

    pub fn union(self, other: Interval) -> Union {
        match (self, other) {
            (a, Empty) | (Empty, a) => Union::Single(a),
            (Infinity, _) | (_, Infinity) => Union::Single(Infinity),

            (Ival(a1, a2), Ival(b1, b2)) => {
                if self.overlap(other) || self.adhere_to(other) {
                    Union::Single(Ival(a1.min(b1), a2.max(b2)))
                } else if b1 > a2 {
                    Union::Couple(self, other)
                } else {
                    Union::Couple(other, self)
                }
            }
        }
    }

    /// Check if intervals overlap
    ///
    /// Note that `Empty` overlap nothing.
    ///
    fn overlap(self, other: Interval) -> bool {
        match (self, other) {
            (_, Empty) | (Empty, _) => false,
            (Infinity, _) | (_, Infinity) => true,
            (Ival(a1, a2), Ival(b1, b2)) => b2 >= a1 && b1 <= a2,
        }
    }

    /// Check if interval endpoints could rejoin (ie ]2 and (2, (2 and 2] ...)
    ///
    fn adhere_to(self, other: Interval) -> bool {
        match (self, other) {
            (_, Empty) | (Empty, _) => false,
            (Infinity, _) | (_, Infinity) => false,
            (Ival(a1, a2), Ival(b1, b2)) => {
                a1.closure() == b2.closure() || b1.closure() == a2.closure()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overlap_1() {
        let a = Interval::new(Unbound, Unbound);
        let b = Interval::new(Unbound, Unbound);

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_2() {
        let a = Interval::new(Unbound, Unbound);
        let b = EMPTY;

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_3() {
        let a = EMPTY;
        let b = Interval::new(Unbound, Unbound);

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_4() {
        let a = Interval::new(Closed(42.), Closed(43.));
        let b = Interval::new(Unbound, Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_5() {
        let a = Interval::new(Unbound, Unbound);
        let b = Interval::new(Closed(42.), Closed(43.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_6() {
        let a = Interval::new(Closed(42.), Open(43.));
        let b = Interval::new(Unbound, Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_7() {
        let a = Interval::new(Unbound, Unbound);
        let b = Interval::new(Closed(42.), Open(43.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_8() {
        let a = Interval::new(Open(42.), Open(43.));
        let b = Interval::new(Unbound, Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_9() {
        let a = Interval::new(Unbound, Unbound);
        let b = Interval::new(Open(42.), Open(43.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_10() {
        let a = Interval::new(Unbound, Open(43.));
        let b = Interval::new(Unbound, Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_11() {
        let a = Interval::new(Unbound, Unbound);
        let b = Interval::new(Open(42.), Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_12() {
        let a = EMPTY;
        let b = Interval::new(Unbound, Unbound);

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_13() {
        let a = EMPTY;
        let b = EMPTY;

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_14() {
        let a = Interval::new(Closed(42.), Closed(52.));
        let b = Interval::new(Closed(42.), Closed(52.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_15() {
        let a = Interval::new(Closed(42.), Closed(52.));
        let b = Interval::new(Open(42.), Open(52.));

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_16() {
        let a = Interval::new(Closed(42.), Closed(52.));
        let b = Interval::new(Open(42.), Open(52.));

        assert!(b.overlap(a));
    }

    #[test]
    fn test_overlap_17() {
        let a = Interval::new(Open(42.), Closed(52.));
        let b = Interval::new(Open(42.), Open(52.));

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_18() {
        let a = Interval::new(Open(42.), Closed(52.));
        let b = Interval::new(Open(42.), Open(52.));

        assert!(b.overlap(a));
    }

    #[test]
    fn test_overlap_19() {
        let a = Interval::new(Closed(42.), Open(52.));
        let b = Interval::new(Open(42.), Open(52.));

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_20() {
        let a = Interval::new(Closed(42.), Open(52.));
        let b = Interval::new(Open(42.), Open(52.));

        assert!(b.overlap(a));
    }

    #[test]
    fn test_overlap_21() {
        let a = Interval::new(Open(42.), Open(52.));
        let b = Interval::new(Open(42.), Open(52.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_22() {
        let a = Interval::new(Unbound, Closed(42.));
        let b = Interval::new(Open(42.), Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_23() {
        let a = Interval::new(Unbound, Open(42.));
        let b = Interval::new(Open(42.), Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_24() {
        let a = Interval::new(Closed(52.), Unbound);
        let b = Interval::new(Open(42.), Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_25() {
        let a = Interval::new(Open(52.), Unbound);
        let b = Interval::new(Open(42.), Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_union_1() {
        assert!(matches!(EMPTY.union(EMPTY), Union::Single(Empty)));
    }

    #[test]
    fn test_union_2() {
        let i = Interval::new(Open(42.), Closed(43.));
        assert!(match i.union(EMPTY) {
            Union::Single(Ival(Left(Open(k1)), Right(Closed(k2)))) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_union_3() {
        let i = Interval::new(Open(42.), Closed(43.));
        assert!(match EMPTY.union(i) {
            Union::Single(Ival(Left(Open(k1)), Right(Closed(k2)))) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_union_4() {
        assert!(matches!(EMPTY.union(EMPTY), Union::Single(Empty)));
    }

    #[test]
    fn test_union_5() {
        assert!(matches!(INFINITY.union(INFINITY), Union::Single(Infinity)));
    }

    #[test]
    fn test_union_6() {
        let a = Interval::new(Closed(42.), Closed(52.));
        let b = Interval::new(Open(42.), Open(52.));
        assert!(matches!(
            a.union(b),
            Union::Single(Ival(Left(Closed(b1)), Right(Closed(b2)))) if b1 == 42. && b2 == 52.
        ));
    }

    #[test]
    fn test_union_7() {
        let a = Interval::new(Closed(42.), Closed(52.));
        let b = Interval::new(Open(42.), Open(52.));
        assert!(matches!(
            b.union(a),
            Union::Single(Ival(Left(Closed(b1)), Right(Closed(b2)))) if b1 == 42. && b2 == 52.
        ));
    }

    #[test]
    fn test_union_8() {
        let a = Interval::new(Closed(42.), Closed(52.));
        let b = Interval::new(Open(22.), Open(45.));
        assert!(matches!(
            b.union(a),
            Union::Single(Ival(Left(Open(b1)), Right(Closed(b2)))) if b1 == 22. && b2 == 52.
        ));
    }

    #[test]
    fn test_build_1() {
        assert!(matches!(Interval::new(Unbound, Unbound), Infinity));
    }

    #[test]
    fn test_build_2() {
        assert!(match Interval::new(Unbound, Closed(42.)) {
            Ival(Left(Bound::Unbound), Right(Closed(k))) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_3() {
        assert!(match Interval::new(Unbound, Open(42.)) {
            Ival(Left(Bound::Unbound), Right(Open(k))) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_4() {
        assert!(match Interval::new(Closed(42.), Closed(43.)) {
            Ival(Left(Closed(k1)), Right(Closed(k2))) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_build_5() {
        assert_eq!(Interval::new(Closed(43.), Closed(42.)), EMPTY);
    }

    #[test]
    fn test_build_6() {
        assert_eq!(Interval::new(Closed(42.), Open(42.)), EMPTY);
    }

    #[test]
    fn test_build_7() {
        assert!(match Interval::new(Closed(42.), Open(43.)) {
            Ival(Left(Closed(k1)), Right(Open(k2))) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_build_8() {
        assert_eq!(Interval::new(Closed(43.), Open(42.)), EMPTY);
    }

    #[test]
    fn test_build_9() {
        assert!(match Interval::new(Closed(42.), Unbound) {
            Ival(Left(Closed(k)), Right(Bound::Unbound)) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_10() {
        assert!(match Interval::new(Open(42.), Closed(43.)) {
            Ival(Left(Open(k1)), Right(Closed(k2))) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_build_11() {
        assert_eq!(Interval::new(Open(43.), Closed(42.)), EMPTY);
    }

    #[test]
    fn test_build_12() {
        assert_eq!(Interval::new(Open(42.), Closed(42.)), EMPTY);
    }

    #[test]
    fn test_build_13() {
        assert_eq!(Interval::new(Open(42.), Open(42.)), EMPTY);
    }

    #[test]
    fn test_build_14() {
        assert!(match Interval::new(Open(42.), Unbound) {
            Ival(Left(Open(k)), Right(Bound::Unbound)) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_15() {
        assert!(match Interval::singleton(42.) {
            Ival(Left(Closed(k1)), Right(Closed(k2))) => k1 == k2,
            _ => false,
        });
    }

    #[test]
    fn test_build_16() {
        assert!(Interval::singleton(42.).is_singleton());
    }

    #[test]
    fn test_empty_1() {
        assert!(Interval::new(Open(42.), Open(42.)).is_empty());
    }

    #[test]
    fn test_empty_2() {
        assert!(EMPTY.is_empty());
    }

    #[test]
    fn test_display_1() {
        assert_eq!(format!("{EMPTY}"), "∅");
    }

    #[test]
    fn test_display_2() {
        let inf = Interval::new(Unbound, Unbound);
        assert_eq!(format!("{inf}"), "]-∞,+∞[");
    }

    #[test]
    fn test_display_3() {
        let sing = Interval::new(Closed(42.), Closed(42.));
        assert_eq!(format!("{sing}"), "{42.00}");
    }

    #[test]
    fn test_display_4() {
        let i = Interval::new(Closed(42.), Closed(43.));
        assert_eq!(format!("{i}"), "[42.00,43.00]");
    }

    #[test]
    fn test_display_5() {
        let i = Interval::new(Closed(42.), Open(43.));
        assert_eq!(format!("{i}"), "[42.00,43.00[");
    }

    #[test]
    fn test_display_6() {
        let i = Interval::new(Closed(42.), Unbound);
        assert_eq!(format!("{i}"), "[42.00,+∞[");
    }

    #[test]
    fn test_display_7() {
        let i = Interval::new(Open(42.), Closed(43.00));
        assert_eq!(format!("{i}"), "]42.00,43.00]");
    }

    #[test]
    fn test_display_8() {
        let i = Interval::new(Open(42.), Open(43.00));
        assert_eq!(format!("{i}"), "]42.00,43.00[");
    }

    #[test]
    fn test_display_9() {
        let i = Interval::new(Open(42.), Unbound);
        assert_eq!(format!("{i}"), "]42.00,+∞[");
    }

    #[test]
    fn test_display_10() {
        let i = Interval::new(Unbound, Closed(42.));
        assert_eq!(format!("{i}"), "]-∞,42.00]");
    }

    #[test]
    fn test_display_11() {
        let i = Interval::new(Unbound, Open(42.));
        assert_eq!(format!("{i}"), "]-∞,42.00[");
    }
}
