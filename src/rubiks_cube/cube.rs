use rand::prelude::*;

use super::Operation;
use super::algebra::rotation::Rotation;

/// Represents Rubik's Cube state by edge permutation, edge orientation,
/// corner permutation and corner orientation.
///
/// Faces:
///   +-+
///   |U|
/// +-+-+-+-+
/// |L|F|R|B|
/// +-+-+-+-+
///   |D|
///   +-+
///
/// Edges (0 to 11):
///   BL, BR, FR, FL, UB, UR, UF, UL, DB, DR, DF, DL
///
///   The position of edge parts is represented as permutation
///   from their initial positions.
///
///   All edge pieces have orientation, and their orientation changes
///   with rotation (when considering the edge at a corner, its orientation is
///   defined by the colors on either side). The orientation of a piece is
///   defined by comparison with the orientation of the piece that originally
///   occupied that position. The value is 0 if they are the same,
///   and 1 if they are different.
///
/// Corners (0 to 7):
///   UBR, UFR, UFL, UBL, DBR, DFB, DFL, DBL
///
///   The corner of edge parts is represented as permutation
///   from their initial positions.
///
///   Corner pieces, like edge pieces, have orientation. The orientation of
///   a piece is defined by how many times it has rotated from the
///   orientation of the piece that was originally there. This number
///   corresponds to clockwise rotations of 0, 1, or 2 when viewed from
///   the outside of the cube.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cube {
    pub rotation: Rotation,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            rotation: Rotation::default(),
        }
    }

    pub fn random(steps: usize) -> (Self, Vec<Operation>) {
        let mut rng = rand::rng();
        let all_operations = Operation::all();
        let operations = (0..steps)
            .map(|_| all_operations.choose(&mut rng).unwrap().to_owned())
            .collect::<Vec<_>>();

        let mut cube = Self::new();
        for operation in &operations {
            cube.apply_operation(operation);
        }

        (cube, operations)
    }

    pub fn apply_operation(&mut self, operation: &Operation) {
        self.rotation *= operation.rotation();
    }

    pub fn operation_applied(&self, operation: &Operation) -> Self {
        Self {
            rotation: &self.rotation * operation.rotation(),
        }
    }
}
