//! A simple adjacency list based directed graph

use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    mem, cmp::Reverse,
};

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
pub struct InputEdge {
    weight: usize,
    source: NodeIndex,
    target: NodeIndex,
}

#[derive(Debug)]
pub struct Successors<'graph, T> {
    graph: &'graph Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

#[derive(PartialEq, Eq)]
struct WeightedPath {
    current_cost: usize,
    total_cost: usize,
    path: Vec<NodeIndex>,
}

impl PartialOrd for WeightedPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_cost.partial_cmp(&other.total_cost)
    }
}

impl Ord for WeightedPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}

impl<T> Graph<T> {
    pub fn new_empty() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn new(nodes: Vec<T>, edges: Vec<InputEdge>) -> Result<Self, ()> {
        let mut graph = Graph::new_empty();
        for node in nodes {
            graph.add_node(node);
        }

        for edge in edges {
            match graph.add_edge(edge.weight, edge.source, edge.target) {
                Ok(_) => {}
                Err(_) => return Err(()),
            }
        }

        Ok(graph)
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

    pub fn a_star_search(
        &self,
        source: NodeIndex,
        target: NodeIndex,
        heuristic: HashMap<NodeIndex, usize>,
    ) -> Option<Vec<NodeIndex>> {
        let mut extended_list = HashSet::new();
        let mut agenda = BinaryHeap::new();
        agenda.push(Reverse(WeightedPath {
            current_cost: 0,
            total_cost: *heuristic.get(&source).expect("missing heuristic value"),
            path: vec![source],
        }));

        while let Some(weighted_path) = agenda.pop() {
            let index = weighted_path.0.path[weighted_path.0.path.len() - 1];
            if index == target {
                return Some(weighted_path.0.path);
            }

            extended_list.insert(index);

            for (node_index, cost) in self.successors(index) {
                // Only extend nodes we've not already extended
                if !extended_list.contains(&node_index) {
                    let mut new_path = weighted_path.0.path.clone();
                    let new_cost = weighted_path.0.current_cost + cost;
                    new_path.push(node_index);

                    agenda.push(Reverse(WeightedPath {
                        current_cost: new_cost,
                        total_cost: new_cost
                            + heuristic.get(&node_index).expect("missing heuristic value"),
                        path: new_path,
                    }))
                }
            }
        }

        // No path found
        None
    }

    pub fn breadth_first_search(
        &self,
        source: NodeIndex,
        target: NodeIndex,
    ) -> Option<Vec<NodeIndex>> {
        let mut extended_list = HashSet::new();
        let mut agenda = VecDeque::<Vec<NodeIndex>>::new();
        agenda.push_back(vec![source]);

        while let Some(path) = agenda.pop_front() {
            let index = path[path.len() - 1];
            if index == target {
                return Some(path);
            }

            extended_list.insert(index);

            for (node_index, _) in self.successors(index) {
                // Only extend nodes we've not already extended
                if !extended_list.contains(&node_index) {
                    let mut new_path = path.clone();
                    new_path.push(node_index);
                    agenda.push_back(new_path);
                }
            }
        }

        // No path found
        None
    }

    pub fn depth_first_search(
        &self,
        source: NodeIndex,
        target: NodeIndex,
    ) -> Option<Vec<NodeIndex>> {
        let mut extended_list = HashSet::new();
        let mut agenda = Vec::<Vec<NodeIndex>>::new();
        agenda.push(vec![source]);

        while let Some(path) = agenda.pop() {
            let index = path[path.len() - 1];
            if index == target {
                return Some(path);
            }

            extended_list.insert(index);

            for (node_index, _) in self.successors(index) {
                // Only extend nodes we've not already extended
                if !extended_list.contains(&node_index) {
                    let mut new_path = path.clone();
                    new_path.push(node_index);
                    agenda.push(new_path);
                }
            }
        }

        // No path found
        None
    }
}

impl<'graph, T> Iterator for Successors<'graph, T> {
    type Item = (NodeIndex, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            Some(index) => {
                let edge = &self.graph.edges[index.0];
                self.current_edge_index = edge.next_outgoing_edge;
                Some((edge.target, edge.weight))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Graph, InputEdge, NodeIndex};

    fn generate_test_graphs<'a>() -> Vec<(Graph<&'a str>, HashMap<NodeIndex, usize>)> {
        let mut graphs = vec![];

        let nodes = vec!["S", "A", "B", "C", "D", "E", "F", "G"];
        let edges = vec![
            InputEdge {
                weight: 10,
                source: NodeIndex(0),
                target: NodeIndex(1),
            },
            InputEdge {
                weight: 4,
                source: NodeIndex(0),
                target: NodeIndex(2),
            },
            InputEdge {
                weight: 9,
                source: NodeIndex(1),
                target: NodeIndex(3),
            },
            InputEdge {
                weight: 8,
                source: NodeIndex(2),
                target: NodeIndex(3),
            },
            InputEdge {
                weight: 7,
                source: NodeIndex(3),
                target: NodeIndex(4),
            },
            InputEdge {
                weight: 9,
                source: NodeIndex(3),
                target: NodeIndex(5),
            },
            InputEdge {
                weight: 7,
                source: NodeIndex(4),
                target: NodeIndex(5),
            },
            InputEdge {
                weight: 12,
                source: NodeIndex(4),
                target: NodeIndex(6),
            },
            InputEdge {
                weight: 8,
                source: NodeIndex(5),
                target: NodeIndex(6),
            },
            InputEdge {
                weight: 5,
                source: NodeIndex(5),
                target: NodeIndex(7),
            },
            InputEdge {
                weight: 10,
                source: NodeIndex(6),
                target: NodeIndex(7),
            },
        ];
        let heuristic = HashMap::from([
            (NodeIndex(0), 25),
            (NodeIndex(1), 20),
            (NodeIndex(2), 22),
            (NodeIndex(3), 15),
            (NodeIndex(4), 8),
            (NodeIndex(5), 3),
            (NodeIndex(6), 9),
            (NodeIndex(7), 0),
        ]);
        graphs.push((Graph::new(nodes, edges).expect("invalid test graph"), heuristic));

        graphs
    }

