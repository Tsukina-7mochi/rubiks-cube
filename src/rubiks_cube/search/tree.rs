use super::Searchable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Node(Vec<Tree<T>>),
    Leaf(T),
}

impl<T: Clone> Searchable for Tree<T> {
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
