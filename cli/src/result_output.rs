//! Output utilities for experiment results
use psychohistory_core::experiment::ExperimentResult;

/// Print experiment results to stdout
pub fn print_experiment_results(region_name: &str, result: &ExperimentResult) {
    println!("=== Experiment Results for region: {} ===", region_name);
    println!("    Metric                        Mean      Stddev");
    println!("-----------------------------------------------------");
    println!(
        "  Law Quality               {:8.3}  {:8.3}",
        result.mean.average_law_quality, result.stddev.average_law_quality
    );
    println!(
        "  Corruption Level          {:8.3}  {:8.3}",
        result.mean.average_corruption_level, result.stddev.average_corruption_level
    );
    println!(
        "  Public Trust              {:8.3}  {:8.3}",
        result.mean.average_public_trust, result.stddev.average_public_trust
    );
    println!(
        "  Crisis Response           {:8.3}  {:8.3}",
        result.mean.average_crisis_response, result.stddev.average_crisis_response
    );
    println!(
        "  Adaptability              {:8.3}  {:8.3}",
        result.mean.average_adaptability, result.stddev.average_adaptability
    );
    println!(
        "  Representation Accuracy   {:8.3}  {:8.3}",
        result.mean.average_representation_accuracy, result.stddev.average_representation_accuracy
    );
    println!(
        "  Legislative Speed         {:8.3}  {:8.3}",
        result.mean.average_legislative_speed, result.stddev.average_legislative_speed
    );
    println!(
        "  Economic Outcome          {:8.3}  {:8.3}",
        result.mean.average_economic_outcome, result.stddev.average_economic_outcome
    );
    println!(
        "  Composite Score           {:8.3}  {:8.3}",
        result.mean.average_composite_score, result.stddev.average_composite_score
    );
}
