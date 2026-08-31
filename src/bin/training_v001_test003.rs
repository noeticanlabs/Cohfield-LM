use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[derive(Clone)]
struct Example {
    source_id: String,
    source_type: String,
    label: Vec<u8>,
    context: String,
    target: String,
}

fn hex_decode(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i + 1 < bytes.len() {
        let hi = (bytes[i] as char).to_digit(16).expect("hex");
        let lo = (bytes[i + 1] as char).to_digit(16).expect("hex");
        out.push(((hi << 4) | lo) as u8);
        i += 2;
    }
    out
}

fn load(path: &str) -> Vec<Example> {
    fs::read_to_string(path)
        .expect("read dataset")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let p: Vec<&str> = line.split('\t').collect();
            assert_eq!(p.len(), 5, "five-column frozen TSV");
            Example {
                source_id: p[0].to_string(),
                source_type: p[1].to_string(),
                label: hex_decode(p[2]),
                context: p[3].to_string(),
                target: p[4].to_string(),
            }
        })
        .collect()
}

fn features(example: &Example, context_enabled: bool) -> Vec<String> {
    let mut f = vec![format!("source_type:{}", example.source_type)];
    if !context_enabled {
        return f;
    }
    f.push(format!("context:{}", example.context));
    f.push(format!(
        "source_type_context:{}|{}",
        example.source_type, example.context
    ));
    let mut seen = HashSet::new();
    for window in example.label.windows(2) {
        let key = format!(
            "byte_pair_context:{:02x}{:02x}|{}",
            window[0].to_ascii_lowercase(),
            window[1].to_ascii_lowercase(),
            example.context
        );
        if seen.insert(key.clone()) {
            f.push(key);
        }
    }
    f
}

struct RelationalModel {
    feature_target: HashMap<String, HashMap<String, u64>>,
    target_prior: HashMap<String, u64>,
    target_vocabulary: HashSet<String>,
}

impl RelationalModel {
    fn train(examples: &[Example]) -> Self {
        let mut model = Self {
            feature_target: HashMap::new(),
            target_prior: HashMap::new(),
            target_vocabulary: HashSet::new(),
        };
        for example in examples {
            *model.target_prior.entry(example.target.clone()).or_insert(0) += 1;
            model.target_vocabulary.insert(example.target.clone());
            for feature in features(example, true) {
                *model
                    .feature_target
                    .entry(feature)
                    .or_default()
                    .entry(example.target.clone())
                    .or_insert(0) += 1;
            }
        }
        model
    }

    fn predict(
        &self,
        example: &Example,
        context_enabled: bool,
    ) -> (Option<String>, HashMap<String, f64>) {
        let mut score = HashMap::<String, f64>::new();
        for feature in features(example, context_enabled) {
            let Some(counts) = self.feature_target.get(&feature) else {
                continue;
            };
            let total: u64 = counts.values().sum();
            let denominator = ((total + 1) as f64).ln_1p();
            for (target, n) in counts {
                *score.entry(target.clone()).or_insert(0.0) +=
                    (*n as f64).ln_1p() / denominator;
            }
        }
        let best = score.iter().max_by(|(ta, sa), (tb, sb)| {
            sa.partial_cmp(sb)
                .unwrap()
                .then_with(|| {
                    self.target_prior
                        .get(*ta)
                        .unwrap_or(&0)
                        .cmp(self.target_prior.get(*tb).unwrap_or(&0))
                })
                .then_with(|| ta.cmp(tb))
        });
        (best.map(|(t, _)| t.clone()), score)
    }
}

#[derive(Default)]
struct Metrics {
    n: usize,
    correct: usize,
    rank_sum: f64,
}

fn evaluate(
    model: &RelationalModel,
    examples: &[Example],
    variant: &str,
    contexts: &[String],
) -> Metrics {
    let supported: Vec<&Example> = examples
        .iter()
        .filter(|e| model.target_vocabulary.contains(&e.target))
        .collect();
    let mut metrics = Metrics::default();
    for (i, original) in supported.iter().enumerate() {
        let mut example = (*original).clone();
        let mut context_enabled = true;
        match variant {
            "true" => {}
            "context_ablation" => context_enabled = false,
            "wrong_context" => {
                let index = contexts
                    .iter()
                    .position(|c| c == &example.context)
                    .unwrap_or(0);
                example.context = contexts[(index + 1) % contexts.len()].clone();
            }
            "shuffled_context" => {
                example.context = supported[(i + 137) % supported.len()].context.clone();
            }
            _ => panic!("unknown variant"),
        }
        let (prediction, scores) = model.predict(&example, context_enabled);
        metrics.n += 1;
        if prediction.as_deref() == Some(original.target.as_str()) {
            metrics.correct += 1;
        }
        let true_score = *scores.get(&original.target).unwrap_or(&0.0);
        metrics.rank_sum +=
            (1 + scores.values().filter(|value| **value > true_score).count()) as f64;
    }
    metrics
}

