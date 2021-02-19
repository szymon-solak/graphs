use std::cell::RefCell;
use crate::graph::graph_representation::GraphRepresentation;
use crate::graph::node::Node;

pub struct Dfs<'a>(
    pub &'a dyn GraphRepresentation,
);

impl<'a> IntoIterator for &'a Dfs<'a> {
    type Item = &'a Node;
    type IntoIter = DfsIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DfsIntoIterator {
            inner: self.0,
            current_node: self.0.first(),
            iter_stack: vec![],
        }
    }
}

pub struct DfsIntoIterator<'a> {
    inner: &'a dyn GraphRepresentation,
    iter_stack: Vec<RefCell<Vec<&'a Node>>>,
    current_node: Option<&'a Node>,
}

impl<'a> Iterator for DfsIntoIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_node;
        
        if let Some(node) = self.current_node {
            if let Some(next) = self.inner.get_edges(&node.id).first() {
                if next.edges.len() > 0 {
                    let edges = self.inner.get_edges(&node.id)[1..].iter().map(|n| n.to_owned()).collect::<Vec<&Node>>();
                    self.iter_stack.push(RefCell::new(edges));
                }
                self.current_node = Some(next.to_owned());
            } else {
                self.current_node = None;
            }

            return current
        }

        if let Some(last_stack) = self.iter_stack.last() {
            if last_stack.borrow_mut().len() == 0 {
                self.iter_stack.pop();
                self.next();

                return current;
            }

            if let Some(node) = last_stack.borrow_mut().pop() {
                return Some(node);
            }
        }

        current
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
