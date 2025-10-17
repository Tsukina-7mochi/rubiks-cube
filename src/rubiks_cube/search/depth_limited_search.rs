use super::Searchable;

fn depth_limited_search_internal<T: Searchable>(
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
        if let Some(result) = depth_limited_search_internal(n, depth + 1, max_depth) {
            return Some(result);
        }
    }

    None
}

pub fn depth_limited_search<T: Searchable>(root: T, max_depth: usize) -> Option<T> {
    if root.is_goal() {
        return Some(root);
    }
    depth_limited_search_internal(root, 0, max_depth)
}

#[cfg(test)]
mod tests {
    use super::super::tree::Tree;
    use super::*;

    #[test]
    fn test_depth_limited_search() {
        let tree = Tree::Node(vec![
            Tree::Node(vec![Tree::Node(vec![Tree::Leaf(1)]), Tree::Leaf(2)]),
            Tree::Leaf(3),
        ]);

        assert_eq!(depth_limited_search(tree.clone(), 0), None);
        assert_eq!(depth_limited_search(tree.clone(), 1), Some(Tree::Leaf(3)));
        assert_eq!(depth_limited_search(tree.clone(), 2), Some(Tree::Leaf(2)));
        assert_eq!(depth_limited_search(tree.clone(), 3), Some(Tree::Leaf(1)));
    }

    #[test]
    fn test_depth_limited_search_root() {
        let tree = Tree::Leaf(1);

        assert_eq!(depth_limited_search(tree, 1), Some(Tree::Leaf(1)));
    }
}
