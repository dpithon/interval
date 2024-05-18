use std::cmp::Ordering;
use std::fmt::Display;

use super::bound::Bound::{self, Closed, Open, Unbound};
use super::left::Left;

#[derive(Debug, Clone, Copy)]
pub struct Right(pub Bound);

impl Right {
    pub fn min(self, other: Right) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    pub fn max(self, other: Right) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    pub fn closure(self, other: Left) -> bool {
        let Left(left) = other;
        let Right(right) = self;

        match (left, right) {
            (Closed(k1), Closed(k2)) => k1 == k2,
            (Open(k1), Closed(k2)) => k1 == k2,
            (Closed(k1), Open(k2)) => k1 == k2,
            _ => false,
        }
    }
}

impl Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Right(Closed(k)) => write!(f, "{k:5.2}]"),
            Right(Open(k)) => write!(f, "{k:5.2})"),
            Right(Unbound) => write!(f, "+∞)"),
        }
    }
}

impl PartialEq for Right {
    fn eq(&self, other: &Self) -> bool {
        let (Right(k1), Right(k2)) = (self, other);
        k1 == k2
    }
}

impl PartialEq<Left> for Right {
    fn eq(&self, other: &Left) -> bool {
        let (Right(right), Left(left)) = (self, other);
        match (left, right) {
            (Closed(k2), Closed(k1)) => k1 == k2,
            _ => false,
        }
    }
}

