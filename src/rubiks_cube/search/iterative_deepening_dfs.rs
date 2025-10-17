use super::{Searchable, depth_limited_search};

pub fn iterative_deepening_dfs<T: Searchable>(root: T, max_depth: usize) -> Option<T> {
    for depth in 0..=max_depth {
        if let Some(result) = depth_limited_search(root.clone(), depth) {
            return Some(result);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::super::tree::Tree;
    use super::*;

    #[test]
    fn test_iterative_deepening_dfs() {
        let tree = Tree::Node(vec![
            Tree::Node(vec![Tree::Node(vec![Tree::Leaf(1)]), Tree::Leaf(2)]),
            Tree::Leaf(3),
        ]);

        assert_eq!(iterative_deepening_dfs(tree, 3), Some(Tree::Leaf(3)));
    }

    #[test]
    fn test_iterative_deepening_dfs_root() {
        let tree = Tree::Leaf(1);

        assert_eq!(iterative_deepening_dfs(tree, 1), Some(Tree::Leaf(1)));
    }
}
