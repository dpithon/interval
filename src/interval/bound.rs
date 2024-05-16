#[derive(Debug, Clone, Copy)]
pub enum Bound {
    Open(f64),
    Closed(f64),
    Unbound,
}

use Bound::*;

impl PartialEq for Bound {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Open(k1), Open(k2)) => k1 == k2,
            (Closed(k1), Closed(k2)) => k1 == k2,
            (Unbound, Unbound) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let bounds = [Closed(42.), Open(42.), Unbound];

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
}
