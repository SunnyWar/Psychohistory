// core/src/time.rs
#[derive(Clone, Copy, Debug)]
pub struct SimulationTime {
    pub step: u64,
    pub granularity: TimeGranularity,
}

#[derive(Clone, Copy, Debug)]
pub enum TimeGranularity {
    Step,
    Monthly,
    Quarterly,
    Yearly,
}
