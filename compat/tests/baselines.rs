//! Fails when a case renders further from Labelary than its recorded baseline, so changes
//! can only improve compatibility. After an improvement, lock it in with
//! `cargo run -p compat --release -- --update-baselines`.

use std::sync::Mutex;

use compat::{baselines_path, cases_dir, load_baselines, load_cases, run_case, BASELINE_SLACK};

#[test]
fn no_case_gets_worse_than_its_baseline() {
    let cases = load_cases(&cases_dir()).unwrap();
    let baselines = load_baselines(&baselines_path()).unwrap();
    let failures = Mutex::new(Vec::new());
    let next = Mutex::new(cases.iter());
    // Unoptimized test builds render slowly; spread the cases over the cores.
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let Some(case) = next.lock().unwrap().next() else { break };
                let Some(result) = run_case(case).unwrap_or_else(|e| panic!("{}: {e}", case.name)) else { continue };
                let score = result.counts.tolerant();
                let failure = match baselines.get(&case.name) {
                    None => Some(format!("{}: {score:.2} has no baseline", case.name)),
                    Some(&baseline) if score > baseline + BASELINE_SLACK => {
                        Some(format!("{}: {score:.2} > baseline {baseline:.2}", case.name))
                    }
                    Some(_) => None,
                };
                failures.lock().unwrap().extend(failure);
            });
        }
    });
    let mut failures = failures.into_inner().unwrap();
    failures.sort();
    assert!(
        failures.is_empty(),
        "cases worse than baseline (see `cargo run -p compat --release`):\n{}",
        failures.join("\n")
    );
}
