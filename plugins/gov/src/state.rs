#[derive(Debug, Clone)]
pub struct GovState {
    pub stability: f64,
}

impl Default for GovState {
    fn default() -> Self {
        Self { stability: 0.5 }
    }
}
