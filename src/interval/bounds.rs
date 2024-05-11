use std::cmp::{Ordering, PartialEq, PartialOrd};

pub enum Bound {
    Open(f64),
    Closed(f64),
    Unbound,
}

/// IBounds of an interval
#[derive(Debug, Clone, Copy)]
pub enum IBound {
    LeftOpen(f64),
    RightOpen(f64),
    Closed(f64),
    NegInfy,
    PosInfy,
}

use IBound::{Closed, LeftOpen, NegInfy, PosInfy, RightOpen};

impl PartialEq for IBound {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Closed(k1), Closed(k2)) => k1 == k2,
            (LeftOpen(k1), LeftOpen(k2)) => k1 == k2,
            (RightOpen(k1), RightOpen(k2)) => k1 == k2,
            (PosInfy, PosInfy) => true,
            (NegInfy, NegInfy) => true,
            _ => false,
        }
    }
}

impl PartialOrd for IBound {
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Closed(k1), Closed(k2))
            | (Closed(k1), RightOpen(k2))
            | (LeftOpen(k1), Closed(k2))
            | (LeftOpen(k1), LeftOpen(k2))
            | (LeftOpen(k1), RightOpen(k2))
            | (RightOpen(k1), RightOpen(k2)) => k1 < k2,
            (Closed(k1), LeftOpen(k2))
            | (RightOpen(k1), Closed(k2))
            | (RightOpen(k1), LeftOpen(k2)) => k1 <= k2,
            (PosInfy, _) | (_, NegInfy) => false,
            _ => true,
        }
    }

    fn le(&self, other: &Self) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Closed(k1), Closed(k2))
            | (Closed(k1), LeftOpen(k2))
            | (LeftOpen(k1), LeftOpen(k2))
            | (RightOpen(k1), Closed(k2))
            | (RightOpen(k1), LeftOpen(k2))
            | (RightOpen(k1), RightOpen(k2)) => k1 > k2,
            (LeftOpen(k1), Closed(k2))
            | (LeftOpen(k1), RightOpen(k2))
            | (Closed(k1), RightOpen(k2)) => k1 >= k2,
            (NegInfy, _) | (_, PosInfy) => false,
            _ => true,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        self > other || self == other
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl IBound {
    pub fn min(self, b2: IBound) -> IBound {
        if self < b2 {
            self
        } else {
            b2
        }
    }

    pub fn max(self, b2: IBound) -> IBound {
        if self > b2 {
            self
        } else {
            b2
        }
    }

    pub fn closure(self) -> Self {
        match self {
            NegInfy => NegInfy,
            PosInfy => PosInfy,
            Closed(k) | LeftOpen(k) | RightOpen(k) => Closed(k),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let bounds = [Closed(42.), LeftOpen(42.), RightOpen(42.), PosInfy, NegInfy];

        for (i, b1) in bounds.iter().enumerate() {
            for (j, b2) in bounds.iter().enumerate() {
                if i == j {
                    assert_eq!(b1, b2);
                } else {
                    assert_ne!(b1, b2);
                }
            }
        }
    }

    #[test]
    fn test_lt_1() {
        let b1 = Closed(42.);
        let bounds = [
            Closed(43.),
            LeftOpen(42.),
            PosInfy,
            RightOpen(43.),
            LeftOpen(43.),
        ];

        for bound in bounds {
            assert!(dbg!(b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_2() {
        let b1 = Closed(42.);
        let bounds = [
            Closed(41.),
            RightOpen(42.),
            NegInfy,
            LeftOpen(41.),
            RightOpen(41.),
        ];

        for bound in bounds {
            assert!(dbg!(bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_3() {
        let b1 = Closed(42.);
        let bounds = [
            Closed(41.),
            RightOpen(42.),
            NegInfy,
            LeftOpen(41.),
            RightOpen(41.),
        ];

        for bound in bounds {
            assert!(dbg!(!b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_4() {
        let b1 = Closed(42.);
        let bounds_are_not_lt_b1 = [
            Closed(43.),
            LeftOpen(42.),
            PosInfy,
            RightOpen(43.),
            LeftOpen(43.),
        ];

        for bound in bounds_are_not_lt_b1 {
            assert!(dbg!(!bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_5() {
        let b1 = LeftOpen(42.);
        let bounds = [Closed(43.), LeftOpen(43.), PosInfy, RightOpen(43.)];

        for bound in bounds {
            assert!(dbg!(b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_6() {
        let b1 = LeftOpen(42.);
        let bounds = [Closed(42.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_7() {
        let b1 = LeftOpen(42.);
        let bounds = [Closed(42.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(!b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_8() {
        let b1 = LeftOpen(42.);
        let bounds_are_not_lt_b1 = [Closed(43.), LeftOpen(43.), PosInfy, RightOpen(43.)];

        for bound in bounds_are_not_lt_b1 {
            assert!(dbg!(!bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_9() {
        let b1 = RightOpen(42.);
        let bounds = [Closed(42.), LeftOpen(42.), PosInfy, RightOpen(43.)];

        for bound in bounds {
            assert!(dbg!(b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_10() {
        let b1 = RightOpen(42.);
        let bounds = [Closed(41.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_11() {
        let b1 = RightOpen(42.);
        let bounds = [Closed(41.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(!b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_12() {
        let b1 = RightOpen(42.);
        let bounds_are_not_lt_b1 = [Closed(42.), LeftOpen(42.), PosInfy, RightOpen(43.)];

        for bound in bounds_are_not_lt_b1 {
            assert!(dbg!(!bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_13() {
        let b1 = PosInfy;
        let bounds = [Closed(41.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_14() {
        let b1 = PosInfy;
        let b2 = PosInfy;

        assert!(dbg!(!b1.lt(&b2)));
    }

    #[test]
    fn test_lt_15() {
        let b1 = NegInfy;
        let bounds = [Closed(41.), RightOpen(41.), PosInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_16() {
        let b1 = NegInfy;
        let bounds = [Closed(41.), RightOpen(41.), PosInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(b1.lt(&bound)));
        }
    }

    #[test]
    fn test_gt_1() {
        let b1 = Closed(42.);
        let bounds = [
            Closed(43.),
            LeftOpen(42.),
            PosInfy,
            RightOpen(43.),
            LeftOpen(43.),
        ];

        for bound in bounds {
            assert!(dbg!(bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_2() {
        let b1 = Closed(42.);
        let bounds = [
            Closed(41.),
            RightOpen(42.),
            NegInfy,
            LeftOpen(41.),
            RightOpen(41.),
        ];

        for bound in bounds {
            assert!(dbg!(b1.gt(&bound)));
        }
    }

    #[test]
    fn test_gt_3() {
        let b1 = Closed(42.);
        let bounds = [
            Closed(41.),
            RightOpen(42.),
            NegInfy,
            LeftOpen(41.),
            RightOpen(41.),
        ];

        for bound in bounds {
            assert!(dbg!(!bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_4() {
        let b1 = Closed(42.);
        let bounds = [
            Closed(43.),
            LeftOpen(42.),
            PosInfy,
            RightOpen(43.),
            LeftOpen(43.),
        ];

        for bound in bounds {
            assert!(dbg!(bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_5() {
        let b1 = LeftOpen(42.);
        let bounds = [Closed(43.), LeftOpen(43.), PosInfy, RightOpen(43.)];

        for bound in bounds {
            assert!(dbg!(!b1.gt(&bound)));
        }
    }

    #[test]
    fn test_gt_6() {
        let b1 = LeftOpen(42.);
        let bounds = [Closed(42.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(b1.gt(&bound)));
        }
    }

    #[test]
    fn test_gt_7() {
        let b1 = LeftOpen(42.);
        let bounds = [Closed(42.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(!bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_8() {
        let b1 = LeftOpen(42.);
        let bounds = [Closed(43.), LeftOpen(43.), PosInfy, RightOpen(43.)];

        for bound in bounds {
            assert!(dbg!(bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_9() {
        let b1 = RightOpen(42.);
        let bounds = [Closed(42.), LeftOpen(42.), PosInfy, RightOpen(43.)];

        for bound in bounds {
            assert!(dbg!(!b1.gt(&bound)));
        }
    }

    #[test]
    fn test_gt_10() {
        let b1 = RightOpen(42.);
        let bounds = [Closed(41.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(b1.gt(&bound)));
        }
    }

    #[test]
    fn test_gt_11() {
        let b1 = RightOpen(42.);
        let bounds = [Closed(41.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(!bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_12() {
        let b1 = RightOpen(42.);
        let bounds = [Closed(42.), LeftOpen(42.), PosInfy, RightOpen(43.)];

        for bound in bounds {
            assert!(dbg!(bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_13() {
        let b1 = PosInfy;
        let bounds = [Closed(41.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(!bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_14() {
        let b1 = PosInfy;
        let bounds = [Closed(41.), RightOpen(41.), NegInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(b1.gt(&bound)));
        }
    }

    #[test]
    fn test_gt_15() {
        let b1 = PosInfy;
        let b2 = PosInfy;

        assert!(dbg!(!b1.gt(&b2)));
    }

    #[test]
    fn test_gt_16() {
        let b1 = NegInfy;
        let bounds = [Closed(41.), RightOpen(41.), PosInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(bound.gt(&b1)));
        }
    }

    #[test]
    fn test_gt_17() {
        let b1 = NegInfy;
        let bounds = [Closed(41.), RightOpen(41.), PosInfy, LeftOpen(41.)];

        for bound in bounds {
            assert!(dbg!(!b1.gt(&bound)));
        }
    }

    #[test]
    fn test_gt_18() {
        let b1 = NegInfy;
        let b2 = NegInfy;

        assert!(dbg!(!b1.gt(&b2)));
    }
}
