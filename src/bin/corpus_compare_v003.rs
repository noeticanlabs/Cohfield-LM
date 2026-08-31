use cohfield_lm::corpus_bridge_v001::{parse_pack, CorpusRecord};
use cohfield_lm::corpus_pilot_v002::{HistoryModel, HistoryState};
use cohfield_lm::corpus_pilot_v003::{TraceModel, TraceState};
use std::env;
use std::fs;
use std::process;

#[derive(Clone, Copy)]
struct Metrics {
    activation: f64,
    rank: f64,
    top1: f64,
    rotated_activation: f64,
    boundary_activation: f64,
}

fn rank(field: &[f64], target: u8) -> usize {
    let value = field[target as usize];
    1 + field.iter().filter(|&&candidate| candidate > value).count()
}

fn boundary_only(input: &[u8]) -> &[u8] {
    const WIDTH: usize = b"\n\nAssistant: ".len();
    let start = input.len().saturating_sub(WIDTH);
    &input[start..]
}

fn evaluate<F>(records: &[CorpusRecord], continuation: F) -> Metrics
where
    F: Fn(&[u8]) -> Vec<f64>,
{
    let eligible: Vec<&CorpusRecord> = records.iter().filter(|r| !r.target.is_empty()).collect();
    let n = eligible.len();
    let mut activation = 0.0;
    let mut ranks = 0.0;
    let mut top1 = 0.0;
    let mut rotated = 0.0;
    let mut boundary = 0.0;
    for (i, record) in eligible.iter().enumerate() {
        let target = record.target[0];
        let field = continuation(&record.input);
        let rotated_field = continuation(&eligible[(i + 1) % n].input);
        let boundary_field = continuation(boundary_only(&record.input));
        let value = field[target as usize];
        let maximum = field.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        activation += value;
        ranks += rank(&field, target) as f64;
        top1 += if value >= maximum { 1.0 } else { 0.0 };
        rotated += rotated_field[target as usize];
        boundary += boundary_field[target as usize];
    }
    let d = n as f64;
    Metrics {
        activation: activation / d,
        rank: ranks / d,
        top1: top1 / d,
        rotated_activation: rotated / d,
        boundary_activation: boundary / d,
    }
}

fn rotate_targets(records: &[CorpusRecord]) -> Vec<CorpusRecord> {
    if records.len() < 2 {
        return records.to_vec();
    }
    records
        .iter()
        .enumerate()
        .map(|(i, r)| CorpusRecord {
            input: r.input.clone(),
            target: records[(i + 1) % records.len()].target.clone(),
        })
        .collect()
}

fn print_metrics(label: &str, m: Metrics, shuffled: Metrics, ablated: Metrics) {
    println!("  \"{label}\": {{");
    println!("    \"mean_correct_activation\": {:.17},", m.activation);
    println!("    \"mean_rank\": {:.17},", m.rank);
    println!("    \"top1_or_tied_rate\": {:.17},", m.top1);
    println!("    \"pairing_activation_delta\": {:.17},", m.activation - shuffled.activation);
    println!("    \"prompt_pair_activation_delta\": {:.17},", m.activation - m.rotated_activation);
    println!("    \"boundary_activation_delta\": {:.17},", m.activation - m.boundary_activation);
    println!("    \"history_ablation_delta\": {:.17}", m.activation - ablated.activation);
    println!("  }}");
}

fn usage() -> ! {
    eprintln!("usage: corpus_compare_v003 <train.cflm> <holdout.cflm> [epochs]");
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        usage();
    }
    let epochs: usize = if args.len() == 4 { args[3].parse().unwrap_or_else(|_| usage()) } else { 1 };
    if epochs == 0 { usage(); }
    let train = parse_pack(&fs::read(&args[1]).unwrap()).unwrap();
    let holdout = parse_pack(&fs::read(&args[2]).unwrap()).unwrap();
    let shuffled = rotate_targets(&train);

    let v2 = HistoryModel::default();
    let v2_state: HistoryState = v2.train(&train, epochs);
    let v2_shuffled = v2.train(&shuffled, epochs);
    let v2_true = evaluate(&holdout, |input| v2.continuation_field(&v2_state, input, true));
    let v2_control = evaluate(&holdout, |input| v2.continuation_field(&v2_shuffled, input, true));
    let v2_ablation = evaluate(&holdout, |input| v2.continuation_field(&v2_state, input, false));

    let v3 = TraceModel::default();
    let v3_state: TraceState = v3.train(&train, epochs);
    let v3_shuffled = v3.train(&shuffled, epochs);
    let v3_true = evaluate(&holdout, |input| v3.continuation_field(&v3_state, input, true));
    let v3_control = evaluate(&holdout, |input| v3.continuation_field(&v3_shuffled, input, true));
    let v3_ablation = evaluate(&holdout, |input| v3.continuation_field(&v3_state, input, false));

    println!("{{");
    println!("  \"experiment\": \"CF-LM Corpus Pilots v0.02-v0.03 matched comparison\",");
    println!("  \"epochs\": {epochs},");
    println!("  \"train_records\": {},", train.len());
    println!("  \"holdout_records\": {},", holdout.len());
    println!("  \"v002_learned_pairs\": {},", HistoryModel::learned_pairs(&v2_state));
    println!("  \"v002_learned_paths\": {},", HistoryModel::learned_paths(&v2_state));
    println!("  \"v003_learned_pairs\": {},", TraceModel::learned_pairs(&v3_state));
    println!("  \"v003_learned_trace_relations\": {},", TraceModel::learned_trace_relations(&v3_state));
    print_metrics("v002", v2_true, v2_control, v2_ablation);
    println!(",");
    print_metrics("v003", v3_true, v3_control, v3_ablation);
    println!("}}");
}
