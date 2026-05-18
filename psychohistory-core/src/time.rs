#[derive(Clone, Copy, Debug)]
pub struct SimulationTime {
    pub step: u64,
    pub granularity: TimeGranularity,
}

#[derive(Clone, Copy, Debug)]
pub enum TimeGranularity {
    Monthly,
    Quarterly,
    Yearly,
}
