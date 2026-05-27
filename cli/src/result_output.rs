//! Output utilities for experiment results
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use psychohistory_core::experiment::ExperimentResult;

/// Print experiment results to stdout
pub fn print_experiment_results(region_name: &str, result: &ExperimentResult) {
    println!("=== Experiment Results for region: {region_name} ===");
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(["Metric", "Mean", "Stddev"]);

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
        table.add_row([
            metric,
            format!("{:.3}", mean).as_str(),
            format!("{:.3}", stddev).as_str(),
        ]);
    }

    println!("{table}");
}
