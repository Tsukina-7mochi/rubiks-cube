use super::Prunable;
use super::Searchable;
use super::depth_limited_search;

pub fn iterative_deepening_dfs<T: Searchable + Prunable>(root: T, max_depth: usize) -> Option<T> {
    for depth in 0..=max_depth {
        if let Some(result) = depth_limited_search(root.clone(), depth) {
            return Some(result);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Tree {
        Node(Vec<Tree>),
        Leaf(i32),
    }

    impl Searchable for Tree {
        fn is_goal(&self) -> bool {
            match self {
                Tree::Node(_) => false,
                Tree::Leaf(_) => true,
            }
        }

        fn next(&self) -> Option<impl IntoIterator<Item = Self>> {
            match self {
                Tree::Node(v) => Some(v.clone()),
                Tree::Leaf(_) => None,
            }
        }
    }

    impl Prunable for Tree {
        fn to_prune(&self, _depth: usize, _max_depth: usize) -> bool {
            if let Tree::Node(v) = self {
                v.len() == 2
            } else {
                false
            }
        }
    }

    #[test]
    fn test_iterative_deepening_dfs() {
        let tree = Tree::Node(vec![
            Tree::Node(vec![Tree::Node(vec![Tree::Leaf(1)]), Tree::Leaf(2)]),
            Tree::Node(vec![Tree::Node(vec![Tree::Leaf(3)]), Tree::Leaf(4)]),
            Tree::Node(vec![Tree::Leaf(5), Tree::Leaf(6), Tree::Leaf(7)]),
        ]);

        assert_eq!(iterative_deepening_dfs(tree, 3), Some(Tree::Leaf(5)));
    }

    #[test]
    fn test_iterative_deepening_dfs_root() {
        let tree = Tree::Leaf(1);

        assert_eq!(iterative_deepening_dfs(tree, 1), Some(Tree::Leaf(1)));
    }
}
