pub trait Node {
    /// Replaces a node with a new one
    fn update(self, replacement: Self);

    /// Deletes a node
    fn delete(self);

    fn get_parent(&self) -> Option<&Self>;

    fn get_parent_mut(&mut self) -> Option<&mut Self>;

    fn get_children(&self) -> [&Self];

    fn get_children_mut(&mut self) -> &mut Vec<&mut Self>;

    fn is_root(&self) -> bool;
}
