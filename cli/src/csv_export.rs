//! CSV export utilities for experiment results
use psychohistory_core::experiment::ExperimentResult;
use psychohistory_core::run_result::RunResult;
use std::fs::File;
use std::io::{Result, Write};

/// Write summary statistics (mean/stddev) to a CSV file
pub fn write_summary_csv(path: &str, region: &str, result: &ExperimentResult) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "Region,Metric,Mean,Stddev")?;
    macro_rules! row {
        ($metric:expr, $mean:expr, $stddev:expr) => {
            writeln!(file, "{},{},{:.5},{:.5}", region, $metric, $mean, $stddev)?;
        };
    }
    let metrics = [
        "Law Quality",
        "Corruption Level",
        "Public Trust",
        "Crisis Response",
        "Adaptability",
        "Representation Accuracy",
        "Legislative Speed",
        "Economic Outcome",
        "Composite Score",
    ];
    for (i, &metric) in metrics.iter().enumerate() {
        let mean = result.mean.averages.get(i).copied().unwrap_or(0.0);
        let stddev = result.stddev.averages.get(i).copied().unwrap_or(0.0);
        row!(metric, mean, stddev);
    }
    Ok(())
}

/// Write per-run results to a CSV file
pub fn write_per_run_csv(path: &str, region: &str, runs: &[RunResult]) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(
        file,
        "Region,Run,Law Quality,Corruption Level,Public Trust,Crisis Response,Adaptability,Representation Accuracy,Legislative Speed,Economic Outcome,Composite Score"
    )?;
    for (i, run) in runs.iter().enumerate() {
        let mut row = format!("{},{}", region, i + 1);
        for val in run.averages.iter().take(9) {
            row.push_str(&format!(",{:.5}", val));
        }
        writeln!(file, "{}", row)?;
    }
    Ok(())
}
