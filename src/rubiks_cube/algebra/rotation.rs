use std::ops::{Mul, MulAssign};

use super::quotient_group_vec::QuotientGroupVec;
use super::symmetric_group::SymmetricGroup;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rotation {
    edge_perm: SymmetricGroup<12>,
    edge_orient: QuotientGroupVec<2, 12>,
    corner_perm: SymmetricGroup<8>,
    corner_orient: QuotientGroupVec<3, 8>,
}

impl Rotation {
    fn new(
        edge_perm: SymmetricGroup<12>,
        edge_orient: QuotientGroupVec<2, 12>,
        corner_perm: SymmetricGroup<8>,
        corner_orient: QuotientGroupVec<3, 8>,
    ) -> Self {
        Self {
            edge_perm,
            edge_orient,
            corner_perm,
            corner_orient,
        }
    }
}

impl Mul for &Rotation {
    type Output = Rotation;

    fn mul(self, rhs: Self) -> Self::Output {
        let perm_edge_orient = rhs.edge_perm.permute(&self.edge_orient.into()).into();
        let perm_corner_orient = rhs.corner_perm.permute(&self.corner_orient.into()).into();

        Self::Output::new(
            &self.edge_perm * &rhs.edge_perm,
            &perm_edge_orient + &rhs.edge_orient,
            &self.corner_perm * &rhs.corner_perm,
            &perm_corner_orient + &rhs.corner_orient,
        )
    }
}

impl MulAssign<&Rotation> for Rotation {
    fn mul_assign(&mut self, rhs: &Rotation) {
        self.edge_perm *= &rhs.edge_perm;

        self.edge_orient = rhs.edge_perm.permute(&self.edge_orient.into()).into();
        self.edge_orient += rhs.edge_orient;

        self.corner_perm *= &rhs.corner_perm;

        self.corner_orient = rhs.corner_perm.permute(&self.corner_orient.into()).into();
        self.corner_orient += rhs.corner_orient;
    }
}

pub mod rotations {
    use super::*;
    use std::sync::LazyLock;

    pub static R: LazyLock<Rotation> = LazyLock::new(|| {
        Rotation::new(
            [0, 5, 9, 3, 4, 2, 6, 7, 8, 1, 10, 11].try_into().unwrap(),
            [0; 12].into(),
            [0, 2, 6, 3, 4, 1, 5, 7].try_into().unwrap(),
            [0, 1, 2, 0, 0, 2, 1, 0].into(),
        )
    });

    pub static L: LazyLock<Rotation> = LazyLock::new(|| {
        Rotation::new(
            [11, 1, 2, 7, 4, 5, 6, 0, 8, 9, 10, 3].try_into().unwrap(),
            [0; 12].into(),
            [4, 1, 2, 0, 7, 5, 6, 3].try_into().unwrap(),
            [2, 0, 0, 1, 1, 0, 0, 2].into(),
        )
    });

    pub static U: LazyLock<Rotation> = LazyLock::new(|| {
        Rotation::new(
            [0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11].try_into().unwrap(),
            [0; 12].into(),
            [3, 0, 1, 2, 4, 5, 6, 7].try_into().unwrap(),
            [0; 8].into(),
        )
    });

    pub static D: LazyLock<Rotation> = LazyLock::new(|| {
        Rotation::new(
            [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8].try_into().unwrap(),
            [0; 12].into(),
            [0, 1, 2, 3, 5, 6, 7, 4].try_into().unwrap(),
            [0; 8].into(),
        )
    });

    pub static F: LazyLock<Rotation> = LazyLock::new(|| {
        Rotation::new(
            [0, 1, 6, 10, 4, 5, 3, 7, 8, 9, 2, 11].try_into().unwrap(),
            [0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0].into(),
            [0, 1, 3, 7, 4, 5, 2, 6].try_into().unwrap(),
            [0, 0, 1, 2, 0, 0, 2, 1].into(),
        )
    });

