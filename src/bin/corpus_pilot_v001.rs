use cohfield_lm::corpus_bridge_v001::parse_pack;
use cohfield_lm::corpus_pilot_v001::run_pilot;
use std::env;
use std::fs;
use std::process;

fn usage() -> ! {
    eprintln!("usage: corpus_pilot_v001 <train.cflm> <holdout.cflm> [epochs]");
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        usage();
    }
    let epochs: usize = if args.len() == 4 {
        args[3].parse().unwrap_or_else(|_| usage())
    } else {
        1
    };
    if epochs == 0 {
        usage();
    }

    let train_bytes = fs::read(&args[1]).unwrap_or_else(|error| {
        eprintln!("failed to read train pack: {error}");
        process::exit(1);
    });
    let holdout_bytes = fs::read(&args[2]).unwrap_or_else(|error| {
        eprintln!("failed to read holdout pack: {error}");
        process::exit(1);
    });
    let train = parse_pack(&train_bytes).unwrap_or_else(|error| {
        eprintln!("failed to parse train pack: {error}");
        process::exit(1);
    });
    let holdout = parse_pack(&holdout_bytes).unwrap_or_else(|error| {
        eprintln!("failed to parse holdout pack: {error}");
        process::exit(1);
    });
    if train.is_empty() || holdout.is_empty() {
        eprintln!("train and holdout packs must both contain records");
        process::exit(1);
    }

    let report = run_pilot(&train, &holdout, epochs);
    println!("{{");
    println!("  \"pilot\": \"CF-LM Corpus Pilot v0.01\",");
    println!("  \"epochs\": {epochs},");
    println!("  \"train_records\": {},", train.len());
    println!("  \"holdout_records\": {},", holdout.len());
    println!(
        "  \"true_mean_correct_activation\": {:.17},",
        report.true_trained.mean_correct_activation
    );
    println!("  \"true_mean_rank\": {:.17},", report.true_trained.mean_rank);
    println!(
        "  \"true_top1_or_tied_rate\": {:.17},",
        report.true_trained.top1_or_tied_rate
    );
    println!(
        "  \"shuffled_target_mean_correct_activation\": {:.17},",
        report.shuffled_target_trained.mean_correct_activation
    );
    println!(
        "  \"untrained_mean_correct_activation\": {:.17},",
        report.untrained.mean_correct_activation
    );
    println!(
        "  \"rotated_prompt_mean_correct_activation\": {:.17},",
        report.true_trained.mean_rotated_prompt_correct_activation
    );
    println!(
        "  \"boundary_only_mean_correct_activation\": {:.17},",
        report.true_trained.mean_boundary_only_correct_activation
    );
    println!(
        "  \"pairing_activation_delta\": {:.17},",
        report.pairing_activation_delta
    );
    println!(
        "  \"prompt_pair_activation_delta\": {:.17},",
        report.prompt_pair_activation_delta
    );
    println!(
        "  \"boundary_activation_delta\": {:.17},",
        report.boundary_activation_delta
    );
    println!(
        "  \"mean_field_l1_vs_rotated_prompt\": {:.17},",
        report.true_trained.mean_field_l1_vs_rotated_prompt
    );
    println!(
        "  \"mean_field_l1_vs_boundary_only\": {:.17}",
        report.true_trained.mean_field_l1_vs_boundary_only
    );
    println!("}}");
}
