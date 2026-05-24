//! Output utilities for experiment results
use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};
use psychohistory_core::experiment::ExperimentResult;

/// Print experiment results to stdout
pub fn print_experiment_results(region_name: &str, result: &ExperimentResult) {
    println!("=== Experiment Results for region: {region_name} ===");
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(["Metric", "Mean", "Stddev"]);

    table.add_row([
        "Law Quality",
        format!("{:.3}", result.mean.average_law_quality).as_str(),
        format!("{:.3}", result.stddev.average_law_quality).as_str(),
    ]);
    table.add_row([
        "Corruption Level",
        format!("{:.3}", result.mean.average_corruption_level).as_str(),
        format!("{:.3}", result.stddev.average_corruption_level).as_str(),
    ]);
    table.add_row([
        "Public Trust",
        format!("{:.3}", result.mean.average_public_trust).as_str(),
        format!("{:.3}", result.stddev.average_public_trust).as_str(),
    ]);
    table.add_row([
        "Crisis Response",
        format!("{:.3}", result.mean.average_crisis_response).as_str(),
        format!("{:.3}", result.stddev.average_crisis_response).as_str(),
    ]);
    table.add_row([
        "Adaptability",
        format!("{:.3}", result.mean.average_adaptability).as_str(),
        format!("{:.3}", result.stddev.average_adaptability).as_str(),
    ]);
    table.add_row([
        "Representation Accuracy",
        format!("{:.3}", result.mean.average_representation_accuracy).as_str(),
        format!("{:.3}", result.stddev.average_representation_accuracy).as_str(),
    ]);
    table.add_row([
        "Legislative Speed",
        format!("{:.3}", result.mean.average_legislative_speed).as_str(),
        format!("{:.3}", result.stddev.average_legislative_speed).as_str(),
    ]);
    table.add_row([
        "Economic Outcome",
        format!("{:.3}", result.mean.average_economic_outcome).as_str(),
        format!("{:.3}", result.stddev.average_economic_outcome).as_str(),
    ]);
    table.add_row([
        "Composite Score",
        format!("{:.3}", result.mean.average_composite_score).as_str(),
        format!("{:.3}", result.stddev.average_composite_score).as_str(),
    ]);

    println!("{table}");
}
