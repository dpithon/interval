mod ibound;

use ibound::IBound::{self, Closed, LeftOpen, NegInfy, PosInfy, RightOpen};

use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Set {
    Empty,
    Interval(IBound, IBound),
}

use Set::{Empty, Interval};

pub enum Bound {
    Open(f64),
    Closed(f64),
    Unbound,
}

pub const INFINITY: Set = Set::Interval(NegInfy, PosInfy);
pub const EMPTY: Set = Set::Empty;

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Empty => write!(f, "∅"),
            Interval(b1, b2) => match (b1, b2) {
                (Closed(k1), Closed(k2)) if k1 == k2 => write!(f, "{{{k1:5.2}}}"),
                (Closed(k1), Closed(k2)) => write!(f, "[{k1:5.2},{k2:5.2}]"),
                (Closed(k1), RightOpen(k2)) => write!(f, "[{k1:5.2},{k2:5.2}["),
                (Closed(k1), PosInfy) => write!(f, "[{k1:5.2},+∞["),
                (LeftOpen(k1), Closed(k2)) => write!(f, "]{k1:5.2},{k2:5.2}]"),
                (LeftOpen(k1), RightOpen(k2)) => write!(f, "]{k1:5.2},{k2:5.2}["),
                (LeftOpen(k1), PosInfy) => write!(f, "]{k1:5.2},+∞["),
                (NegInfy, Closed(k2)) => write!(f, "]-∞,{k2:5.2}]"),
                (NegInfy, RightOpen(k2)) => write!(f, "]-∞,{k2:5.2}["),
                (NegInfy, PosInfy) => write!(f, "]-∞,+∞["),
                _ => panic!("Malformed interval {:?}", self),
            },
        }
    }
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Empty, Empty) => true,
            (Interval(a1, a2), Interval(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
}

impl Set {
    /// Build interval from given bounds
    ///
    /// # Returns
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use set::{Set, Open, Closed, Unbound};
    ///
    /// let a = Set::new(Open(42.), Closed(43.));
    /// let b = Set::new(Unbound, Unbound);
    /// let c = Set::singleton(42.);
    ///
    /// assert_eq!(format!("{a}"), "]42.00,43.00]");
    /// assert_eq!(format!("{b}"), "]-∞,+∞[");
    /// assert_eq!(format!("{c}"), "{42.00}");
    /// ```
    ///
    pub fn new(b1: Bound, b2: Bound) -> Self {
        let b1 = match b1 {
            Bound::Open(k) => LeftOpen(k),
            Bound::Closed(k) => Closed(k),
            Bound::Unbound => NegInfy,
        };
        let b2 = match b2 {
            Bound::Open(k) => RightOpen(k),
            Bound::Closed(k) => Closed(k),
            Bound::Unbound => PosInfy,
        };

        if b2 < b1 {
            Empty
        } else {
            Self::Interval(b1, b2)
        }
    }

    pub fn singleton(k: f64) -> Self {
        Interval(Closed(k), Closed(k))
    }

    pub fn is_singleton(&self) -> bool {
        match self {
            Interval(Closed(k1), Closed(k2)) => k1 == k2,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }

    pub fn union(self, other: Set) -> Set {
        match (self, other) {
            (Empty, _) => other,
            (_, Empty) => self,
            (Interval(NegInfy, PosInfy), _) | (_, Interval(NegInfy, PosInfy)) => {
                Interval(NegInfy, PosInfy)
            }
            (Interval(a1, a2), Interval(b1, b2))
                if Interval(a1, a2).overlap(Interval(b1, b2))
                    || a2.closure() == b1.closure()
                    || a1.closure() == b2.closure() =>
            {
                Interval(a1.min(b1), a2.max(b2))
            }
            (Interval(a1, a2), Interval(b1, b2)) => Set::Empty,
        }
    }

    /// Check if intervals overlap
    ///
    /// Note that ``Empty` overlap nothing.
    ///
    pub fn overlap(self, other: Set) -> bool {
        match (self, other) {
            (Set::Empty, _) | (_, Set::Empty) => false,
            (Interval(NegInfy, PosInfy), _) => true,
            (_, Interval(NegInfy, PosInfy)) => true,
            (Interval(a1, a2), Interval(b1, b2)) => b2 >= a1 && b1 <= a2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overlap_1() {
        let a = Set::new(Bound::Unbound, Bound::Unbound);
        let b = Set::new(Bound::Unbound, Bound::Unbound);

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_2() {
        let a = Set::new(Bound::Unbound, Bound::Unbound);
        let b = Set::Empty;

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_3() {
        let a = Set::Empty;
        let b = Set::new(Bound::Unbound, Bound::Unbound);

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_4() {
        let a = Set::new(Bound::Closed(42.), Bound::Closed(43.));
        let b = Set::new(Bound::Unbound, Bound::Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_5() {
        let a = Set::new(Bound::Unbound, Bound::Unbound);
        let b = Set::new(Bound::Closed(42.), Bound::Closed(43.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_6() {
        let a = Set::new(Bound::Closed(42.), Bound::Open(43.));
        let b = Set::new(Bound::Unbound, Bound::Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_7() {
        let a = Set::new(Bound::Unbound, Bound::Unbound);
        let b = Set::new(Bound::Closed(42.), Bound::Open(43.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_8() {
        let a = Set::new(Bound::Open(42.), Bound::Open(43.));
        let b = Set::new(Bound::Unbound, Bound::Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_9() {
        let a = Set::new(Bound::Unbound, Bound::Unbound);
        let b = Set::new(Bound::Open(42.), Bound::Open(43.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_10() {
        let a = Set::new(Bound::Unbound, Bound::Open(43.));
        let b = Set::new(Bound::Unbound, Bound::Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_11() {
        let a = Set::new(Bound::Unbound, Bound::Unbound);
        let b = Set::new(Bound::Open(42.), Bound::Unbound);
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_12() {
        let a = Set::Empty;
        let b = Set::new(Bound::Unbound, Bound::Unbound);

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_13() {
        let a = Set::Empty;
        let b = Set::Empty;

        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_14() {
        let a = Set::new(Bound::Closed(42.), Bound::Closed(52.));
        let b = Set::new(Bound::Closed(42.), Bound::Closed(52.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_15() {
        let a = Set::new(Bound::Closed(42.), Bound::Closed(52.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_16() {
        let a = Set::new(Bound::Closed(42.), Bound::Closed(52.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));

        assert!(b.overlap(a));
    }

    #[test]
    fn test_overlap_17() {
        let a = Set::new(Bound::Open(42.), Bound::Closed(52.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_18() {
        let a = Set::new(Bound::Open(42.), Bound::Closed(52.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));

        assert!(b.overlap(a));
    }

    #[test]
    fn test_overlap_19() {
        let a = Set::new(Bound::Closed(42.), Bound::Open(52.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));

        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_20() {
        let a = Set::new(Bound::Closed(42.), Bound::Open(52.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));

        assert!(b.overlap(a));
    }

    #[test]
    fn test_overlap_21() {
        let a = Set::new(Bound::Open(42.), Bound::Open(52.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));
        assert!(a.overlap(b));
    }

    #[test]
    fn test_overlap_22() {
        let a = Set::new(Bound::Unbound, Bound::Closed(42.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_23() {
        let a = Set::new(Bound::Unbound, Bound::Open(42.));
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_24() {
        let a = Set::new(Bound::Closed(52.), Bound::Unbound);
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_overlap_25() {
        let a = Set::new(Bound::Open(52.), Bound::Unbound);
        let b = Set::new(Bound::Open(42.), Bound::Open(52.));
        assert!(!a.overlap(b));
    }

    #[test]
    fn test_union_1() {
        assert!(matches!(EMPTY.union(EMPTY), Set::Empty));
    }

    #[test]
    fn test_union_2() {
        let i = Set::new(Bound::Open(42.), Bound::Closed(43.));
        assert!(match i.union(EMPTY) {
            Interval(LeftOpen(k1), Closed(k2)) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_union_3() {
        let i = Set::new(Bound::Open(42.), Bound::Closed(43.));
        assert!(match EMPTY.union(i) {
            Interval(LeftOpen(k1), Closed(k2)) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_union_4() {
        assert!(matches!(
            INFINITY.union(EMPTY),
            Set::Interval(NegInfy, PosInfy)
        ));
    }

    #[test]
    fn test_union_5() {
        assert!(matches!(
            INFINITY.union(INFINITY),
            Set::Interval(NegInfy, PosInfy)
        ));
    }

    #[test]
    fn test_build_1() {
        assert!(matches!(
            Set::new(Bound::Unbound, Bound::Unbound),
            Set::Interval(NegInfy, PosInfy)
        ));
    }

    #[test]
    fn test_build_2() {
        assert!(match Set::new(Bound::Unbound, Bound::Closed(42.)) {
            Set::Interval(NegInfy, Closed(k)) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_3() {
        assert!(match Set::new(Bound::Unbound, Bound::Open(42.)) {
            Set::Interval(NegInfy, RightOpen(k)) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_4() {
        assert!(match Set::new(Bound::Closed(42.), Bound::Closed(43.)) {
            Set::Interval(Closed(k1), Closed(k2)) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_build_5() {
        assert!(matches!(
            Set::new(Bound::Closed(43.), Bound::Closed(42.)),
            Set::Empty
        ));
    }

    #[test]
    fn test_build_6() {
        assert!(matches!(
            Set::new(Bound::Closed(42.), Bound::Open(42.)),
            Set::Empty
        ));
    }

    #[test]
    fn test_build_7() {
        assert!(match Set::new(Bound::Closed(42.), Bound::Open(43.)) {
            Set::Interval(Closed(k1), RightOpen(k2)) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_build_8() {
        assert!(matches!(
            Set::new(Bound::Closed(43.), Bound::Open(42.)),
            Set::Empty
        ));
    }

    #[test]
    fn test_build_9() {
        assert!(match Set::new(Bound::Closed(42.), Bound::Unbound) {
            Set::Interval(Closed(k), PosInfy) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_10() {
        assert!(match Set::new(Bound::Open(42.), Bound::Closed(43.)) {
            Set::Interval(LeftOpen(k1), Closed(k2)) => k1 == 42. && k2 == 43.,
            _ => false,
        });
    }

    #[test]
    fn test_build_11() {
        assert!(matches!(
            Set::new(Bound::Open(43.), Bound::Closed(42.)),
            Set::Empty
        ));
    }

    #[test]
    fn test_build_12() {
        assert!(matches!(
            Set::new(Bound::Open(42.), Bound::Closed(42.)),
            Set::Empty
        ));
    }

    #[test]
    fn test_build_13() {
        assert!(matches!(
            Set::new(Bound::Open(42.), Bound::Open(42.)),
            Set::Empty
        ));
    }

    #[test]
    fn test_build_14() {
        assert!(match Set::new(Bound::Open(42.), Bound::Unbound) {
            Set::Interval(LeftOpen(k), PosInfy) => k == 42.,
            _ => false,
        });
    }

    #[test]
    fn test_build_15() {
        assert!(match Set::singleton(42.) {
            Set::Interval(Closed(k1), Closed(k2)) => k1 == k2,
            _ => false,
        });
    }

    #[test]
    fn test_build_16() {
        assert!(Set::singleton(42.).is_singleton());
    }

    #[test]
    fn test_empty() {
        assert!(Set::new(Bound::Open(42.), Bound::Open(42.)).is_empty());
    }

    #[test]
    fn test_display_1() {
        assert_eq!(format!("{Empty}"), "∅");
    }

    #[test]
    fn test_display_2() {
        let inf = Set::new(Bound::Unbound, Bound::Unbound);
        assert_eq!(format!("{inf}"), "]-∞,+∞[");
    }

    #[test]
    fn test_display_3() {
        let sing = Set::new(Bound::Closed(42.), Bound::Closed(42.));
        assert_eq!(format!("{sing}"), "{42.00}");
    }

    #[test]
    fn test_display_4() {
        let i = Set::new(Bound::Closed(42.), Bound::Closed(43.));
        assert_eq!(format!("{i}"), "[42.00,43.00]");
    }

    #[test]
    fn test_display_5() {
        let i = Set::new(Bound::Closed(42.), Bound::Open(43.));
        assert_eq!(format!("{i}"), "[42.00,43.00[");
    }

    #[test]
    fn test_display_6() {
        let i = Set::new(Bound::Closed(42.), Bound::Unbound);
        assert_eq!(format!("{i}"), "[42.00,+∞[");
    }

    #[test]
    fn test_display_7() {
        let i = Set::new(Bound::Open(42.), Bound::Closed(43.00));
        assert_eq!(format!("{i}"), "]42.00,43.00]");
    }

    #[test]
    fn test_display_8() {
        let i = Set::new(Bound::Open(42.), Bound::Open(43.00));
        assert_eq!(format!("{i}"), "]42.00,43.00[");
    }

    #[test]
    fn test_display_9() {
        let i = Set::new(Bound::Open(42.), Bound::Unbound);
        assert_eq!(format!("{i}"), "]42.00,+∞[");
    }

    #[test]
    fn test_display_10() {
        let i = Set::new(Bound::Unbound, Bound::Closed(42.));
        assert_eq!(format!("{i}"), "]-∞,42.00]");
    }

    #[test]
    fn test_display_11() {
        let i = Set::new(Bound::Unbound, Bound::Open(42.));
        assert_eq!(format!("{i}"), "]-∞,42.00[");
    }
}
