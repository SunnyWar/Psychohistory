use clap::{CommandFactory, Parser, ValueHint};

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Number of years to simulate
    #[arg(short, long, default_value_t = 10)]
    pub years: usize,

    /// Number of Monte Carlo runs per region
    #[arg(short, long, default_value_t = 10)]
    pub runs: usize,

    /// Path to scenario JSON directory
    #[arg(short, long, value_hint = ValueHint::DirPath, default_value = "scenarios")]
    pub scenario_dir: String,

    /// Log file output directory
    #[arg(long, value_hint = ValueHint::DirPath, default_value = "logs")]
    pub log_dir: String,

    /// Verbosity (repeat for more detail, e.g. -v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
