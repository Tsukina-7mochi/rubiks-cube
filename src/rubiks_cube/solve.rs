use std::rc::Rc;

use super::algebra::rotation::Rotation;
use super::search;
use super::search::{Prunable, Searchable};
use super::{Cube, Operation};

#[derive(Debug, Clone)]
struct SearchNode {
    rotation: Rotation,
    parent: Option<(Rc<SearchNode>, Operation)>,
}

impl Searchable for Rc<SearchNode> {
    fn is_goal(&self) -> bool {
        // let mut result = vec![];
        // let mut current = self.clone();
        // loop {
        //     if let Some((parent, op)) = &current.parent {
        //         result.push(*op);
        //         current = parent.clone();
        //     } else {
        //         break;
        //     }
        // }
        // println!("{:?}", result.into_iter().rev().collect::<Vec<_>>());

        self.rotation == Rotation::default()
    }

    fn next(&self) -> Option<impl IntoIterator<Item = Self>> {
        let parent_operation = self.parent.as_ref().map(|(_, op)| op);

        let operations = if let Some(parent_operation) = parent_operation {
            use Operation::*;
            match parent_operation {
                R | R2 | R3 => vec![
                    Operation::up(),
                    Operation::down(),
                    Operation::front(),
                    Operation::back(),
                    Operation::left(),
                ]
                .concat(),
                L | L2 | L3 => vec![
                    Operation::up(),
                    Operation::down(),
                    Operation::front(),
                    Operation::back(),
                ]
                .concat(),
                U | U2 | U3 => vec![
                    Operation::right(),
                    Operation::left(),
                    Operation::front(),
                    Operation::back(),
                    Operation::down(),
                ]
                .concat(),
                D | D2 | D3 => vec![
                    Operation::right(),
                    Operation::left(),
                    Operation::front(),
                    Operation::back(),
                ]
                .concat(),
                F | F2 | F3 => vec![
                    Operation::right(),
                    Operation::left(),
                    Operation::up(),
                    Operation::down(),
                    Operation::back(),
                ]
                .concat(),
                B | B2 | B3 => vec![
                    Operation::right(),
                    Operation::left(),
                    Operation::up(),
                    Operation::down(),
                ]
                .concat(),
            }
        } else {
            Operation::all()
        };

        let nodes = operations
            .into_iter()
            .map(|op| {
                let new_rotation = &self.rotation * op.rotation();
                Rc::new(SearchNode {
                    rotation: new_rotation,
                    parent: Some((self.clone(), op)),
                })
            })
            .collect::<Vec<_>>();

        Some(nodes)
    }
}

impl Prunable for Rc<SearchNode> {
    fn to_prune(&self, depth: usize, max_depth: usize) -> bool {
        let remaining_depth = max_depth - depth;
        if remaining_depth == 1 {
            self.rotation.edge_hamming_distance(&Rotation::default()) > 4
                || self.rotation.corner_hamming_distance(&Rotation::default()) > 4
        } else if remaining_depth == 2 {
            self.rotation.edge_hamming_distance(&Rotation::default()) > 8
        } else if remaining_depth == 3 {
            self.rotation.edge_hamming_distance(&Rotation::default()) > 10
        } else {
            false
        }
    }
}

pub fn solve(cube: Cube, max_depth: usize) -> Option<Vec<Operation>> {
    let node = Rc::new(SearchNode {
        rotation: cube.rotation.clone(),
        parent: None,
    });
    let result_node = search::iterative_deepening_dfs(node, max_depth)?;

    let mut result = vec![];
    let mut current = &result_node;
    loop {
        if let Some((parent, op)) = &current.parent {
            result.push(*op);
            current = &parent;
        } else {
            break;
        }
    }

    Some(result.into_iter().rev().collect())
}
