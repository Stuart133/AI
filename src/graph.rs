//! A simple adjacency list based directed graph

use std::mem;

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

pub struct NodeIndex(usize);

pub struct Node {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub struct EdgeIndex(usize);

pub struct Edge {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl Graph {
    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node {
            first_outgoing_edge: None,
        });

        NodeIndex(index)
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = EdgeIndex(self.edges.len());
        let node = &mut self.nodes[source.0];

        self.edges.push(Edge {
            target,
            next_outgoing_edge: mem::replace(&mut node.first_outgoing_edge, Some(edge_index)),
        });
    }
}
