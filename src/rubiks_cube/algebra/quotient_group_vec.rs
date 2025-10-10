use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index},
};

use super::quotient_group::QuotientGroup;

/// A fixed-size vector of quotient group elements.
/// Represents a vector of `M` elements from the quotient group Z/NZ.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QuotientGroupVec<const N: u8, const M: usize> {
    value: [QuotientGroup<N>; M],
}

impl<const N: u8, const M: usize> Default for QuotientGroupVec<N, M> {
    fn default() -> Self {
        Self {
            value: [QuotientGroup::<N>::default(); M],
        }
    }
}

impl<const N: u8, const M: usize> Index<usize> for QuotientGroupVec<N, M> {
    type Output = QuotientGroup<N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl<const N: u8, const M: usize> Add for &QuotientGroupVec<N, M> {
    type Output = QuotientGroupVec<N, M>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: std::array::from_fn(|i| self.value[i] + rhs.value[i]),
        }
    }
}

impl<const N: u8, const M: usize> AddAssign for QuotientGroupVec<N, M> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M {
            self.value[i] += rhs.value[i];
        }
    }
}

impl<const N: u8, const M: usize> From<[u8; M]> for QuotientGroupVec<N, M> {
    fn from(value: [u8; M]) -> Self {
        Self {
            value: std::array::from_fn(|i| QuotientGroup::<N>::from(value[i])),
        }
    }
}

impl<const N: u8, const M: usize> From<QuotientGroupVec<N, M>> for [u8; M] {
    fn from(value: QuotientGroupVec<N, M>) -> Self {
        value.value.map(|x| x.into())
    }
}

impl<const N: u8, const M: usize> Debug for QuotientGroupVec<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QuotientGroupVec({N})[")?;
        for i in 0..M {
            write!(f, "{}", self.value[i])?;

            if i < M - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = QuotientGroupVec::<6, 3>::from([3, 4, 5]);
        let y = QuotientGroupVec::<6, 3>::from([2, 5, 2]);

        assert_eq!(&x + &y, [5, 3, 1].into());
    }

    #[test]
    fn test_add_assign() {
        let mut x = QuotientGroupVec::<6, 3>::from([3, 2, 1]);

        x += [2, 1, 4].into();

        assert_eq!(x, [5, 3, 5].into());
    }
}