impl PartialOrd for Right {
    fn lt(&self, other: &Self) -> bool {
        let (Right(bound1), Right(bound2)) = (self, other);
        match (bound1, bound2) {
            (Closed(k1), Closed(k2)) => k1 < k2, // ..k1] < ..k2]
            (Open(k1), Open(k2)) => k1 < k2,     // ..k1[ < ..k2[
            (Open(k1), Closed(k2)) => k1 <= k2,  // ..k1[ < ..k2]
            (Closed(k1), Open(k2)) => k1 < k2,   // ..k1] < ..k2[
            (Unbound, _) => false,
            (_, Unbound) => true,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        let (Right(bound1), Right(bound2)) = (self, other);
        match (bound1, bound2) {
            (Closed(k1), Closed(k2)) => k1 > k2, // ..k1] > ..k2]
            (Open(k1), Open(k2)) => k1 > k2,     // ..k1[ > ..k2[
            (Open(k1), Closed(k2)) => k1 > k2,   // ..k1[ > ..k2]
            (Closed(k1), Open(k2)) => k1 >= k2,  // ..k1] > ..k2[
            (_, Unbound) => false,
            (Unbound, _) => true,
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

impl PartialOrd<Left> for Right {
    fn gt(&self, other: &Left) -> bool {
        let (Right(right), Left(left)) = (self, other);
        match (right, left) {
            (Open(k1), Open(k2)) => k1 > k2,     // ..k1[ > [k2..
            (Open(k1), Closed(k2)) => k1 > k2,   // ..k1] > [k2..
            (Closed(k1), Closed(k2)) => k1 > k2, // ..k1] > ]k2..
            (Closed(k1), Open(k2)) => k1 > k2,   // ..k1[ > ]k2..
            (_, Unbound) => true,
            (Unbound, _) => true,
        }
    }

    fn lt(&self, other: &Left) -> bool {
        let (Right(right), Left(left)) = (self, other);
        match (right, left) {
            (Open(k1), Open(k2)) => k1 <= k2,    // ..k1[ < ]k2..
            (Open(k1), Closed(k2)) => k1 <= k2,  // ..k1[ < [k2..
            (Closed(k1), Closed(k2)) => k1 < k2, // ..k1] < [k2..
            (Closed(k1), Open(k2)) => k1 <= k2,  // ..k1] < ]k2..
            (Unbound, _) => false,
            (_, Unbound) => false,
        }
    }

    fn partial_cmp(&self, other: &Left) -> Option<Ordering> {
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
        let bounds = [Right(Closed(42.)), Right(Open(42.)), Right(Unbound)];

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
        let b1 = Right(Closed(42.));
        let set1 = [
            Right(Closed(43.)),
            Right(Closed(43.)),
            Right(Open(43.)),
            Right(Unbound),
        ];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_lt_2() {
        let b1 = Right(Closed(42.));
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
    fn test_lt_3() {
        let b1 = Right(Open(42.));
        let set1 = [Right(Closed(42.)), Right(Unbound)];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_lt_4() {
        let b1 = Right(Open(42.));
        let set1 = [Right(Open(42.)), Right(Open(41.)), Right(Closed(41.))];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    //  #[test]
    //  fn test_lt_5() {
    //      let b1 = Right(Unbound);
    //      let set1 = [Right(Closed(42.)), Right(Open(42.))];
    //
    //      for bound in set1 {
    //          assert!(b1.lt(&bound));
    //      }
    //  }

    #[test]
    fn test_lt_6() {
        let b1 = Right(Unbound);
        let set1 = [Right(Unbound), Right(Closed(42.)), Right(Open(42.))];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_gt_1() {
        let b1 = Right(Closed(42.));
        let set1 = [Right(Closed(41.)), Right(Open(41.)), Right(Open(42.))];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gt_2() {
        let b1 = Right(Closed(42.));
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
    fn test_gt_3() {
        let b1 = Right(Open(42.));
        let set1 = [Right(Closed(41.)), Right(Open(41.))];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gt_4() {
        let b1 = Right(Open(42.));
        let set1 = [
            Right(Open(43.)),
            Right(Open(42.)),
            Right(Closed(42.)),
            Right(Unbound),
        ];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    //   #[test]
    //   fn test_gt_5() {
    //       let b1 = Right(Unbound);
    //       let set1 = [Right(Closed(42.)), Right(Open(42.))];
    //
    //       for bound in set1 {
    //           assert!(b1.lt(&bound));
    //       }
    //   }

    #[test]
    fn test_gt_6() {
        let b1 = Right(Unbound);
        let set1 = [Right(Unbound)];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    #[test]
    fn test_min_1() {
        assert_eq!(
            Right(Closed(42.)).min(Right(Closed(42.))),
            Right(Closed(42.))
        );
    }

    #[test]
    fn test_min_2() {
        assert_eq!(Right(Closed(42.)).min(Right(Open(42.))), Right(Open(42.)));
    }

    #[test]
    fn test_min_3() {
        assert_eq!(Right(Closed(42.)).min(Right(Unbound)), Right(Closed(42.)));
    }

    #[test]
    fn test_min_4() {
        assert_eq!(Right(Open(42.)).min(Right(Closed(42.))), Right(Open(42.)));
    }

    #[test]
    fn test_min_5() {
        assert_eq!(Right(Open(42.)).min(Right(Open(42.))), Right(Open(42.)));
    }

    #[test]
    fn test_min_6() {
        assert_eq!(Right(Open(42.)).min(Right(Unbound)), Right(Open(42.)));
    }

    #[test]
    fn test_min_7() {
        assert_eq!(Right(Unbound).min(Right(Closed(42.))), Right(Closed(42.)));
    }

    #[test]
    fn test_min_8() {
        assert_eq!(Right(Unbound).min(Right(Open(42.))), Right(Open(42.)));
    }

    #[test]
    fn test_min_9() {
        assert_eq!(Right(Unbound).min(Right(Unbound)), Right(Unbound));
    }

    #[test]
    fn test_max_1() {
        assert_eq!(
            Right(Closed(42.)).max(Right(Closed(42.))),
            Right(Closed(42.))
        );
    }

    #[test]
    fn test_max_2() {
        assert_eq!(Right(Closed(42.)).max(Right(Open(42.))), Right(Closed(42.)));
    }

    #[test]
    fn test_max_3() {
        assert_eq!(Right(Closed(42.)).max(Right(Unbound)), Right(Unbound));
    }

    #[test]
    fn test_max_4() {
        assert_eq!(Right(Open(42.)).max(Right(Closed(42.))), Right(Closed(42.)));
    }

    #[test]
    fn test_max_5() {
        assert_eq!(Right(Open(42.)).max(Right(Open(42.))), Right(Open(42.)));
    }

    #[test]
    fn test_max_6() {
        assert_eq!(Right(Open(42.)).max(Right(Unbound)), Right(Unbound));
    }

    #[test]
    fn test_max_7() {
        assert_eq!(Right(Unbound).max(Right(Closed(42.))), Right(Unbound));
    }

    #[test]
    fn test_max_8() {
        assert_eq!(Right(Unbound).max(Right(Open(42.))), Right(Unbound));
    }

    #[test]
    fn test_max_9() {
        assert_eq!(Right(Unbound).max(Right(Unbound)), Right(Unbound));
    }

    #[test]
    fn test_closure_1() {
        assert!(Right(Closed(42.)).closure(Left(Closed(42.))));
    }

    #[test]
    fn test_closure_2() {
        assert!(Right(Closed(42.)).closure(Left(Open(42.))));
    }

    #[test]
    fn test_closure_3() {
        assert!(Right(Open(42.)).closure(Left(Closed(42.))));
    }

    #[test]
    fn test_closure_4() {
        assert!(!Right(Open(42.)).closure(Left(Open(42.))));
    }

    #[test]
    fn test_closure_5() {
        assert!(!Right(Closed(43.)).closure(Left(Closed(42.))));
    }

    #[test]
    fn test_fmt_1() {
        assert_eq!(format!("{}", Right(Closed(42.))), "42.00]");
    }

    #[test]
    fn test_fmt_2() {
        assert_eq!(format!("{}", Right(Open(42.))), "42.00)");
    }

    #[test]
    fn test_fmt_3() {
        assert_eq!(format!("{}", Right(Unbound)), "+∞)");
    }

    #[test]
    fn test_eql_1() {
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
                    (Left(Closed(l)), Right(Closed(r))) if l == r => assert!(right == left),
                    _ => assert!(left != right),
                }
            }
        }
    }

    #[test]
    fn test_ltl_1() {
        let b1 = Right(Closed(42.));
        let set1 = [Left(Closed(43.)), Left(Open(43.)), Left(Open(42.))];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_ltl_2() {
        let b1 = Right(Closed(42.));
        let set1 = [
            Left(Closed(42.)),
            Left(Closed(41.)),
            Left(Open(41.)),
            Left(Unbound),
        ];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_ltl_3() {
        let b1 = Right(Open(42.));
        let set1 = [
            Left(Closed(42.)),
            Left(Open(42.)),
            Left(Closed(43.)),
            Left(Open(43.)),
        ];

        for bound in set1 {
            assert!(b1.lt(&bound));
        }
    }

    #[test]
    fn test_ltl_4() {
        let b1 = Right(Open(42.));
        let set1 = [Left(Closed(41.)), Left(Open(41.)), Left(Unbound)];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    //   #[test]
    //   fn test_ltl_5() {
    //       let b1 = Right(Unbound);
    //       let set1 = [Left(Closed(42.)), Left(Open(42.)), Left(Unbound)];
    //
    //       for bound in set1 {
    //           assert!(b1.lt(&bound));
    //       }
    //   }

    #[test]
    fn test_ltl_6() {
        let b1 = Right(Unbound);
        let set1 = [Left(Closed(42.)), Left(Open(42.)), Left(Unbound)];

        for bound in set1 {
            assert!(!b1.lt(&bound));
        }
    }

    #[test]
    fn test_gtl_1() {
        let b1 = Right(Closed(42.));
        let set1 = [Left(Closed(41.)), Left(Open(41.)), Left(Unbound)];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gtl_2() {
        let b1 = Right(Closed(42.));
        let set1 = [Left(Closed(42.)), Left(Open(42.)), Left(Open(43.))];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    #[test]
    fn test_gtl_3() {
        let b1 = Right(Open(42.));
        let set1 = [Left(Closed(41.)), Left(Open(41.)), Left(Unbound)];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    #[test]
    fn test_gtl_4() {
        let b1 = Right(Open(42.));
        let set1 = [Left(Open(42.)), Left(Closed(42.))];

        for bound in set1 {
            assert!(!b1.gt(&bound));
        }
    }

    #[test]
    fn test_gtl_5() {
        let b1 = Right(Unbound);
        let set1 = [Left(Closed(42.)), Left(Open(42.)), Left(Unbound)];

        for bound in set1 {
            assert!(b1.gt(&bound));
        }
    }

    //   #[test]
    //   fn test_gtl_6() {
    //       let b1 = Right(Unbound);
    //       let set1 = [Left(Unbound), Left(Closed(42.)), Left(Open(42.))];
    //
    //       for bound in set1 {
    //           assert!(!b1.gt(&bound));
    //       }
    //   }
}
