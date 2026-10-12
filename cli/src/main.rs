use pdf_renderer::PdfRenderer;
use png_renderer::PngRenderer;
use std::env;
use std::io::{self, Read};
use std::path::Path;
use std::time::{Duration, Instant};
use zpl_parser::Parser;
use zpl_state_manager::{StateManager, DEFAULT_DPMM, DPMM_VALUES};

/// Wall time of each pipeline stage, printed with `--timings`.
struct Timings {
    stages: Vec<(&'static str, Duration)>,
    last: Instant,
}

impl Timings {
    fn new() -> Self {
        Self { stages: Vec::new(), last: Instant::now() }
    }

    /// Records the time since the previous mark as `stage`.
    fn mark(&mut self, stage: &'static str) {
        let now = Instant::now();
        self.stages.push((stage, now - self.last));
        self.last = now;
    }

    fn get(&self, stage: &str) -> Duration {
        self.stages.iter().filter(|(s, _)| *s == stage).map(|(_, d)| *d).sum()
    }

    fn print(&self, output_bytes: usize, instructions: usize) {
        let total: Duration = self.stages.iter().map(|(_, d)| *d).sum();
        let compute = self.get("parse") + self.get("state") + self.get("render") + self.get("encode");
        let io = self.get("read stdin") + self.get("write file");
        println!("timings:");
        for (stage, d) in &self.stages {
            println!(
                "  {:<11} {:>10}  {:>5.1}%",
                stage,
                fmt(*d),
                100.0 * d.as_secs_f64() / total.as_secs_f64()
            );
        }
        println!("  {:<11} {:>10}", "compute", fmt(compute));
        println!("  {:<11} {:>10}", "io", fmt(io));
        println!("  {:<11} {:>10}", "total", fmt(total));
        println!("  {} draw instructions, {} output bytes", instructions, output_bytes);
    }
}

fn fmt(d: Duration) -> String {
    let us = d.as_secs_f64() * 1e6;
    if us >= 1000.0 {
        format!("{:.2}ms", us / 1000.0)
    } else {
        format!("{:.1}µs", us)
    }
}

fn fail(message: &str, error: impl std::fmt::Debug) -> ! {
    eprintln!("{}: {:?}", message, error);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let debug = args.iter().any(|arg| arg == "--debug" || arg == "-d");
    let timings_enabled = args.iter().any(|arg| arg == "--timings" || arg == "-t");
    // --dpmm N: printer resolution in dots per millimetre (6, 8, 12 or 24; default 8).
    let dpmm_at = args.iter().position(|arg| arg == "--dpmm");
    let dpmm = match dpmm_at.map(|i| args.get(i + 1).map(String::as_str).unwrap_or("")) {
        None => DEFAULT_DPMM,
        Some(value) => match value.parse::<u32>() {
            Ok(dpmm) if DPMM_VALUES.contains(&dpmm) => dpmm,
            _ => fail("--dpmm must be 6, 8, 12 or 24", value),
        },
    };
    let output_file = args
        .iter()
        .enumerate()
        .find(|&(i, arg)| !arg.starts_with('-') && dpmm_at.is_none_or(|at| i != at + 1))
        .map(|(_, s)| s.as_str())
        .unwrap_or("output.png");
    let is_pdf = output_file.to_ascii_lowercase().ends_with(".pdf");

    let mut timings = Timings::new();

    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes).expect("Failed to read from stdin");
    // UTF-8, or else one character per byte (Latin-1) for legacy single-byte labels.
    let input = String::from_utf8(bytes).unwrap_or_else(|e| e.into_bytes().into_iter().map(char::from).collect());
    timings.mark("read stdin");

    let commands = Parser::with_dpmm(&input, dpmm)
        .parse()
        .unwrap_or_else(|e| fail("Parse error", e));
    timings.mark("parse");

    let mut state = StateManager::new();
    state.set_dpmm(dpmm);
    let pages = state.process_pages(commands);
    let instruction_count: usize = pages.iter().map(|p| p.instructions.len()).sum();
    timings.mark("state");

    // One file per label: a PDF holds every page; images are written as `output.png`,
    // `output-2.png`, `output-3.png`, ...
    let outputs: Vec<(String, Vec<u8>)> = if is_pdf {
        let pages: Vec<_> = pages.iter().map(|p| (p.setup, p.instructions.as_slice())).collect();
        let bytes = PdfRenderer::for_page(&pages[0].0).render_pages(&pages);
        timings.mark("render");
        vec![(output_file.to_string(), bytes)]
    } else {
        pages
            .iter()
            .enumerate()
            .map(|(i, page)| {
                let image = PngRenderer::for_page(&page.setup, debug).render(&page.instructions);
                timings.mark("render");
                let path = page_path(output_file, i + 1);
                let bytes = png_renderer::encode_image(&image, &path)
                    .unwrap_or_else(|e| fail("Failed to encode image", e));
                timings.mark("encode");
                (path, bytes)
            })
            .collect()
    };

    // Same figure as before: parse through render, excluding encoding and disk IO.
    if !timings_enabled {
        let duration = timings.get("parse") + timings.get("state") + timings.get("render");
        println!("{:?}", duration);
    }

    for (path, bytes) in &outputs {
        std::fs::write(path, bytes).unwrap_or_else(|e| fail("Failed to save output", e));
    }
    timings.mark("write file");

    if timings_enabled {
        timings.print(outputs.iter().map(|(_, b)| b.len()).sum(), instruction_count);
    }
    if is_pdf {
        let count = pages.len();
        println!("PDF ({count} page{}) saved to: {output_file}", if count == 1 { "" } else { "s" });
    } else {
        for (path, _) in &outputs {
            println!("PNG saved to: {path}");
        }
    }
}

/// Output path for page `n` (1-based): `path` itself for the first, `<stem>-<n>.<ext>` after.
fn page_path(path: &str, n: usize) -> String {
    if n == 1 {
        return path.to_string();
    }
    let path = Path::new(path);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let name = match path.extension() {
        Some(ext) => format!("{stem}-{n}.{}", ext.to_string_lossy()),
        None => format!("{stem}-{n}"),
    };
    path.with_file_name(name).to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::page_path;

    #[test]
    fn page_paths() {
        assert_eq!(page_path("out/label.png", 1), "out/label.png");
        assert_eq!(page_path("out/label.png", 2), "out/label-2.png");
        assert_eq!(page_path("label", 3), "label-3");
    }
}
