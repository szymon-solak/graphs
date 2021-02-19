use crate::graph::graph_representation::GraphRepresentation;
use crate::graph::node::Node;
use std::collections::HashMap;

pub struct Dfs<'a>(pub &'a dyn GraphRepresentation);

impl<'a> IntoIterator for &'a Dfs<'a> {
    type Item = &'a Node;
    type IntoIter = DfsIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = vec![];
        let visited = vec![];

        let iters = self
            .0
            .get_nodes()
            .iter()
            .map(|n| (&n.id, self.0.get_edges(&n.id)))
            .map(|(id, edge_list)| (id, edge_list.into_iter()))
            .collect();

        if let Some(node) = self.0.first() {
            stack.push(node);
        }

        DfsIntoIterator {
            stack,
            visited,
            iters,
        }
    }
}

pub struct DfsIntoIterator<'a> {
    stack: Vec<&'a Node>,
    visited: Vec<&'a String>,
    iters: HashMap<&'a String, std::vec::IntoIter<&'a Node>>,
}

impl<'a> Iterator for DfsIntoIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&current) = self.stack.last() {
            if let Some(neighbour) = self.iters.get_mut(&current.id).unwrap().next() {
                if !self.visited.contains(&&neighbour.id) {
                    self.stack.push(&neighbour);
                }
            } else {
                self.stack.pop();
            }

            if !self.visited.contains(&&current.id) {
                self.visited.push(&current.id);
                return Some(current);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms;
    use crate::graph::{adjacent_list_representation::AdjacentList, node::Node};

    #[test]
    fn dfs_goes_through_all_connected_nodes_when_the_deepest_node_is_the_first() {
        // given
        let graph = AdjacentList::new(vec![
            Node::new("1".into(), vec!["2".into(), "3".into()]),
            Node::new("2".into(), vec!["4".into()]),
            Node::new("5".into(), vec![]),
        ]);

        // when
        let visited = algorithms::Dfs(&graph)
            .into_iter()
            .map(|n| n.id.to_owned())
            .collect::<Vec<String>>();

        // then
        assert_eq!(visited, vec!["1", "2", "4", "3"]);
    }

    #[test]
    fn dfs_goes_through_all_connected_nodes_when_the_deepest_node_is_not_the_first() {
        // given
        let graph = AdjacentList::new(vec![
            Node::new("1".into(), vec!["2".into(), "3".into()]),
            Node::new("2".into(), vec!["4".into(), "5".into()]),
            Node::new("3".into(), vec!["7".into()]),
            Node::new("5".into(), vec!["6".into()]),
        ]);

        // when
        let visited = algorithms::Dfs(&graph)
            .into_iter()
            .map(|n| n.id.to_owned())
            .collect::<Vec<String>>();

        // then
        assert_eq!(visited, vec!["1", "2", "4", "5", "6", "3", "7"]);
    }
}
