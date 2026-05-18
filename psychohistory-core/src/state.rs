pub struct SimulationState {
    pub econ: Option<Box<dyn std::any::Any>>,
    pub gov: Option<Box<dyn std::any::Any>>,
    pub demog: Option<Box<dyn std::any::Any>>,
}

impl SimulationState {
    pub fn new() -> Self {
        Self { econ: None, gov: None, demog: None }
    }
}
