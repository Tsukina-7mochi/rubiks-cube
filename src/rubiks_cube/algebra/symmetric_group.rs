use std::{
    array,
    fmt::Debug,
    ops::{self, Index, IndexMut},
};

use anyhow::anyhow;

/// Represents an element of the symmetric group S_N, which is the group of all permutations
/// of N elements. Each permutation is stored as an array where the value at index i represents
/// where element i maps to under the permutation.
#[derive(Clone, PartialEq, Eq)]
pub struct SymmetricGroup<const N: usize> {
    /// The permutation array where value[i] = j means element i maps to position j
    value: [usize; N],
}

impl<const N: usize> SymmetricGroup<N> {
    /// Returns identity permutation where each element maps to itself
    pub fn identity() -> Self {
        Self {
            value: array::from_fn(|i| i),
        }
    }

    /// Applies the permutation to the given input array and returns the permuted array
    pub fn permute<T>(&self, input: &[T; N]) -> [T; N]
    where
        T: Clone,
    {
        array::from_fn(|i| input[self.value[i]].clone())
    }
}

impl<const N: usize> Default for SymmetricGroup<N> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<const N: usize> Index<usize> for SymmetricGroup<N> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl<const N: usize> IndexMut<usize> for SymmetricGroup<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl<const N: usize> ops::Mul for &SymmetricGroup<N> {
    type Output = SymmetricGroup<N>;

    // define multiplication as composition of permutation.
    // for example, s1 * s2 represented as s2 is permutated by s1
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: rhs.permute(&self.value),
        }
    }
}

impl<const N: usize> ops::MulAssign<&Self> for SymmetricGroup<N> {
    fn mul_assign(&mut self, rhs: &Self) {
        self.value = rhs.permute(&self.value);
    }
}

impl<const N: usize> TryFrom<[usize; N]> for SymmetricGroup<N> {
    type Error = anyhow::Error;

    fn try_from(value: [usize; N]) -> Result<Self, Self::Error> {
        let mut flag = [false; N];
        for v in value {
            if v >= N {
                return Err(anyhow!("value {v} is too large for S_{N}"));
            }
            if flag[v] {
                return Err(anyhow!("duplicated value: {v}"));
            }

            flag[v] = true
        }

        Ok(Self { value })
    }
}

impl<const N: usize> Debug for SymmetricGroup<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SymmetricGroup({})[", N)?;
        for i in 0..N {
            write!(f, "{}", self.value[i])?;
            if i < N - 1 {
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
    fn test_permute() -> anyhow::Result<()> {
        let s: SymmetricGroup<4> = [2, 0, 3, 1].try_into()?;
        let input = ['a', 'b', 'c', 'd'];

        assert_eq!(s.permute(&input), ['c', 'a', 'd', 'b']);

        Ok(())
    }

    #[test]
    fn test_mul() -> anyhow::Result<()> {
        let s1: SymmetricGroup<3> = [2, 1, 0].try_into()?;
        let s2: SymmetricGroup<3> = [1, 2, 0].try_into()?;

        assert_eq!(&s2 * &s1, [0, 2, 1].try_into()?);
        assert_eq!(&s1 * &s2, [1, 0, 2].try_into()?);

        Ok(())
    }

    #[test]
    fn test_mul_identity() -> anyhow::Result<()> {
        let s: SymmetricGroup<3> = [2, 1, 0].try_into()?;

        assert_eq!(&s * &SymmetricGroup::identity(), s);

        Ok(())
    }

    #[test]
    fn test_mul_assign() -> anyhow::Result<()> {
        let s1: SymmetricGroup<3> = [2, 1, 0].try_into()?;
        let mut s2: SymmetricGroup<3> = [1, 2, 0].try_into()?;

        s2 *= &s1;

        assert_eq!(s2, [0, 2, 1].try_into()?);

        Ok(())
    }
}
