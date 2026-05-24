use std::collections::HashMap;

/// NodeId represents a unique identifier for a node in the audio routing graph.
pub type NodeId = u32;

/// Audio buffer representation for PDC calculation.
pub struct AudioBuffer {
    pub data: Vec<f32>,
}

/// Represents the routing graph of audio nodes.
pub struct RoutingGraph {
    /// Latency introduced by each node in samples.
    pub node_latencies: HashMap<NodeId, usize>,
    /// Connections between nodes (from -> to).
    pub connections: Vec<(NodeId, NodeId)>,
}

impl Default for RoutingGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutingGraph {
    pub fn new() -> Self {
        Self {
            node_latencies: HashMap::new(),
            connections: Vec::new(),
        }
    }

    /// Add latency for a specific node.
    pub fn set_node_latency(&mut self, node_id: NodeId, latency_samples: usize) {
        self.node_latencies.insert(node_id, latency_samples);
    }

    /// Calculate the cumulative delay (path latency) for each node up to the master output.
    /// Returns a map of NodeId to its total delay path.
    pub fn calculate_cumulative_latencies(&self) -> HashMap<NodeId, usize> {
        let mut cumulative_latencies = HashMap::new();
        // TODO: Implement graph traversal to calculate cumulative latency for each path.
        // For now, this is a skeleton implementation.
        for (&node, &latency) in &self.node_latencies {
            cumulative_latencies.insert(node, latency);
        }
        cumulative_latencies
    }
}

/// Plugin Delay Compensation (PDC) engine.
pub struct PdcEngine {
    /// The maximum latency found in the graph.
    pub max_latency: usize,
    /// Delay compensation required for each node.
    pub compensation_delays: HashMap<NodeId, usize>,
}

impl Default for PdcEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PdcEngine {
    pub fn new() -> Self {
        Self {
            max_latency: 0,
            compensation_delays: HashMap::new(),
        }
    }

    /// Calculates required delays to compensate for the latencies in the routing graph.
    pub fn calculate_compensation(&mut self, graph: &RoutingGraph) {
        let cumulative_latencies = graph.calculate_cumulative_latencies();
        
        // Find the maximum latency in the graph
        self.max_latency = *cumulative_latencies.values().max().unwrap_or(&0);

        // Calculate compensation for each node (max_latency - node_cumulative_latency)
        self.compensation_delays.clear();
        for (node_id, latency) in cumulative_latencies {
            let compensation = self.max_latency - latency;
            self.compensation_delays.insert(node_id, compensation);
        }
    }

    /// Applies the calculated delay compensation to a given audio buffer for a specific node.
    /// This adds a silence buffer (delay) to the beginning if compensation is required.
    #[allow(clippy::collapsible_if)]
    pub fn apply_compensation(&self, node_id: NodeId, buffer: &mut AudioBuffer) {
        if let Some(&delay_samples) = self.compensation_delays.get(&node_id) {
            if delay_samples > 0 {
                // Add silence (0.0) to the front of the buffer to delay it
                let mut delayed_data = vec![0.0; delay_samples];
                delayed_data.extend_from_slice(&buffer.data);
                buffer.data = delayed_data;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdc_calculation() {
        let mut graph = RoutingGraph::new();
        graph.set_node_latency(1, 128); // Node 1 has 128 samples latency
        graph.set_node_latency(2, 0);   // Node 2 has no latency

        let mut pdc = PdcEngine::new();
        pdc.calculate_compensation(&graph);

        assert_eq!(pdc.max_latency, 128);
        assert_eq!(pdc.compensation_delays.get(&1), Some(&0)); // No compensation needed for the delayed track
        assert_eq!(pdc.compensation_delays.get(&2), Some(&128)); // Needs 128 samples compensation
    }

    #[test]
    fn test_apply_compensation() {
        let mut pdc = PdcEngine::new();
        pdc.compensation_delays.insert(1, 10);

        let mut buffer = AudioBuffer {
            data: vec![1.0, 1.0, 1.0],
        };

        pdc.apply_compensation(1, &mut buffer);

        assert_eq!(buffer.data.len(), 13);
        assert_eq!(buffer.data[0], 0.0); // Silence added
        assert_eq!(buffer.data[10], 1.0); // Original signal starts
    }
}
