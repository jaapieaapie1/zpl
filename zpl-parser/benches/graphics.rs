//! Parses labels that are one large bitmap, as carrier labels often are: a synthetic 824×1182
//! dot page of text-like glyphs as a compressed ASCII ~DG and ^GFA, and zlib-compressed ^GF Z64.
//!
//! Run with `cargo bench -p zpl-parser`. Set `BENCH_ITERS` to change the number of measured
//! parses per label (default 2000).

use std::hint::black_box;
use std::io::Write;
use std::time::{Duration, Instant};

use base64::Engine as _;
use zpl_parser::{Parser, command::Command};

const WIDTH: usize = 824;
const HEIGHT: usize = 1182;
const BYTES_PER_ROW: usize = WIDTH.div_ceil(8);
const WARMUP: usize = 50;

fn main() {
    let iters: usize = std::env::var("BENCH_ITERS").ok().and_then(|v| v.parse().ok()).unwrap_or(2000);
    let bitmap = page();
    let ascii = compress(&bitmap);
    let total = bitmap.len();
    let z64 = {
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&bitmap).unwrap();
        base64::engine::general_purpose::STANDARD.encode(encoder.finish().unwrap())
    };

    let labels = [
        ("~DG", format!("~DGR:PAGE.GRF,{total},{BYTES_PER_ROW},\n{ascii}\n^XA^FO0,0^XGR:PAGE.GRF,1,1^FS^XZ")),
        ("^GFA", format!("^XA^FO0,0^GFA,{total},{total},{BYTES_PER_ROW},\n{ascii}^FS^XZ")),
        ("^GF Z64", format!("^XA^FO0,0^GFA,{total},{total},{BYTES_PER_ROW},:Z64:{z64}:0000^FS^XZ")),
    ];

    println!("{iters} parses per label, {WARMUP} warmup; {total} bitmap bytes\n");
    println!("{:<8} {:>9} {:>9} {:>9} {:>9} {:>8}", "label", "mean", "median", "p99", "min", "zpl");
    for (name, zpl) in &labels {
        let decoded = Parser::new(zpl).parse().expect("label must parse");
        assert!(decoded.iter().any(|c| matches!(c,
            Command::DownloadGraphic { data, .. } | Command::GraphicField { data, .. } if *data == bitmap)));
        let samples = measure(iters, || {
            black_box(Parser::new(black_box(zpl)).parse().unwrap());
        });
        report(name, &samples, zpl.len());
    }
}

/// Lines of pseudo-random "glyphs" with blank space between them, like a printed page.
fn page() -> Vec<u8> {
    let mut seed: u32 = 0x1234_5678;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed
    };
    let mut bitmap = vec![0u8; BYTES_PER_ROW * HEIGHT];
    for line in 0..HEIGHT / 40 {
        let (top, height, margin) = (line * 40 + 8, 24, 16 + (next() % 64) as usize);
        let glyphs: Vec<[u32; 24]> = (0..24).map(|_| std::array::from_fn(|_| next() & next() & 0x3FFF)).collect();
        let text: Vec<usize> = (0..(next() % 50) as usize + 10).map(|_| (next() % 24) as usize).collect();
        for y in 0..height {
            for (ch, &glyph) in text.iter().enumerate() {
                let x0 = margin + ch * 14;
                let bits = glyphs[glyph][y];
                for dx in 0..14 {
                    let x = x0 + dx;
                    if x < WIDTH && bits >> dx & 1 == 1 {
                        bitmap[(top + y) * BYTES_PER_ROW + x / 8] |= 0x80 >> (x % 8);
                    }
                }
            }
        }
    }
    bitmap
}

/// ZPL ASCII compression: repeated rows as `:`, trailing zeros or ones as `,` / `!`, and runs
/// of a digit behind a `G`-`Y` / `g`-`z` repeat count.
fn compress(bitmap: &[u8]) -> String {
    let mut out = String::new();
    let mut previous: Option<&[u8]> = None;
    for row in bitmap.chunks(BYTES_PER_ROW) {
        if previous == Some(row) {
            out.push(':');
            continue;
        }
        previous = Some(row);
        let digits: Vec<u8> = row.iter().flat_map(|b| [b >> 4, b & 0xF]).collect();
        let mut end = digits.len();
        let tail = digits[end - 1];
        while end > 0 && digits[end - 1] == tail && (tail == 0 || tail == 0xF) {
            end -= 1;
        }
        let mut i = 0;
        while i < end {
            let digit = digits[i];
            // One count covers at most 400 + 19 digits.
            let run = digits[i..end].iter().take_while(|&&d| d == digit).count().min(419);
            if run > 1 {
                if run >= 20 {
                    out.push((b'f' + (run / 20) as u8) as char);
                }
                if run % 20 > 0 {
                    out.push((b'F' + (run % 20) as u8) as char);
                }
            }
            out.push(char::from_digit(digit as u32, 16).unwrap().to_ascii_uppercase());
            i += run;
        }
        if end < digits.len() {
            out.push(if tail == 0 { ',' } else { '!' });
        }
        out.push('\n');
    }
    out
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

fn report(name: &str, samples: &[Duration], bytes: usize) {
    let mean = samples.iter().sum::<Duration>() / samples.len() as u32;
    let pct = |p: f64| samples[((samples.len() - 1) as f64 * p).round() as usize];
    println!(
        "{:<8} {:>9} {:>9} {:>9} {:>9} {:>8}",
        name,
        fmt(mean),
        fmt(pct(0.5)),
        fmt(pct(0.99)),
        fmt(samples[0]),
        bytes
    );
}

fn fmt(d: Duration) -> String {
    format!("{:.1}µs", d.as_secs_f64() * 1e6)
}
