use pdf_renderer::PdfRenderer;
use png_renderer::PngRenderer;
use std::env;
use std::io::{self, Read};
use std::time::{Duration, Instant};
use zpl_parser::Parser;
use zpl_state_manager::StateManager;

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
    let output_file = args
        .iter()
        .find(|arg| !arg.starts_with('-'))
        .map(|s| s.as_str())
        .unwrap_or("output.png");
    let is_pdf = output_file.to_ascii_lowercase().ends_with(".pdf");

    let mut timings = Timings::new();

    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes).expect("Failed to read from stdin");
    // UTF-8, or else one character per byte (Latin-1) for legacy single-byte labels.
    let input = String::from_utf8(bytes).unwrap_or_else(|e| e.into_bytes().into_iter().map(char::from).collect());
    timings.mark("read stdin");

    let commands = Parser::new(&input)
        .parse()
        .unwrap_or_else(|e| fail("Parse error", e));
    timings.mark("parse");

    let mut state_manager = StateManager::new();
    let draw_instructions = state_manager.process(commands);
    let page = state_manager.page_setup();
    let (width, height) = (page.width, page.height);
    timings.mark("state");

    // The PDF renderer produces finished file bytes; the PNG renderer produces an image
    // that still has to be encoded.
    let bytes = if is_pdf {
        let bytes = PdfRenderer::for_page(&page).render(&draw_instructions);
        timings.mark("render");
        bytes
    } else {
        let renderer = if debug {
            PngRenderer::with_debug(width, height, true)
        } else {
            PngRenderer::new(width, height)
        };
        let image = renderer.render(&draw_instructions);
        timings.mark("render");
        let bytes = png_renderer::encode_image(&image, output_file)
            .unwrap_or_else(|e| fail("Failed to encode image", e));
        timings.mark("encode");
        bytes
    };

    // Same figure as before: parse through render, excluding encoding and disk IO.
    if !timings_enabled {
        let duration = timings.get("parse") + timings.get("state") + timings.get("render");
        println!("{:?}", duration);
    }

    std::fs::write(output_file, &bytes).unwrap_or_else(|e| fail("Failed to save output", e));
    timings.mark("write file");

    if timings_enabled {
        timings.print(bytes.len(), draw_instructions.len());
    }
    println!("{} saved to: {}", if is_pdf { "PDF" } else { "PNG" }, output_file);
}
