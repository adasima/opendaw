use std::collections::HashMap;

/// Modulator source identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModSource {
    Macro(u8),
    Lfo(u8),
    Envelope(u8),
    Velocity,
    ModWheel,
    PitchBend,
    Aftertouch,
}

/// A target parameter identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModTarget {
    pub node_id: u32,
    pub param_id: u32,
}

/// Curve shapes for modulation application.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModCurve {
    Linear,
    Exponential,
    Logarithmic,
    SCurve,
}

impl ModCurve {
    /// Applies the curve shaping to the normalized input value (usually 0.0 to 1.0 or -1.0 to 1.0).
    pub fn apply(&self, value: f32) -> f32 {
        let sign = value.signum();
        let abs_val = value.abs();

        let shaped = match self {
            ModCurve::Linear => abs_val,
            ModCurve::Exponential => abs_val * abs_val,
            ModCurve::Logarithmic => abs_val.sqrt(),
            ModCurve::SCurve => abs_val * abs_val * (3.0 - 2.0 * abs_val), // smoothstep
        };

        shaped * sign
    }
}

/// A single routing entry in the modulation matrix.
#[derive(Debug, Clone)]
pub struct ModRouting {
    pub id: u32,
    pub source: ModSource,
    pub target: ModTarget,
    pub amount: f32, // typically -1.0 to 1.0
    pub curve: ModCurve,
    pub enabled: bool,
}

impl ModRouting {
    pub fn apply(&self, source_value: f32) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        let shaped = self.curve.apply(source_value);
        shaped * self.amount
    }
}

/// The Modulation Matrix orchestrates the routing of sources to targets.
#[derive(Debug, Default)]
pub struct ModMatrix {
    pub routings: Vec<ModRouting>,
    next_routing_id: u32,
}

impl ModMatrix {
    pub fn new() -> Self {
        Self {
            routings: Vec::new(),
            next_routing_id: 1,
        }
    }

    /// Adds a new modulation routing.
    pub fn add_routing(&mut self, source: ModSource, target: ModTarget, amount: f32, curve: ModCurve) -> u32 {
        let id = self.next_routing_id;
        self.next_routing_id += 1;
        self.routings.push(ModRouting {
            id,
            source,
            target,
            amount,
            curve,
            enabled: true,
        });
        id
    }

    /// Removes a routing by ID.
    pub fn remove_routing(&mut self, id: u32) {
        self.routings.retain(|r| r.id != id);
    }

    /// Enables or disables a routing.
    pub fn set_routing_enabled(&mut self, id: u32, enabled: bool) {
        if let Some(r) = self.routings.iter_mut().find(|r| r.id == id) {
            r.enabled = enabled;
        }
    }

    /// Sets the modulation amount (depth) for a routing.
    pub fn set_routing_amount(&mut self, id: u32, amount: f32) {
        if let Some(r) = self.routings.iter_mut().find(|r| r.id == id) {
            r.amount = amount;
        }
    }

    /// Calculates the total modulation for each target given the current source values.
    /// `source_values` maps `ModSource` to a normalized value (e.g., -1.0 to 1.0).
    pub fn calculate_modulations(&self, source_values: &HashMap<ModSource, f32>) -> HashMap<ModTarget, f32> {
        let mut target_mods = HashMap::new();

        for routing in &self.routings {
            if !routing.enabled {
                continue;
            }
            if let Some(&val) = source_values.get(&routing.source) {
                let mod_val = routing.apply(val);
                *target_mods.entry(routing.target.clone()).or_insert(0.0) += mod_val;
            }
        }

        target_mods
    }
}
