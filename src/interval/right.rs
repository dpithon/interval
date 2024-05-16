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

    pub fn closure(self) -> Self {
        match self {
            Right(Closed(k)) | Right(Open(k)) => Right(Closed(k)),
            _ => self,
        }
    }
}

impl Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Right(Closed(k)) => write!(f, "{k:5.2}]"),
            Right(Open(k)) => write!(f, "{k:5.2}["),
            Right(Unbound) => write!(f, "+∞["),
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
