//! Scores every case against Labelary, worst first, and writes diff images to `compat/out/`.
//!
//! `cargo run -p compat --release [--update-baselines] [name-filter...]`

use std::collections::BTreeMap;
use std::fs;

use compat::{baselines_path, cases_dir, load_baselines, load_cases, out_dir, run_case, write_baselines, CaseResult};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let update = args.iter().any(|a| a == "--update-baselines");
    let filters: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    if update && !filters.is_empty() {
        exit("--update-baselines rewrites every case; drop the name filters");
    }

    let cases = load_cases(&cases_dir()).unwrap_or_else(|e| exit(&e));
    let baselines = load_baselines(&baselines_path()).unwrap_or_else(|e| exit(&e));
    let out = out_dir();
    fs::create_dir_all(&out).unwrap_or_else(|e| exit(&e.to_string()));

    let mut results: Vec<CaseResult> = Vec::new();
    let mut missing = Vec::new();
    for case in cases.iter().filter(|c| filters.is_empty() || filters.iter().any(|f| c.name.contains(f.as_str()))) {
        match run_case(case) {
            Ok(Some(result)) => {
                for (i, (diff, ours)) in result.diffs.iter().zip(&result.renders).enumerate() {
                    let suffix = if i == 0 { String::new() } else { format!(".{}", i + 1) };
                    let save = |image: &image::RgbImage, kind: &str| {
                        let path = out.join(format!("{}.{kind}{suffix}.png", case.name));
                        image.save(&path).unwrap_or_else(|e| exit(&format!("{}: {e}", path.display())));
                    };
                    save(diff, "diff");
                    save(ours, "ours");
                }
                results.push(result);
            }
            Ok(None) => missing.push(case.name.clone()),
            Err(e) => {
                eprintln!("{}: {e}", case.name);
                missing.push(case.name.clone());
            }
        }
    }

    results.sort_by(|a, b| b.counts.tolerant().total_cmp(&a.counts.tolerant()).then(a.name.cmp(&b.name)));
    println!("{:<28} {:>7} {:>8} {:>9} {:>9}  status", "case", "labels", "raw %", "tolerant", "baseline");
    let mut worse = 0;
    for r in &results {
        let tolerant = r.counts.tolerant();
        let baseline = baselines.get(&r.name).copied();
        let status = match baseline {
            _ if r.unsupported => "unsupported by Labelary",
            None => "new",
            Some(b) if tolerant > b + compat::BASELINE_SLACK => {
                worse += 1;
                "WORSE"
            }
            Some(b) if tolerant + 0.5 < b => "better",
            Some(_) => "",
        };
        let labels = if r.our_pages == r.their_pages {
            r.our_pages.to_string()
        } else {
            format!("{}/{}", r.our_pages, r.their_pages)
        };
        println!(
            "{:<28} {:>7} {:>8.3} {:>9.2} {:>9}  {status}",
            r.name,
            labels,
            r.counts.raw(),
            tolerant,
            baseline.map(|b| format!("{b:.2}")).unwrap_or_default()
        );
    }

    let scores: Vec<f64> = results.iter().filter(|r| !r.unsupported).map(|r| r.counts.tolerant()).collect();
    if !scores.is_empty() {
        let mut sorted = scores.clone();
        sorted.sort_by(f64::total_cmp);
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let close = scores.iter().filter(|s| **s < 1.0).count();
        println!(
            "\n{} cases (without unsupported): mean tolerant {:.2}%, median {:.2}%, {} under 1%. Labels: ours/Labelary where they differ.",
            scores.len(),
            mean,
            sorted[sorted.len() / 2],
            close
        );
    }
    if !missing.is_empty() {
        println!("No reference (run compat/fetch.sh): {}", missing.join(", "));
    }
    println!("Diff images: {} (red: only Labelary, blue: only ours, pale: within tolerance)", out.display());

    if update {
        let scores: BTreeMap<String, f64> = results.iter().map(|r| (r.name.clone(), r.counts.tolerant())).collect();
        write_baselines(&baselines_path(), &scores).unwrap_or_else(|e| exit(&e));
        println!("Baselines written to {}", baselines_path().display());
    } else if worse > 0 {
        println!("{worse} case(s) worse than their baseline");
        std::process::exit(1);
    }
}

fn exit(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}
