use super::node::Node;

pub trait GraphRepresentation {
    fn len(&self) -> usize;
    fn first(&self) -> Option<&Node>;
    fn get_edges(&self, node_id: &String) -> Vec<&Node>;
    fn get_nodes(&self) -> Vec<&Node>;
}
