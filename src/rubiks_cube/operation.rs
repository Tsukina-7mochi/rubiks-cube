use std::fmt::Display;

use super::algebra::rotation::{rotations, Rotation};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    R,
    R2,
    R3,
    L,
    L2,
    L3,
    U,
    U2,
    U3,
    D,
    D2,
    D3,
    F,
    F2,
    F3,
    B,
    B2,
    B3,
}

impl Operation {
    pub fn rotation(&self) -> &'static Rotation {
        match self {
            Operation::R => &rotations::R,
            Operation::R2 => &rotations::R2,
            Operation::R3 => &rotations::R3,
            Operation::L => &rotations::L,
            Operation::L2 => &rotations::L2,
            Operation::L3 => &rotations::L3,
            Operation::U => &rotations::U,
            Operation::U2 => &rotations::U2,
            Operation::U3 => &rotations::U3,
            Operation::D => &rotations::D,
            Operation::D2 => &rotations::D2,
            Operation::D3 => &rotations::D3,
            Operation::F => &rotations::F,
            Operation::F2 => &rotations::F2,
            Operation::F3 => &rotations::F3,
            Operation::B => &rotations::B,
            Operation::B2 => &rotations::B2,
            Operation::B3 => &rotations::B3,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operation::R => "R",
            Operation::R2 => "R2",
            Operation::R3 => "R'",
            Operation::L => "L",
            Operation::L2 => "L2",
            Operation::L3 => "L'",
            Operation::U => "U",
            Operation::U2 => "U2",
            Operation::U3 => "U'",
            Operation::D => "D",
            Operation::D2 => "D2",
            Operation::D3 => "D'",
            Operation::F => "F",
            Operation::F2 => "F2",
            Operation::F3 => "F'",
            Operation::B => "B",
            Operation::B2 => "B2",
            Operation::B3 => "B'",
        };
        write!(f, "{s}")
    }
}
