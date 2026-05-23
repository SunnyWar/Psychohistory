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
    row!(
        "Law Quality",
        result.mean.average_law_quality,
        result.stddev.average_law_quality
    );
    row!(
        "Corruption Level",
        result.mean.average_corruption_level,
        result.stddev.average_corruption_level
    );
    row!(
        "Public Trust",
        result.mean.average_public_trust,
        result.stddev.average_public_trust
    );
    row!(
        "Crisis Response",
        result.mean.average_crisis_response,
        result.stddev.average_crisis_response
    );
    row!(
        "Adaptability",
        result.mean.average_adaptability,
        result.stddev.average_adaptability
    );
    row!(
        "Representation Accuracy",
        result.mean.average_representation_accuracy,
        result.stddev.average_representation_accuracy
    );
    row!(
        "Legislative Speed",
        result.mean.average_legislative_speed,
        result.stddev.average_legislative_speed
    );
    row!(
        "Economic Outcome",
        result.mean.average_economic_outcome,
        result.stddev.average_economic_outcome
    );
    row!(
        "Composite Score",
        result.mean.average_composite_score,
        result.stddev.average_composite_score
    );
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
        writeln!(
            file,
            "{},{},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5}",
            region,
            i + 1,
            run.average_law_quality,
            run.average_corruption_level,
            run.average_public_trust,
            run.average_crisis_response,
            run.average_adaptability,
            run.average_representation_accuracy,
            run.average_legislative_speed,
            run.average_economic_outcome,
            run.average_composite_score
        )?;
    }
    Ok(())
}
