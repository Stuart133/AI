//! A simple adjacency list based directed graph

use std::{mem, collections::HashMap};

#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIndex(usize);

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeIndex(usize);

#[derive(Debug)]
pub struct Edge {
    weight: usize,
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug)]
pub struct Successors<'graph, T> {
    graph: &'graph Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, value: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node::<T> {
            value,
            first_outgoing_edge: None,
        });

        NodeIndex(index)
    }

    pub fn add_edge(
        &mut self,
        weight: usize,
        source: NodeIndex,
        target: NodeIndex,
    ) -> Result<(), ()> {
        if source.0 >= self.nodes.len() || target.0 >= self.nodes.len() {
            return Err(());
        }

        let edge_index = EdgeIndex(self.edges.len());
        let node = &mut self.nodes[source.0];

        self.edges.push(Edge {
            weight,
            target,
            next_outgoing_edge: mem::replace(&mut node.first_outgoing_edge, Some(edge_index)),
        });

        Ok(())
    }

    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        Successors {
            graph: self,
            current_edge_index: self.nodes[source.0].first_outgoing_edge,
        }
    }

    pub fn depth_first_search(&self, source: NodeIndex, target: NodeIndex) -> Option<Vec<NodeIndex>> {
        let mut agenda = Vec::<Vec::<NodeIndex>>::new();
        agenda.push(vec![source]);

        while let Some(path) = agenda.pop() {
            let index = path[path.len() - 1];
            if index == target {
                return Some(path);
            }

            for node_index in self.successors(index) {
                if !path.contains(&node_index) {
                    let mut new_path = path.clone();
                    new_path.push(node_index);
                    agenda.push(new_path);
                }
            }
        }

        // No path found, return empty path for now
        None
    }
}

impl<'graph, T> Iterator for Successors<'graph, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            Some(index) => {
                let edge = &self.graph.edges[index.0];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    pub fn add_nodes() {
        let mut graph = Graph::new();

        graph.add_node(10);
        graph.add_node(10);

        assert_eq!(2, graph.nodes.len());
        assert_eq!(None, graph.nodes[0].first_outgoing_edge);
    }

    #[test]
    pub fn add_nodes_edges() {
        let mut graph = Graph::new();

        let mut indices = vec![];
        for _ in 0..5 {
            indices.push(graph.add_node(1));
        }

        graph
            .add_edge(1, indices[0], indices[1])
            .expect("node index out of range");
        graph
            .add_edge(1, indices[0], indices[2])
            .expect("node index out of range");
        graph
            .add_edge(1, indices[0], indices[3])
            .expect("node index out of range");

        assert_eq!(3, graph.edges.len());

        let mut edge = &graph.edges[graph.nodes[0]
            .first_outgoing_edge
            .expect("node 0 missing expected edge")
            .0];
        for i in 3..0 {
            assert_eq!(i, edge.target.0);
            edge = &graph.edges[edge
                .next_outgoing_edge
                .expect("node 0 missing expected edge")
                .0];
        }
    }
}
