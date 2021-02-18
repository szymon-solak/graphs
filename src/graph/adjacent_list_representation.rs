use std::collections::BTreeMap;

use super::graph_representation::GraphRepresentation;
use super::node::Node;

type Edges = BTreeMap<String, Vec<String>>;

pub struct AdjacentList {
    edges: Edges,
}

impl AdjacentList {
    pub fn new(nodes: Vec<Node>) -> AdjacentList {
        let mut list = BTreeMap::new();

        for node in nodes {
            list.insert(node.id, node.edges);
        }

        AdjacentList { edges: list }
    }

    fn get_node(&self, node_id: String) -> Node {
        Node::new(
            node_id.clone(),
            self.edges.get(&node_id).map_or(vec![], |e| e.to_owned()),
        )
    }
}

impl GraphRepresentation for AdjacentList {
    fn first(&self) -> Option<Node> {
        self.edges
            .keys()
            .next()
            .map(|id| self.get_node(id.to_owned()))
    }

    fn get_edges(&self, node_id: String) -> Vec<Node> {
        if let Some(edges) = self.edges.get(&node_id) {
            return edges.iter().map(|e| self.get_node(e.to_owned())).collect();
        }

        return vec![];
    }

    fn len(&self) -> usize {
        self.edges.iter().map(|(_, v)| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_list_can_be_created_from_single_node() {
        // given
        let node = Node::new("1".into(), vec![]);

        // when
        let list = AdjacentList::new(vec![node]);

        // then
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn adjacent_list_can_be_created_from_node_with_edges() {
        // given
        let node = Node::new("1".into(), vec!["2".into(), "3".into()]);

        // when
        let list = AdjacentList::new(vec![node]);

        // then
        assert_eq!(list.len(), 2);
    }
}
