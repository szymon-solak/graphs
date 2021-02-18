use crate::graph::graph_representation::GraphRepresentation;
use crate::graph::node::Node;

pub fn dfs(graph: &dyn GraphRepresentation, on_visit: &mut dyn FnMut(&Node)) {
    let mut visited = Vec::<String>::new();

    if let Some(node) = graph.first() {
        dfs_visit(&node, graph, &mut visited, on_visit)
    }
}

fn dfs_visit(
    node: &Node,
    graph: &dyn GraphRepresentation,
    visited: &mut Vec<String>,
    on_visit: &mut dyn FnMut(&Node),
) {
    if visited.iter().any(|id| id == &node.id) {
        return;
    }

    visited.push(node.id.to_owned());
    on_visit(node);

    graph
        .get_edges(node.id.to_owned())
        .iter()
        .for_each(|edge| dfs_visit(edge, graph, visited, on_visit));
}

#[cfg(test)]
mod tests {
    use crate::algorithms;
    use crate::graph::{adjacent_list_representation::AdjacentList, node::Node};

    #[test]
    fn dfs_goes_through_all_connected_nodes() {
        // given
        let graph = AdjacentList::new(vec![
            Node::new("1".into(), vec!["2".into(), "3".into()]),
            Node::new("2".into(), vec!["4".into()]),
            Node::new("5".into(), vec![]),
        ]);
        let mut visited: Vec<String> = vec![];

        let mut on_visit = |n: &Node| {
            visited.push(n.id.to_owned());
        };

        // when
        algorithms::dfs(&graph, &mut on_visit);

        // then
        assert_eq!(visited, vec!["1", "2", "4", "3"]);
    }
}
