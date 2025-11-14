pub trait Prunable {
    fn to_prune(&self, depth: usize, max_depth: usize) -> bool;
}
