//! Renders each sample label a couple of thousand times and reports latency statistics.
//!
//! Run with `cargo bench -p pdf-renderer`. Set `BENCH_ITERS` to change the number of
//! measured renders per label (default 2000).

use std::hint::black_box;
use std::time::{Duration, Instant};

use pdf_renderer::PdfRenderer;
use zpl_parser::Parser;
use zpl_state_manager::{DrawInstruction, StateManager};

const LABELS: &[(&str, &str)] = &[
    ("jelle", include_str!("../../jelle.zpl")),
    ("labelary", include_str!("../../labelary.zpl")),
    ("test", include_str!("../../test.zpl")),
];

const WARMUP: usize = 50;

fn main() {
    let iters: usize = std::env::var("BENCH_ITERS").ok().and_then(|v| v.parse().ok()).unwrap_or(2000);

    println!("{iters} renders per label, {WARMUP} warmup\n");
    println!(
        "{:<10} {:<9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>10} {:>8}",
        "label", "stage", "mean", "median", "p99", "min", "max", "renders/s", "bytes"
    );

    for &(name, zpl) in LABELS {
        let (instructions, (width, height)) = prepare(zpl);
        let renderer = PdfRenderer::new(width, height);

        let bytes = renderer.render(&instructions).len();
        let samples = measure(iters, || {
            black_box(renderer.render(black_box(&instructions)));
        });
        report(name, "render", &samples, bytes);

        let samples = measure(iters, || {
            let (instructions, (width, height)) = prepare(black_box(zpl));
            black_box(PdfRenderer::new(width, height).render(&instructions));
        });
        report(name, "pipeline", &samples, bytes);
    }
}

/// Parses `zpl` and runs the state manager, as the CLI does.
fn prepare(zpl: &str) -> (Vec<DrawInstruction>, (u32, u32)) {
    let commands = Parser::new(zpl).parse().expect("sample label must parse");
    let mut state = StateManager::new();
    let instructions = state.process(commands);
    (instructions, state.get_label_dimensions())
}

fn measure(iters: usize, mut f: impl FnMut()) -> Vec<Duration> {
    for _ in 0..WARMUP {
        f();
    }
    let mut samples: Vec<Duration> = (0..iters)
        .map(|_| {
            let start = Instant::now();
            f();
            start.elapsed()
        })
        .collect();
    samples.sort_unstable();
    samples
}

fn report(name: &str, stage: &str, samples: &[Duration], bytes: usize) {
    let total: Duration = samples.iter().sum();
    let mean = total / samples.len() as u32;
    let pct = |p: f64| samples[((samples.len() - 1) as f64 * p).round() as usize];
    println!(
        "{:<10} {:<9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>10.0} {:>8}",
        name,
        stage,
        fmt(mean),
        fmt(pct(0.5)),
        fmt(pct(0.99)),
        fmt(samples[0]),
        fmt(samples[samples.len() - 1]),
        samples.len() as f64 / total.as_secs_f64(),
        bytes,
    );
}

fn fmt(d: Duration) -> String {
    format!("{:.1}µs", d.as_secs_f64() * 1e6)
}
