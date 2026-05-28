use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Number of time steps (ticks)
    #[arg(short = 'y', long = "ticks", default_value_t = 10)]
    pub ticks: usize,

    /// Number of Monte Carlo runs
    #[arg(short = 'r', long = "runs", default_value_t = 5)]
    pub runs: usize,

    /// Time step increment (delta t)
    #[arg(short = 'd', long = "delta-t", default_value_t = 1.0)]
    pub delta_t: f64,
}
