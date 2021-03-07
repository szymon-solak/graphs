use std::{collections::HashMap, hash::Hash};

use super::graph_representation::GraphRepresentation;

pub struct AdjacentList<'a, TIndex, TNode> {
    nodes: HashMap<&'a TIndex, &'a TNode>,
    edges: HashMap<&'a TIndex, Vec<&'a TIndex>>,
}

impl<'a, TIndex, TNode> AdjacentList<'a, TIndex, TNode>
where
    TIndex: Eq + Hash,
{
    pub fn new() -> AdjacentList<'a, TIndex, TNode> {
        AdjacentList {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, (id, node): (&'a TIndex, &'a TNode)) -> &Self {
        self.nodes.insert(id, node);

        self
    }

    pub fn add_edge(
        &mut self,
        (l_index, l_node): (&'a TIndex, &'a TNode),
        (r_index, r_node): (&'a TIndex, &'a TNode),
    ) -> &Self {
        self.nodes.insert(l_index, l_node);
        self.nodes.insert(r_index, r_node);

        if let Some(edges) = self.edges.get_mut(l_index) {
            edges.push(r_index);
        } else {
            self.edges.insert(l_index, vec![r_index]);
        }

        self
    }
}

impl<'a, TIndex, TNode> GraphRepresentation<TIndex, TNode> for AdjacentList<'a, TIndex, TNode>
where
    TIndex: Eq + Hash,
{
    fn first_index(&self) -> Option<&TIndex> {
        self.nodes.keys().next().map(|i| i.to_owned())
    }

    fn first(&self) -> Option<&TNode> {
        self.first_index()
            .map(|id| self.get_node(id))
            .unwrap_or(None)
    }

    fn get_node(&self, node_id: &TIndex) -> Option<&TNode> {
        self.nodes.get(node_id).map(|n| n.to_owned())
    }

    fn get_nodes(&self) -> Vec<(&TIndex, &TNode)> {
        self.nodes
            .keys()
            .filter_map(|node_id| self.get_node(node_id).map(|n| (node_id.to_owned(), n)))
            .collect()
    }

    fn get_edges(&self, node_id: &TIndex) -> Vec<&TIndex> {
        if let Some(ids) = self.edges.get(node_id) {
            return ids.iter().map(|n| n.to_owned()).collect::<Vec<&TIndex>>();
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
        let node = (&1, &1);

        // when
        let mut list = AdjacentList::<i8, i8>::new();
        list.add_node(node);

        // then
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn adjacent_list_can_be_created_from_node_with_edges() {
        // given
        let node = (&1, &2);
        let node_2 = (&2, &4);
        let node_3 = (&3, &1);

        // when
        let mut list = AdjacentList::<i8, i8>::new();
        list.add_edge(node, node_2);
        list.add_edge(node, node_3);

        // then
        assert_eq!(list.len(), 2);
    }
}
