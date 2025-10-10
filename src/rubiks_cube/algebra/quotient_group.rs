use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
};

/// A quotient group Z/NZ representing integers modulo N.
///
/// This type implements the mathematical quotient group structure where elements
/// are equivalence classes of integers under modulo N arithmetic. All operations
/// maintain the invariant that the internal value is always in the range [0, N).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QuotientGroup<const N: u8> {
    value: u8,
}

impl<const N: u8> QuotientGroup<N> {
    /// Creates a new quotient group element from the equivalence class of the
    /// given value. The representative `value % N` is stored as value.
    fn equivalence_class(value: u8) -> Self {
        Self { value: value % N }
    }
}

impl<const N: u8> Default for QuotientGroup<N> {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl<const N: u8> Add for QuotientGroup<N> {
    type Output = QuotientGroup<N>;

    fn add(self, rhs: Self) -> Self::Output {
        (self.value + rhs.value).into()
    }
}

impl<const N: u8> AddAssign for QuotientGroup<N> {
    fn add_assign(&mut self, rhs: QuotientGroup<N>) {
        self.value = (self.value + rhs.value) % N
    }
}

impl<const N: u8> From<u8> for QuotientGroup<N> {
    fn from(value: u8) -> Self {
        Self::equivalence_class(value)
    }
}

impl<const N: u8> From<QuotientGroup<N>> for u8 {
    fn from(value: QuotientGroup<N>) -> Self {
        value.value
    }
}

impl<const N: u8> Debug for QuotientGroup<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QuotientGroup({N})({})", self.value)
    }
}

impl<const N: u8> Display for QuotientGroup<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = QuotientGroup::<6>::from(3);
        let y = QuotientGroup::<6>::from(2);

        assert_eq!(x + y, 5.into());
    }

    #[test]
    fn test_add_large() {
        let x = QuotientGroup::<6>::from(4);
        let y = QuotientGroup::<6>::from(5);

        assert_eq!(x + y, 3.into());
    }

    #[test]
    fn test_add_assign() {
        let mut x = QuotientGroup::<6>::from(3);

        x += 2.into();

        assert_eq!(x, 5.into());
    }
}
