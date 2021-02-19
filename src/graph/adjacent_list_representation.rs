use std::collections::BTreeMap;

use super::graph_representation::GraphRepresentation;
use super::node::Node;

pub struct AdjacentList {
    nodes: BTreeMap<String, Node>,
}

impl AdjacentList {
    pub fn new(nodes: Vec<Node>) -> AdjacentList {
        let mut list = BTreeMap::new();

        for node in nodes {
            for edge in &node.edges {
                if let None = list.get(edge) {
                    list.insert(edge.to_owned(), Node::new(edge.to_owned(), vec![]));
                }
            }

            list.insert(node.id.to_owned(), node);

        }

        AdjacentList { nodes: list }
    }

    fn get_node(&self, node_id: &String) -> Option<&Node> {
        self.nodes.get(node_id)
    }
}

impl GraphRepresentation for AdjacentList {
    fn first(&self) -> Option<&Node> {
        self.nodes
            .keys()
            .next()
            .map(|id| self.get_node(id))
            .unwrap_or(None)
    }

    fn get_edges(&self, node_id: &String) -> Vec<&Node> {
        if let Some(n) = self.nodes.get(node_id) {
            return n.edges.iter().filter_map(|e| self.get_node(e)).collect();
        }

        return vec![];
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|(_, v)| v.edges.len()).sum()
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
