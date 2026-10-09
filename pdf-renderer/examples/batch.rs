//! Converts a large batch of ZPL labels to PDF as fast as possible on all cores, keeping
//! every PDF in memory (no disk writes), and reports throughput and actual CPU usage.
//!
//! ```text
//! cargo run --release -p pdf-renderer --example batch [-- [--progress] <labels> [<threads>]]
//! ```
//!
//! Defaults: 100000 labels, one thread per available core. `--progress` (`-p`) shows a
//! live progress line on stderr. Labels cycle through the sample
//! labels, each with a unique serial-number field so no two inputs are identical.

use std::hint::black_box;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use pdf_renderer::PdfRenderer;
use zpl_parser::Parser;
use zpl_state_manager::StateManager;

const TEMPLATES: &[&str] = &[
    include_str!("../../jelle.zpl"),
    include_str!("../../labelary.zpl"),
    include_str!("../../test.zpl"),
];

/// Labels a worker claims at a time; keeps the shared counter off the hot path.
const CHUNK: usize = 64;

#[derive(Default)]
struct WorkerStats {
    labels: usize,
    bytes: usize,
}

fn main() {
    let (flags, positional): (Vec<String>, Vec<String>) = std::env::args().skip(1).partition(|a| a.starts_with('-'));
    let progress = flags.iter().any(|f| f == "--progress" || f == "-p");
    let mut args = positional.into_iter();
    let total: usize = args.next().map(|a| a.parse().expect("labels must be a number")).unwrap_or(100_000);
    let threads: usize = args
        .next()
        .map(|a| a.parse().expect("threads must be a number"))
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()));

    // Strip the closing ^XZ so a unique field can be appended to each label.
    let templates: Vec<&str> = TEMPLATES.iter().map(|t| t.trim_end().trim_end_matches("^XZ")).collect();

    println!("{total} labels -> PDF on {threads} threads (in memory, no disk writes)");

    let next = AtomicUsize::new(0);
    // Labels finished so far, updated once per chunk.
    let done = AtomicUsize::new(0);
    let finished = AtomicBool::new(false);
    let cpu_before = cpu_time();
    let start = Instant::now();

    let stats: Vec<WorkerStats> = std::thread::scope(|scope| {
        if progress {
            scope.spawn(|| report_progress(&done, &finished, total, start));
        }

        let workers: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(|| {
                    let mut stats = WorkerStats::default();
                    let mut zpl = String::new();
                    loop {
                        let first = next.fetch_add(CHUNK, Ordering::Relaxed);
                        if first >= total {
                            return stats;
                        }
                        let last = (first + CHUNK).min(total);
                        for i in first..last {
                            zpl.clear();
                            zpl.push_str(templates[i % templates.len()]);
                            zpl.push_str(&format!("^FO20,1150^A0N,30,30^FDSN{i:08}^FS^XZ"));

                            let pdf = render(&zpl);
                            stats.labels += 1;
                            stats.bytes += black_box(pdf).len();
                        }
                        done.fetch_add(last - first, Ordering::Relaxed);
                    }
                })
            })
            .collect();
        let results: Vec<_> = workers.into_iter().map(|w| w.join()).collect();
        // Stop the reporter before propagating a worker panic, or the scope would wait on it.
        finished.store(true, Ordering::Relaxed);
        results.into_iter().map(|r| r.expect("worker panicked")).collect()
    });

    let wall = start.elapsed();
    let cpu_after = cpu_time();
    let (user, sys) = (cpu_after.0 - cpu_before.0, cpu_after.1 - cpu_before.1);
    let cpu = user + sys;

    let labels: usize = stats.iter().map(|s| s.labels).sum();
    let bytes: usize = stats.iter().map(|s| s.bytes).sum();
    assert_eq!(labels, total);

    let secs = wall.as_secs_f64();
    let cores_busy = cpu.as_secs_f64() / secs;
    println!();
    println!("wall time      {:>10.3} s", secs);
    println!("throughput     {:>10.0} labels/s", labels as f64 / secs);
    println!("per label      {:>10.1} µs wall, {:.1} µs CPU", secs * 1e6 / labels as f64, cpu.as_secs_f64() * 1e6 / labels as f64);
    println!("cpu time       {:>10.3} s  (user {:.3} s, sys {:.3} s)", cpu.as_secs_f64(), user.as_secs_f64(), sys.as_secs_f64());
    println!("cpu usage      {:>10.0} %  ({:.2} of {} threads busy, {:.0}% utilisation)", cores_busy * 100.0, cores_busy, threads, cores_busy * 100.0 / threads as f64);
    println!("output         {:>10.1} MB  ({:.1} KB avg per PDF)", bytes as f64 / 1e6, bytes as f64 / labels as f64 / 1e3);
    println!("peak rss       {:>10.1} MB", peak_rss_bytes() as f64 / 1e6);
    println!("labels/thread  {:?}", stats.iter().map(|s| s.labels).collect::<Vec<_>>());
}

/// Redraws a single progress line on stderr every 100 ms until the batch is finished.
fn report_progress(done: &AtomicUsize, finished: &AtomicBool, total: usize, start: Instant) {
    const WIDTH: usize = 30;
    let mut stderr = std::io::stderr();
    loop {
        let stop = finished.load(Ordering::Relaxed);
        let count = done.load(Ordering::Relaxed).min(total);
        let elapsed = start.elapsed().as_secs_f64();
        let fraction = count as f64 / total.max(1) as f64;
        let rate = count as f64 / elapsed.max(1e-9);
        let eta = if count > 0 { format!("{:.1}s", (total - count) as f64 / rate) } else { "--".to_string() };
        let filled = (fraction * WIDTH as f64).round() as usize;
        let _ = write!(
            stderr,
            "\r[{}{}] {count}/{total} {:5.1}%  {rate:>7.0} labels/s  elapsed {elapsed:.1}s  eta {eta} ",
            "#".repeat(filled),
            "-".repeat(WIDTH - filled),
            fraction * 100.0,
        );
        let _ = stderr.flush();
        if stop || count >= total {
            let _ = writeln!(stderr);
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Parse, lay out and render one label, as the CLI and HTTP API do.
fn render(zpl: &str) -> Vec<u8> {
    let commands = Parser::new(zpl).parse().expect("sample label must parse");
    let mut state = StateManager::new();
    let instructions = state.process(commands);
    PdfRenderer::for_page(&state.page_setup()).render(&instructions)
}

fn rusage() -> libc::rusage {
    // SAFETY: getrusage only writes into the zeroed struct we pass it.
    unsafe {
        let mut usage: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut usage);
        usage
    }
}

/// Process (user, system) CPU time across all threads.
fn cpu_time() -> (Duration, Duration) {
    let usage = rusage();
    let tv = |t: libc::timeval| Duration::new(t.tv_sec as u64, t.tv_usec as u32 * 1000);
    (tv(usage.ru_utime), tv(usage.ru_stime))
}

fn peak_rss_bytes() -> u64 {
    let max = rusage().ru_maxrss as u64;
    // macOS reports bytes, Linux kilobytes.
    if cfg!(target_os = "macos") { max } else { max * 1024 }
}
