//! A simple adjacency list based directed graph

use std::mem;

pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeIndex(usize);

pub struct Node<T> {
    value: T,
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeIndex(usize);

pub struct Edge {
    weight: usize,
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
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

    pub fn add_edge(&mut self, weight: usize, source: NodeIndex, target: NodeIndex) -> Result<(), ()> {
        if source.0 >= self.nodes.len() || target.0 >= self.nodes.len() {
          return Err(())
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

    pub fn depth_first_search(&self, source: NodeIndex, target: NodeIndex) -> Vec<T> {
        vec![]
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

        graph.add_edge(1, indices[0], indices[1]).expect("node index out of range");
        graph.add_edge(1, indices[0], indices[2]).expect("node index out of range");
        graph.add_edge(1, indices[0], indices[3]).expect("node index out of range");

        assert_eq!(3, graph.edges.len());

        let mut edge = &graph.edges[graph.nodes[0].first_outgoing_edge.expect("node 0 missing expected edge").0];
        for i in 3..0 {
          assert_eq!(i, edge.target.0);
          edge = &graph.edges[edge.next_outgoing_edge.expect("node 0 missing expected edge").0];
        }
      }
}