    #[test]
    pub fn add_nodes() {
        let mut graph = Graph::new_empty();

        graph.add_node(10);
        graph.add_node(10);

        assert_eq!(2, graph.nodes.len());
        assert_eq!(None, graph.nodes[0].first_outgoing_edge);
    }

    #[test]
    pub fn add_nodes_edges() {
        let mut graph = Graph::new_empty();

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

    #[test]
    pub fn get_successors() {
        let nodes = vec![1, 2, 3, 4, 5];
        let edges = vec![
            InputEdge {
                weight: 1,
                source: NodeIndex(0),
                target: NodeIndex(1),
            },
            InputEdge {
                weight: 1,
                source: NodeIndex(0),
                target: NodeIndex(2),
            },
            InputEdge {
                weight: 1,
                source: NodeIndex(0),
                target: NodeIndex(3),
            },
            InputEdge {
                weight: 1,
                source: NodeIndex(0),
                target: NodeIndex(4),
            },
        ];

        let graph = Graph::new(nodes, edges).expect("test graph was not valid");

        for (i, (successor, _)) in graph.successors(NodeIndex(0)).enumerate() {
            assert_eq!(4 - i, successor.0);
        }
    }

    #[test]
    pub fn depth_first_search() {
        for (graph, _) in generate_test_graphs() {
            let path = graph
                .depth_first_search(NodeIndex(0), NodeIndex(7))
                .expect("could not find path");

            // Ensure that the path does start at start and end at end
            assert_eq!(path[0].0, 0);
            assert_eq!(path[path.len() - 1].0, 7);
        }
    }

    #[test]
    pub fn a_star_first_search() {
        for (graph, heuristic) in generate_test_graphs() {
            let path = graph
                .a_star_search(NodeIndex(0), NodeIndex(7), heuristic)
                .expect("could not find path");

            // Ensure that the path does start at start and end at end
            assert_eq!(path[0].0, 0);
            assert_eq!(path[path.len() - 1].0, 7);
        }
    }

    #[test]
    pub fn breadth_first_search() {
        for (graph, _) in generate_test_graphs() {
            let path = graph
                .breadth_first_search(NodeIndex(0), NodeIndex(7))
                .expect("could not find path");

            // Ensure that the path does start at start and end at end
            assert_eq!(path[0].0, 0);
            assert_eq!(path[path.len() - 1].0, 7);
        }
    }
}
