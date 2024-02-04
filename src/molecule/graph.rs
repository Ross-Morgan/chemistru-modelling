use std::collections::HashMap;

use super::id::{EdgeID, NodeID};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnGraph<N, E> {
    pub(crate) nodes: HashMap<NodeID, N>,
    pub(crate) edges: HashMap<(NodeID, NodeID), (E, EdgeID)>,
}

impl<N, E> UnGraph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, value: N) -> NodeID {
        let id = NodeID(self.nodes.len());

        self.nodes.insert(id, value);

        id
    }

    pub fn add_edge(&mut self, from: NodeID, to: NodeID, value: E) -> EdgeID {
        let id = EdgeID(self.edges.len());

        self.edges.entry((from, to)).or_insert((value, id));

        id
    }

    pub fn node_degree(&self, node: NodeID) -> usize {
        self.edges
            .keys()
            .filter(|k| k.0 == node || k.1 == node)
            .count()
    }
}

impl<N, E: Clone> UnGraph<N, E> {
    pub fn edge_value(&self, edge: EdgeID) -> E {
        self.edges
            .values()
            .find(|v| v.1 == edge)
            .map(|e| e.0.clone())
            .unwrap()
    }
}
