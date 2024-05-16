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

    pub fn closure(self) -> Self {
        match self {
            Left(Closed(k)) | Left(Open(k)) => Left(Closed(k)),
            _ => self,
        }
    }
}

impl Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Left(bound) = self;
        match bound {
            Closed(k) => write!(f, "[{k:5.2}"),
            Open(k) => write!(f, "]{k:5.2}"),
            Unbound => write!(f, "]-∞"),
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
    use super::Right;
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
        let lefts = [Left(Closed(43.)), Left(Open(42.)), Left(Open(43.))];

        let rights = [Right(Unbound), Right(Open(43.))];
        for bound in lefts {
            dbg!(bound);
            assert!(dbg!(b1.lt(&bound)));
        }
        for bound in rights {
            dbg!(bound);
            assert!(dbg!(b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_1r() {
        let b1 = Right(Closed(42.));
        let lefts = [Left(Closed(43.)), Left(Open(42.)), Left(Open(43.))];

        let rights = [Right(Unbound), Right(Open(43.))];
        for bound in lefts {
            dbg!(bound);
            assert!(dbg!(b1.lt(&bound)));
        }
        for bound in rights {
            dbg!(bound);
            assert!(dbg!(b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_2() {
        let b1 = Left(Closed(42.));
        let lefts = [Left(Closed(41.)), Left(Unbound), Left(Open(41.))];
        let rights = [Right(Closed(40.)), Right(Open(42.)), Right(Open(41.))];

        for bound in lefts {
            assert!(dbg!(bound.lt(&b1)));
        }
        for bound in rights {
            assert!(dbg!(bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_2r() {
        let b1 = Right(Closed(42.));
        let lefts = [Left(Closed(41.)), Left(Unbound), Left(Open(41.))];
        let rights = [Right(Closed(40.)), Right(Open(42.)), Right(Open(41.))];

        for bound in lefts {
            assert!(dbg!(bound.lt(&b1)));
        }
        for bound in rights {
            assert!(dbg!(bound.lt(&b1)));
        }
    }

    #[test]
    fn test_lt_3() {
        let b1 = Left(Closed(42.));
        let lefts = [Left(Closed(41.)), Left(Unbound), Left(Open(41.))];
        let rights = [Right(Closed(41.)), Right(Open(42.)), Right(Open(41.))];

        for bound in lefts {
            assert!(dbg!(!b1.lt(&bound)));
        }
        for bound in rights {
            assert!(dbg!(!b1.lt(&bound)));
        }
    }

    #[test]
    fn test_lt_3r() {
        let b1 = Right(Closed(42.));
        let lefts = [Left(Closed(41.)), Left(Unbound), Left(Open(41.))];
        let rights = [Right(Closed(41.)), Right(Open(42.)), Right(Open(41.))];

        for bound in lefts {
            assert!(dbg!(!b1.lt(&bound)));
        }
        for bound in rights {
            assert!(dbg!(!b1.lt(&bound)));
        }
    }
}