    pub static B: LazyLock<Rotation> = LazyLock::new(|| {
        Rotation::new(
            [4, 8, 2, 3, 1, 5, 6, 7, 0, 9, 10, 11].try_into().unwrap(),
            [1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0].into(),
            [1, 5, 2, 3, 0, 4, 6, 7].try_into().unwrap(),
            [1, 2, 0, 0, 2, 1, 0, 0].into(),
        )
    });

    pub static R2: LazyLock<Rotation> = LazyLock::new(|| &*R * &*R);
    pub static R3: LazyLock<Rotation> = LazyLock::new(|| &*R2 * &*R);

    pub static L2: LazyLock<Rotation> = LazyLock::new(|| &*L * &*L);
    pub static L3: LazyLock<Rotation> = LazyLock::new(|| &*L2 * &*L);

    pub static U2: LazyLock<Rotation> = LazyLock::new(|| &*U * &*U);
    pub static U3: LazyLock<Rotation> = LazyLock::new(|| &*U2 * &*U);

    pub static D2: LazyLock<Rotation> = LazyLock::new(|| &*D * &*D);
    pub static D3: LazyLock<Rotation> = LazyLock::new(|| &*D2 * &*D);

    pub static F2: LazyLock<Rotation> = LazyLock::new(|| &*F * &*F);
    pub static F3: LazyLock<Rotation> = LazyLock::new(|| &*F2 * &*F);

    pub static B2: LazyLock<Rotation> = LazyLock::new(|| &*B * &*B);
    pub static B3: LazyLock<Rotation> = LazyLock::new(|| &*B2 * &*B);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_assign() {
        let r1 = &*rotations::R * &*rotations::R;

        let mut r2 = rotations::R.clone();
        r2 *= &*rotations::R;

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_r() {
        let mut rot = Rotation::default();
        rot *= &*rotations::R;
        rot *= &*rotations::R;
        rot *= &*rotations::R;
        rot *= &*rotations::R;

        assert_eq!(rot, Rotation::default());
    }

    #[test]
    fn test_l() {
        let mut rot = Rotation::default();
        rot *= &*rotations::L;
        rot *= &*rotations::L;
        rot *= &*rotations::L;
        rot *= &*rotations::L;

        assert_eq!(rot, Rotation::default());
    }

    #[test]
    fn test_u() {
        let mut rot = Rotation::default();
        rot *= &*rotations::U;
        rot *= &*rotations::U;
        rot *= &*rotations::U;
        rot *= &*rotations::U;

        assert_eq!(rot, Rotation::default());
    }

    #[test]
    fn test_d() {
        let mut rot = Rotation::default();
        rot *= &*rotations::D;
        rot *= &*rotations::D;
        rot *= &*rotations::D;
        rot *= &*rotations::D;

        assert_eq!(rot, Rotation::default());
    }

    #[test]
    fn test_f() {
        let mut rot = Rotation::default();
        rot *= &*rotations::F;
        rot *= &*rotations::F;
        rot *= &*rotations::F;
        rot *= &*rotations::F;

        assert_eq!(rot, Rotation::default());
    }

    #[test]
    fn test_b() {
        let mut rot = Rotation::default();
        rot *= &*rotations::B;
        rot *= &*rotations::B;
        rot *= &*rotations::B;
        rot *= &*rotations::B;

        assert_eq!(rot, Rotation::default());
    }

    #[test]
    fn test_r_r3() {
        assert_eq!(&*rotations::R * &*rotations::R3, Rotation::default());
    }

    #[test]
    fn test_l_l3() {
        assert_eq!(&*rotations::L * &*rotations::L3, Rotation::default());
    }

    #[test]
    fn test_u_u3() {
        assert_eq!(&*rotations::U * &*rotations::U3, Rotation::default());
    }

    #[test]
    fn test_d_d3() {
        assert_eq!(&*rotations::D * &*rotations::D3, Rotation::default());
    }

    #[test]
    fn test_f_f3() {
        assert_eq!(&*rotations::F * &*rotations::F3, Rotation::default());
    }

    #[test]
    fn test_b_b3() {
        assert_eq!(&*rotations::B * &*rotations::B3, Rotation::default());
    }
}
