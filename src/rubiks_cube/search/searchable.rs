pub trait Searchable: Sized + Clone {
    fn next(&self) -> Option<impl IntoIterator<Item = Self>>;

    fn is_goal(&self) -> bool;
}
