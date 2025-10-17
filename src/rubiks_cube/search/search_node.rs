use super::Searchable;

#[derive (Clone, Debug)]
pub struct SearchNode<T> where T: Searchable {
    item: T,
    depth: usize,
}
