use std::cmp::Ordering;
use std::fmt::Display;

use super::bound::Bound::{self, Closed, Open, Unbound};
use super::right::Right;

#[derive(Debug, Clone, Copy)]
pub struct Left(pub Bound);

impl Left {
    pub fn min(self, other: Left) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    pub fn max(self, other: Left) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    pub fn closure(self, other: Right) -> bool {
        let Left(left) = self;
        let Right(right) = other;

        match (left, right) {
            (Closed(k1), Closed(k2)) => k1 == k2,
            (Open(k1), Closed(k2)) => k1 == k2,
            (Closed(k1), Open(k2)) => k1 == k2,
            _ => false,
        }
    }
}

impl Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Left(bound) = self;
        match bound {
            Closed(k) => write!(f, "[{k:5.2}"),
            Open(k) => write!(f, "({k:5.2}"),
            Unbound => write!(f, "(-∞"),
        }
    }
}

impl PartialEq for Left {
    fn eq(&self, other: &Self) -> bool {
        let (Left(k1), Left(k2)) = (self, other);
        k1 == k2
    }
}

impl PartialEq<Right> for Left {
    fn eq(&self, other: &Right) -> bool {
        let (Left(left), Right(right)) = (self, other);
        match (left, right) {
            (Closed(k1), Closed(k2)) => k1 == k2,
            _ => false,
        }
    }
}

