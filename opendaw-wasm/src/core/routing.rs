use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Track,
    SendBus,
    Master,
    Sidechain,
}

#[derive(Debug, Clone)]
pub struct AudioNode {
    pub id: NodeId,
    pub name: String,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Default)]
pub struct RoutingGraph {
    pub nodes: HashMap<NodeId, AudioNode>,
    pub edges: HashMap<NodeId, HashSet<NodeId>>,
    next_id: usize,
}

impl RoutingGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_node(&mut self, name: String, node_type: NodeType) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;

        let node = AudioNode {
            id,
            name,
            node_type,
        };

        self.nodes.insert(id, node);
        self.edges.insert(id, HashSet::new());
        id
    }

    pub fn remove_node(&mut self, id: NodeId) -> Option<AudioNode> {
        if let Some(node) = self.nodes.remove(&id) {
            self.edges.remove(&id);
            for connections in self.edges.values_mut() {
                connections.remove(&id);
            }
            Some(node)
        } else {
            None
        }
    }

    pub fn connect(&mut self, source: NodeId, destination: NodeId) -> Result<(), &'static str> {
        if !self.nodes.contains_key(&source) {
            return Err("Source node does not exist");
        }
        if !self.nodes.contains_key(&destination) {
            return Err("Destination node does not exist");
        }
        if source == destination {
            return Err("Cannot connect a node to itself");
        }

        if let Some(destinations) = self.edges.get_mut(&source) {
            if destinations.contains(&destination) {
                return Err("Edge already exists");
            }
            destinations.insert(destination);
        }

        if self.has_cycle() {
            if let Some(destinations) = self.edges.get_mut(&source) {
                destinations.remove(&destination);
            }
            return Err("Adding this connection would create a cycle");
        }

        Ok(())
    }

    pub fn disconnect(&mut self, source: NodeId, destination: NodeId) -> Result<(), &'static str> {
        if let Some(destinations) = self.edges.get_mut(&source) {
            if destinations.remove(&destination) {
                Ok(())
            } else {
                Err("Connection does not exist")
            }
        } else {
            Err("Source node does not exist")
        }
    }

    fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();

        for &node_id in self.nodes.keys() {
            if self.cycle_dfs(node_id, &mut visited, &mut recursion_stack) {
                return true;
            }
        }
        false
    }

    fn cycle_dfs(
        &self,
        node: NodeId,
        visited: &mut HashSet<NodeId>,
        recursion_stack: &mut HashSet<NodeId>,
    ) -> bool {
        if recursion_stack.contains(&node) {
            return true;
        }
        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);
        recursion_stack.insert(node);

        if let Some(destinations) = self.edges.get(&node) {
            for &dest_id in destinations {
                if self.cycle_dfs(dest_id, visited, recursion_stack) {
                    return true;
                }
            }
        }

        recursion_stack.remove(&node);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cycle_empty_graph() {
        let graph = RoutingGraph::new();
        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_disconnected_nodes() {
        let mut graph = RoutingGraph::new();
        graph.add_node("Track 1".to_string(), NodeType::Track);
        graph.add_node("Track 2".to_string(), NodeType::Track);
        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_linear_connections() {
        let mut graph = RoutingGraph::new();
        let node1 = graph.add_node("Track 1".to_string(), NodeType::Track);
        let node2 = graph.add_node("Track 2".to_string(), NodeType::Track);
        let node3 = graph.add_node("Master".to_string(), NodeType::Master);

        graph.connect(node1, node2).unwrap();
        graph.connect(node2, node3).unwrap();

        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_self_referential() {
        let mut graph = RoutingGraph::new();
        let node1 = graph.add_node("Track 1".to_string(), NodeType::Track);

        // We have to bypass `connect` as it checks for cycles and prevents adding them.
        graph.edges.get_mut(&node1).unwrap().insert(node1);

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_simple_cycle() {
        let mut graph = RoutingGraph::new();
        let node1 = graph.add_node("Track 1".to_string(), NodeType::Track);
        let node2 = graph.add_node("Track 2".to_string(), NodeType::Track);

        graph.connect(node1, node2).unwrap();

        // Bypass `connect` to force the cycle
        graph.edges.get_mut(&node2).unwrap().insert(node1);

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_complex_cycle() {
        let mut graph = RoutingGraph::new();
        let node1 = graph.add_node("Track 1".to_string(), NodeType::Track);
        let node2 = graph.add_node("Track 2".to_string(), NodeType::Track);
        let node3 = graph.add_node("Track 3".to_string(), NodeType::Track);
        let node4 = graph.add_node("Track 4".to_string(), NodeType::Track);

        graph.connect(node1, node2).unwrap();
        graph.connect(node2, node3).unwrap();
        graph.connect(node3, node4).unwrap();

        // Bypass `connect` to force the cycle A -> B -> C -> D -> B
        graph.edges.get_mut(&node4).unwrap().insert(node2);

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_multiple_components() {
        let mut graph = RoutingGraph::new();

        // Component 1: Linear
        let node1 = graph.add_node("Track 1".to_string(), NodeType::Track);
        let node2 = graph.add_node("Track 2".to_string(), NodeType::Track);
        graph.connect(node1, node2).unwrap();

        // Component 2: Cycle
        let node3 = graph.add_node("Track 3".to_string(), NodeType::Track);
        let node4 = graph.add_node("Track 4".to_string(), NodeType::Track);
        graph.connect(node3, node4).unwrap();
        // Bypass `connect` to force the cycle
        graph.edges.get_mut(&node4).unwrap().insert(node3);

        assert!(graph.has_cycle());
    }
}
