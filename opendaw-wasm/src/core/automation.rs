#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurveShape {
    Constant,
    Linear,
    Exponential(f32),
    Bezier,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutomationPoint {
    pub time: f64,
    pub value: f32,
    pub shape: CurveShape,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AutomationEnvelope {
    pub points: Vec<AutomationPoint>,
}

impl AutomationEnvelope {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Adds a point, keeping the points sorted by time.
    pub fn add_point(&mut self, point: AutomationPoint) {
        match self.points.binary_search_by(|p| p.time.partial_cmp(&point.time).unwrap_or(std::cmp::Ordering::Equal)) {
            Ok(pos) => self.points[pos] = point,
            Err(pos) => self.points.insert(pos, point),
        }
    }

    /// Interpolates the value at a given time `t`.
    pub fn interpolate(&self, t: f64) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }

        #[allow(clippy::collapsible_if)]
        if let Some(first) = self.points.first() {
            if t <= first.time {
                return first.value;
            }
        }

        #[allow(clippy::collapsible_if)]
        if let Some(last) = self.points.last() {
            if t >= last.time {
                return last.value;
            }
        }

        for window in self.points.windows(2) {
            let p0 = &window[0];
            let p1 = &window[1];

            if t >= p0.time && t < p1.time {
                let time_range = p1.time - p0.time;
                if time_range == 0.0 {
                    return p0.value;
                }

                let t_norm = ((t - p0.time) / time_range) as f32;

                return match p0.shape {
                    CurveShape::Constant => p0.value,
                    CurveShape::Linear => p0.value + (p1.value - p0.value) * t_norm,
                    CurveShape::Exponential(tension) => {
                        // Simplified placeholder for exponential calculation
                        let exp_t = if tension == 0.0 {
                            t_norm
                        } else if tension > 0.0 {
                            t_norm.powf(1.0 + tension)
                        } else {
                            t_norm.powf(1.0 / (1.0 - tension))
                        };
                        p0.value + (p1.value - p0.value) * exp_t
                    }
                    CurveShape::Bezier => {
                        // Placeholder for Bezier - defaulting to linear for now
                        p0.value + (p1.value - p0.value) * t_norm
                    }
                };
            }
        }

        0.0
    }
}