impl PartialOrd for Left {
    fn lt(&self, other: &Self) -> bool {
        let (Left(bound1), Left(bound2)) = (self, other);
        match (bound1, bound2) {
            (Closed(k1), Closed(k2)) => k1 < k2, // [k1.. < [k2..
            (Open(k1), Open(k2)) => k1 < k2,     // ]k1.. < ]k2..
            (Open(k1), Closed(k2)) => k1 < k2,   // ]k1.. < [k2..
            (Closed(k1), Open(k2)) => k1 <= k2,  // [k1.. < ]k2..
            (_, Unbound) => false,
            (Unbound, _) => true,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        let (Left(bound1), Left(bound2)) = (self, other);
        match (bound1, bound2) {
            (Closed(k1), Closed(k2)) => k1 > k2, // [k1.. > [k2..
            (Open(k1), Open(k2)) => k1 > k2,     // ]k1.. > ]k2..
            (Open(k1), Closed(k2)) => k1 >= k2,  // ]k1.. > [k2..
            (Closed(k1), Open(k2)) => k1 > k2,   // [k1.. > ]k2..
            (Unbound, _) => false,
            (_, Unbound) => true,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialOrd<Right> for Left {
    fn gt(&self, other: &Right) -> bool {
        let (Left(left), Right(right)) = (self, other);
        match (left, right) {
            (Open(k1), Open(k2)) => k1 >= k2,    // ]k1.. > ..k2[
            (Open(k1), Closed(k2)) => k1 >= k2,  // ]k1.. > ..k2]
            (Closed(k1), Open(k2)) => k1 >= k2,  // [k1.. > ..k2[
            (Closed(k1), Closed(k2)) => k1 > k2, // [k1.. > ..k2]
            _ => false,
        }
    }

    fn lt(&self, other: &Right) -> bool {
        let (Left(left), Right(right)) = (self, other);
        match (left, right) {
            (Open(k1), Open(k2)) => k1 < k2,     // ]k1.. < ..k2[
            (Open(k1), Closed(k2)) => k1 < k2,   // ]k1.. < ..k2]
            (Closed(k1), Open(k2)) => k1 < k2,   // [k1.. < ..k2[
            (Closed(k1), Closed(k2)) => k1 < k2, // [k1.. < ..k2]
            (Unbound, _) => true,                // -inf < Right(+inf, close, open)
            (_, Unbound) => true,                // Left(-inf, close, open) < +inf
        }
    }

    fn partial_cmp(&self, other: &Right) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else {
            assert!(self == other);
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let bounds = [Left(Closed(42.)), Left(Open(42.)), Left(Unbound)];

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
        let b1 = Left(Closed(42.));
        let set1 = [Left(Closed(43.)), Left(Open(42.)), Left(Open(43.))];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_lt_2() {
        let b1 = Left(Closed(42.));
        let set1 = [
            Left(Closed(42.)),
            Left(Closed(41.)),
            Left(Unbound),
            Left(Open(41.)),
        ];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_lt_3() {
        let b1 = Left(Open(42.));
        let set1 = [Left(Closed(43.)), Left(Open(43.))];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_lt_4() {
        let b1 = Left(Open(42.));
        let set1 = [
            Left(Open(42.)),
            Left(Open(41.)),
            Left(Closed(41.)),
            Left(Unbound),
        ];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_lt_5() {
        let b1 = Left(Unbound);
        let set1 = [Left(Closed(42.)), Left(Open(42.))];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_lt_6() {
        let b1 = Left(Unbound);
        let set1 = [Left(Unbound)];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_gt_1() {
        let b1 = Left(Closed(42.));
        let set1 = [Left(Closed(41.)), Left(Open(41.)), Left(Unbound)];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gt_2() {
        let b1 = Left(Closed(42.));
        let set1 = [
            Left(Closed(42.)),
            Left(Closed(43.)),
            Left(Open(42.)),
            Left(Open(43.)),
        ];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    #[test]
    fn test_gt_3() {
        let b1 = Left(Open(43.));
        let set1 = [Left(Closed(43.)), Left(Open(42.)), Left(Unbound)];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gt_4() {
        let b1 = Left(Open(42.));
        let set1 = [Left(Open(42.)), Left(Open(43.)), Left(Closed(43.))];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    //   #[test]
    //   fn test_gt_5() {
    //       let b1 = Left(Unbound);
    //       let set1 = [Left(Closed(42.)), Left(Open(42.))];
    //
    //       for bound in set1 {
    //           assert!(b1.lt(&bound));
    //       }
    //   }

    #[test]
    fn test_gt_6() {
        let b1 = Left(Unbound);
        let set1 = [Left(Unbound), Left(Closed(42.)), Left(Open(42.))];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    #[test]
    fn test_min_1() {
        assert_eq!(Left(Closed(42.)).min(Left(Closed(42.))), Left(Closed(42.)));
    }

    #[test]
    fn test_min_2() {
        assert_eq!(Left(Closed(42.)).min(Left(Open(42.))), Left(Closed(42.)));
    }

    #[test]
    fn test_min_3() {
        assert_eq!(Left(Closed(42.)).min(Left(Unbound)), Left(Unbound));
    }

    #[test]
    fn test_min_4() {
        assert_eq!(Left(Open(42.)).min(Left(Closed(42.))), Left(Closed(42.)));
    }

    #[test]
    fn test_min_5() {
        assert_eq!(Left(Open(42.)).min(Left(Open(42.))), Left(Open(42.)));
    }

    #[test]
    fn test_min_6() {
        assert_eq!(Left(Open(42.)).min(Left(Unbound)), Left(Unbound));
    }

    #[test]
    fn test_min_7() {
        assert_eq!(Left(Unbound).min(Left(Closed(42.))), Left(Unbound));
    }

    #[test]
    fn test_min_8() {
        assert_eq!(Left(Unbound).min(Left(Open(42.))), Left(Unbound));
    }

    #[test]
    fn test_min_9() {
        assert_eq!(Left(Unbound).min(Left(Unbound)), Left(Unbound));
    }

    #[test]
    fn test_max_1() {
        assert_eq!(Left(Closed(42.)).max(Left(Closed(42.))), Left(Closed(42.)));
    }

    #[test]
    fn test_max_2() {
        assert_eq!(Left(Closed(42.)).max(Left(Open(42.))), Left(Open(42.)));
    }

    #[test]
    fn test_max_3() {
        assert_eq!(Left(Closed(42.)).max(Left(Unbound)), Left(Closed(42.)));
    }

    #[test]
    fn test_max_4() {
        assert_eq!(Left(Open(42.)).max(Left(Closed(42.))), Left(Open(42.)));
    }

    #[test]
    fn test_max_5() {
        assert_eq!(Left(Open(42.)).max(Left(Open(42.))), Left(Open(42.)));
    }

    #[test]
    fn test_max_6() {
        assert_eq!(Left(Open(42.)).max(Left(Unbound)), Left(Open(42.)));
    }

    #[test]
    fn test_max_7() {
        assert_eq!(Left(Unbound).max(Left(Closed(42.))), Left(Closed(42.)));
    }

    #[test]
    fn test_max_8() {
        assert_eq!(Left(Unbound).max(Left(Open(42.))), Left(Open(42.)));
    }

    #[test]
    fn test_max_9() {
        assert_eq!(Left(Unbound).max(Left(Unbound)), Left(Unbound));
    }

    #[test]
    fn test_closure_1() {
        assert!(Left(Closed(42.)).closure(Right(Closed(42.))));
    }

    #[test]
    fn test_closure_2() {
        assert!(Left(Closed(42.)).closure(Right(Open(42.))));
    }

    #[test]
    fn test_closure_3() {
        assert!(Left(Open(42.)).closure(Right(Closed(42.))));
    }

    #[test]
    fn test_closure_4() {
        assert!(!Left(Open(42.)).closure(Right(Open(42.))));
    }

    #[test]
    fn test_closure_5() {
        assert!(!Left(Closed(43.)).closure(Right(Closed(42.))));
    }

    #[test]
    fn test_fmt_1() {
        assert_eq!(format!("{}", Left(Closed(42.))), "[42.00");
    }

    #[test]
    fn test_fmt_2() {
        assert_eq!(format!("{}", Left(Open(42.))), "(42.00");
    }

    #[test]
    fn test_fmt_3() {
        assert_eq!(format!("{}", Left(Unbound)), "(-∞");
    }

    #[test]
    fn test_eqr_1() {
        let lefts = [
            Left(Closed(42.)),
            Left(Open(42.)),
            Left(Unbound),
            Left(Closed(43.)),
        ];
        let rights = [
            Right(Closed(42.)),
            Right(Open(42.)),
            Right(Unbound),
            Right(Closed(43.)),
        ];

        for left in lefts.iter() {
            for right in rights.iter() {
                match (left, right) {
                    (Left(Closed(l)), Right(Closed(r))) if l == r => assert!(left == right),
                    _ => assert!(left != right),
                }
            }
        }
    }

    #[test]
    fn test_ltr_1() {
        let b1 = Left(Closed(42.));
        let set1 = [Right(Closed(43.)), Right(Open(43.)), Right(Unbound)];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_ltr_2() {
        let b1 = Left(Closed(42.));
        let set1 = [Right(Closed(42.)), Right(Closed(41.)), Right(Open(41.))];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_ltr_3() {
        let b1 = Left(Open(42.));
        let set1 = [Right(Closed(43.)), Right(Open(43.)), Right(Unbound)];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_ltr_4() {
        let b1 = Left(Open(42.));
        let set1 = [
            Right(Closed(42.)),
            Right(Open(42.)),
            Right(Closed(41.)),
            Right(Open(41.)),
        ];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_ltr_5() {
        let b1 = Left(Unbound);
        let set1 = [Right(Closed(42.)), Right(Open(42.)), Right(Unbound)];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    //   #[test]
    //   fn test_ltr_6() {
    //       let b1 = Left(Unbound);
    //       let set1 = [Right(Unbound)];
    //
    //       for bound in set1 {
    //           assert!(!b1.lt(&bound));
    //       }
    //   }

    #[test]
    fn test_gtr_1() {
        let b1 = Left(Closed(42.));
        let set1 = [Right(Closed(41.)), Right(Open(41.)), Right(Open(42.))];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gtr_2() {
        let b1 = Left(Closed(42.));
        let set1 = [
            Right(Closed(42.)),
            Right(Closed(43.)),
            Right(Open(43.)),
            Right(Unbound),
        ];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    #[test]
    fn test_gtr_3() {
        let b1 = Left(Open(42.));
        let set1 = [Right(Closed(42.)), Right(Open(42.)), Right(Open(41.))];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gtr_4() {
        let b1 = Left(Open(42.));
        let set1 = [Right(Unbound), Right(Open(43.)), Right(Closed(43.))];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    //   #[test]
    //   fn test_gtr_5() {
    //       let b1 = Left(Unbound);
    //       let set1 = [Right(Closed(42.)), Right(Open(42.))];
    //
    //       for bound in set1 {
    //           assert!(b1.lt(&bound));
    //       }
    //   }

    #[test]
    fn test_gtr_6() {
        let b1 = Left(Unbound);
        let set1 = [Right(Unbound), Right(Closed(42.)), Right(Open(42.))];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }
}
