//! A simple adjacency list based directed graph

use std::mem;

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeIndex(usize);

pub struct Node {
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeIndex(usize);

pub struct Edge {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node {
            first_outgoing_edge: None,
        });

        NodeIndex(index)
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> Result<(), ()> {
        if source.0 >= self.nodes.len() || target.0 >= self.nodes.len() {
          return Err(())
        }

        let edge_index = EdgeIndex(self.edges.len());
        let node = &mut self.nodes[source.0];

        self.edges.push(Edge {
            target,
            next_outgoing_edge: mem::replace(&mut node.first_outgoing_edge, Some(edge_index)),
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    pub fn add_nodes() {
        let mut graph = Graph::new();

        graph.add_node();
        graph.add_node();

        assert_eq!(2, graph.nodes.len());
        assert_eq!(None, graph.nodes[0].first_outgoing_edge);
    }

    #[test]
    pub fn add_nodes_edges() {
        let mut graph = Graph::new();

        let mut indices = vec![];
        for _ in 0..5 {
            indices.push(graph.add_node());
        }

        graph.add_edge(indices[0], indices[1]).expect("node index out of range");
        graph.add_edge(indices[0], indices[2]).expect("node index out of range");
        graph.add_edge(indices[0], indices[3]).expect("node index out of range");

        assert_eq!(3, graph.edges.len());

        let mut edge = &graph.edges[graph.nodes[0].first_outgoing_edge.expect("node 0 missing expected edge").0];
        for i in 3..0 {
          assert_eq!(i, edge.target.0);
          edge = &graph.edges[edge.next_outgoing_edge.expect("node 0 missing expected edge").0];
        }
      }
}
