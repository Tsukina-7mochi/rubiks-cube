use super::Prunable;
use super::Searchable;

fn depth_limited_search_internal<T: Searchable + Prunable>(
    node: T,
    depth: usize,
    max_depth: usize,
) -> Option<T> {
    if depth >= max_depth {
        return None;
    }

    for n in node.next()?.into_iter() {
        if n.is_goal() {
            return Some(n);
        }
        if n.to_prune(depth, max_depth) {
            continue;
        }
        if let Some(result) = depth_limited_search_internal(n, depth + 1, max_depth) {
            return Some(result);
        }
    }

    None
}

pub fn depth_limited_search<T: Searchable + Prunable>(root: T, max_depth: usize) -> Option<T> {
    if root.is_goal() {
        return Some(root);
    }
    if root.to_prune(0, max_depth) {
        return None;
    }

    depth_limited_search_internal(root, 0, max_depth)
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
    fn test_depth_limited_search() {
        let tree = Tree::Node(vec![
            Tree::Node(vec![Tree::Node(vec![Tree::Leaf(1)]), Tree::Leaf(2)]),
            Tree::Leaf(3),
            Tree::Leaf(4),
        ]);

        assert_eq!(depth_limited_search(tree.clone(), 0), None);
        assert_eq!(depth_limited_search(tree.clone(), 1), Some(Tree::Leaf(3)));
        assert_eq!(depth_limited_search(tree.clone(), 2), Some(Tree::Leaf(3)));
    }

    #[test]
    fn test_depth_limited_search_root() {
        let tree = Tree::Leaf(1);

        assert_eq!(depth_limited_search(tree, 1), Some(Tree::Leaf(1)));
    }
}
