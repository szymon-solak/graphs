use crate::graph::graph_representation::GraphRepresentation;
use std::{collections::HashMap, hash::Hash};

pub struct Dfs<'a, TIndex, TNode>(
    pub &'a dyn GraphRepresentation<TIndex, TNode>,
    pub &'a TIndex,
);

impl<'a, TIndex, TNode> IntoIterator for &'a Dfs<'a, TIndex, TNode>
where
    TIndex: Eq + Hash,
{
    type Item = &'a TNode;
    type IntoIter = DfsIntoIterator<'a, TIndex, TNode>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = vec![];
        let visited = vec![];

        let iters = self
            .0
            .get_nodes()
            .iter()
            .map(|(index, _node)| (index.to_owned(), self.0.get_edges(index).into_iter()))
            .collect();

        stack.push(self.1);

        DfsIntoIterator {
            stack,
            visited,
            iters,
            graph: self.0,
        }
    }
}

pub struct DfsIntoIterator<'a, TIndex, TNode> {
    stack: Vec<&'a TIndex>,
    visited: Vec<&'a TIndex>,
    iters: HashMap<&'a TIndex, std::vec::IntoIter<&'a TIndex>>,
    graph: &'a dyn GraphRepresentation<TIndex, TNode>,
}

impl<'a, TIndex, TNode> Iterator for DfsIntoIterator<'a, TIndex, TNode>
where
    TIndex: Eq + Hash,
{
    type Item = &'a TNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&current) = self.stack.last() {
            if let Some(neighbour) = self.iters.get_mut(&current).unwrap().next() {
                if !self.visited.contains(&&neighbour) {
                    self.stack.push(&neighbour);
                }
            } else {
                self.stack.pop();
            }

            if !self.visited.contains(&&current) {
                self.visited.push(&current);
                return Some(self.graph.get_node(current).unwrap());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms;
    use crate::graph::adjacent_list_representation::AdjacentList;

    #[test]
    fn dfs_goes_through_all_connected_nodes_when_the_deepest_node_is_the_first() {
        // given
        let mut graph = AdjacentList::new();
        graph.add_edge((&1, &1), (&2, &2));
        graph.add_edge((&1, &1), (&3, &3));
        graph.add_edge((&2, &2), (&4, &4));
        graph.add_node((&5, &5));

        // when
        let visited = algorithms::Dfs(&graph, &1)
            .into_iter()
            .map(|id| id.to_owned())
            .collect::<Vec<i32>>();

        // then
        assert_eq!(visited, vec![1, 2, 4, 3]);
    }

    #[test]
    fn dfs_goes_through_all_connected_nodes_when_the_deepest_node_is_not_the_first() {
        // given
        let mut graph = AdjacentList::new();
        graph.add_edge((&1, &1), (&2, &2));
        graph.add_edge((&1, &1), (&3, &3));
        graph.add_edge((&2, &2), (&4, &4));
        graph.add_edge((&2, &2), (&5, &5));
        graph.add_edge((&3, &3), (&7, &7));
        graph.add_edge((&5, &5), (&6, &6));

        // when
        let visited = algorithms::Dfs(&graph, &1)
            .into_iter()
            .map(|id| id.to_owned())
            .collect::<Vec<i32>>();

        // then
        assert_eq!(visited, vec![1, 2, 4, 5, 6, 3, 7]);
    }
}
