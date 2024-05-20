use super::{Interval, INFINITY};
use auto_ops::impl_op_ex;
use std::fmt::Display;

#[derive(Default, Clone)]
pub struct IntervalSet {
    union: Vec<Interval>,
}

impl Display for IntervalSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "∅")
        } else {
            let (head, tail) = (self.union[0], &self.union[1..]);
            write!(f, "{head}")?;
            for i in tail {
                write!(f, " U {i}")?;
            }
            Ok(())
        }
    }
}

impl IntervalSet {
    pub fn new() -> Self {
        IntervalSet { union: Vec::new() }
    }

    pub fn from(array: &[Interval]) -> Self {
        let mut i = IntervalSet::new();
        for segment in array {
            i = i.union_interval(segment);
        }
        i
    }

    pub fn is_empty(&self) -> bool {
        self.union.len() == 0
    }

    pub fn is_infinity(&self) -> bool {
        self.union.len() == 1 && self.union[0] == INFINITY
    }

    pub fn union_interval(&self, interval: &Interval) -> Self {
        let mut res = IntervalSet::new();
        let mut current = *interval;

        for (i, segment) in self.union.iter().enumerate() {
            match current.union(*segment) {
                (a, Some(b)) if a == current && b == *segment => {
                    res.union.push(current);
                    res.union.extend_from_slice(&self.union[i..]);
                    return res;
                }
                (_, Some(_)) => {
                    res.union.push(*segment);
                }
                (new, None) => {
                    current = new;
                }
            }
        }

        if !current.is_empty() {
            res.union.push(current);
        }
        res
    }

    pub fn union_intervals(&self, intervals: &IntervalSet) -> Self {
        let mut res = self.clone();
        for segment in intervals.union.iter() {
            res = res.union_interval(segment)
        }
        res
    }
}

impl PartialEq for IntervalSet {
    fn eq(&self, other: &Self) -> bool {
        if self.union.len() != other.union.len() {
            return false;
        }

        if self.is_empty() && other.is_empty() {
            return true;
        }

        if self.is_infinity() && other.is_infinity() {
            return true;
        }

        for (i, segment) in self.union.iter().enumerate() {
            if *segment != other.union[i] {
                return false;
            }
        }

        true
    }
}

impl_op_ex!(| |lhs: &IntervalSet, rhs: &Interval| -> IntervalSet {
    lhs.union_interval(rhs)
});

impl_op_ex!(| |lhs: &Interval, rhs: &IntervalSet| -> IntervalSet {
    rhs.union_interval(lhs)
});

impl_op_ex!(| |lhs: &IntervalSet, rhs: &IntervalSet| -> IntervalSet {
    lhs.union_intervals(rhs)
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Closed, EMPTY};

    #[test]
    fn test_empty_1() {
        let e = IntervalSet::new();
        assert!(e.is_empty());
    }

    #[test]
    fn test_union_empty_1() {
        let a = IntervalSet::new();
        let b = a | EMPTY;
        assert!(b.is_empty());
    }

    #[test]
    fn test_union_empty_2() {
        let a = IntervalSet::new();
        let b = EMPTY | a;
        assert!(b.is_empty());
    }

    #[test]
    fn test_union_empty_3() {
        let a = IntervalSet::new();
        let b = Interval::new(Closed(42.), Closed(43.));
        let c = a | b;
        assert!(!c.is_empty());
        assert_eq!(c.union[0], b);
    }

    #[test]
    fn test_union_empty_4() {
        let a = IntervalSet::new();
        let b = Interval::new(Closed(42.), Closed(43.));
        let c = b | a;
        assert!(!c.is_empty());
        assert_eq!(c.union[0], b);
    }

    #[test]
    fn test_union_infinity_1() {
        let a = IntervalSet::new();
        let b = INFINITY;
        let c = b | a;
        assert!(c.is_infinity());
    }

    #[test]
    fn test_union_infinity_2() {
        let a = IntervalSet::new() | INFINITY;
        let b = Interval::new(Closed(42.), Closed(43.));

        assert!((a | b).is_infinity());
    }
}