fn majority_accuracy(
    training: &[Example],
    holdout: &[Example],
    mode: &str,
    targets: &HashSet<String>,
) -> f64 {
    let mut table = HashMap::<String, HashMap<String, u64>>::new();
    let mut prior = HashMap::<String, u64>::new();
    for example in training {
        *prior.entry(example.target.clone()).or_insert(0) += 1;
        let key = match mode {
            "context" => example.context.clone(),
            "source_type" => example.source_type.clone(),
            _ => format!("{}|{}", example.source_type, example.context),
        };
        *table
            .entry(key)
            .or_default()
            .entry(example.target.clone())
            .or_insert(0) += 1;
    }
    let global = prior.iter().max_by_key(|(_, n)| *n).unwrap().0.clone();
    let supported: Vec<&Example> = holdout
        .iter()
        .filter(|e| targets.contains(&e.target))
        .collect();
    let mut correct = 0;
    for example in &supported {
        let key = match mode {
            "context" => example.context.clone(),
            "source_type" => example.source_type.clone(),
            _ => format!("{}|{}", example.source_type, example.context),
        };
        let prediction = table
            .get(&key)
            .and_then(|counts| counts.iter().max_by_key(|(_, n)| *n).map(|(t, _)| t.clone()))
            .unwrap_or_else(|| global.clone());
        if prediction == example.target {
            correct += 1;
        }
    }
    correct as f64 / supported.len() as f64
}

fn paired_accuracy(model: &RelationalModel, examples: &[Example]) -> (usize, usize) {
    let mut by_source = HashMap::<String, Vec<&Example>>::new();
    for example in examples {
        by_source
            .entry(example.source_id.clone())
            .or_default()
            .push(example);
    }
    let mut eligible = 0;
    let mut both_correct = 0;
    for group in by_source.values() {
        if group.len() != 2
            || !group
                .iter()
                .all(|e| model.target_vocabulary.contains(&e.target))
        {
            continue;
        }
        eligible += 1;
        if group.iter().all(|e| {
            model.predict(e, true).0.as_deref() == Some(e.target.as_str())
        }) {
            both_correct += 1;
        }
    }
    (eligible, both_correct)
}

fn print_metrics(name: &str, metrics: &Metrics) {
    println!(
        "\"{}\":{{\"n\":{},\"accuracy\":{:.12},\"mean_rank\":{:.12}}}",
        name,
        metrics.n,
        metrics.correct as f64 / metrics.n as f64,
        metrics.rank_sum / metrics.n as f64
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: training_v001_test003 <train.tsv> <validation.tsv> <test.tsv>");
        std::process::exit(2);
    }
    let training = load(&args[1]);
    let validation = load(&args[2]);
    let test = load(&args[3]);
    let model = RelationalModel::train(&training);
    let mut contexts: Vec<String> = training
        .iter()
        .map(|e| e.context.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    contexts.sort();

    println!("{{");
    println!("\"experiment\":\"Cohfield-LM Training v0.01 Test 003\",");
    println!("\"train_examples\":{},", training.len());
    println!("\"learned_features\":{},", model.feature_target.len());
    println!("\"target_vocabulary\":{},", model.target_vocabulary.len());
    for (split_name, holdout) in [("validation", &validation), ("test", &test)] {
        println!("\"{}\":{{", split_name);
        let variants = ["true", "context_ablation", "wrong_context", "shuffled_context"];
        for (index, variant) in variants.iter().enumerate() {
            let metrics = evaluate(&model, holdout, variant, &contexts);
            print_metrics(variant, &metrics);
            if index + 1 < variants.len() {
                println!(",");
            }
        }
        let (eligible, both_correct) = paired_accuracy(&model, holdout);
        println!(",");
        println!("\"paired_supported_sources\":{},", eligible);
        println!("\"paired_both_correct\":{},", both_correct);
        println!(
            "\"paired_accuracy\":{:.12},",
            both_correct as f64 / eligible as f64
        );
        println!(
            "\"baseline_context_majority\":{:.12},",
            majority_accuracy(&training, holdout, "context", &model.target_vocabulary)
        );
        println!(
            "\"baseline_source_type_majority\":{:.12},",
            majority_accuracy(&training, holdout, "source_type", &model.target_vocabulary)
        );
        println!(
            "\"baseline_source_type_context_majority\":{:.12}",
            majority_accuracy(&training, holdout, "both", &model.target_vocabulary)
        );
        println!("}}");
        if split_name == "validation" {
            println!(",");
        }
    }
    println!("}}");
}
