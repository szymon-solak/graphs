pub trait GraphRepresentation<TIndex, TNode> {
    fn len(&self) -> usize;
    fn first(&self) -> Option<&TNode>;
    fn first_index(&self) -> Option<&TIndex>;
    fn get_edges(&self, node_id: &TIndex) -> Vec<&TIndex>;
    fn get_node(&self, node_id: &TIndex) -> Option<&TNode>;
    fn get_nodes(&self) -> Vec<(&TIndex, &TNode)>;
}
