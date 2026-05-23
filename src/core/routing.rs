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
