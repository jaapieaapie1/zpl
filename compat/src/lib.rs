//! Compatibility harness: renders the labels in `cases/` and compares them with reference
//! renders from Labelary (https://labelary.com), the de-facto reference ZPL renderer.
//!
//! - `cases/<name>.zpl` is a small label exercising one feature. A `^FX compat: WxH`
//!   comment gives the label size in inches (optionally `dpmm=N`, default 8). The word
//!   `labelary-unsupported` marks a feature Labelary does not render (it prints the field
//!   data as text instead); such cases are scored but left out of the summary.
//! - `cases/<name>.labelary.png` is Labelary's render of label 1, `<name>.labelary.2.png`
//!   of label 2 and so on. They are fetched once with `compat/fetch.sh` and committed, so
//!   nothing here needs the network.
//! - `cargo run -p compat --release` scores every case, worst first, and writes diff
//!   images to `compat/out/` (black = both, red = only Labelary, blue = only ours; pale
//!   red/blue = a mismatch within [`TOLERANCE`] dots of the same colour in the other image).
//! - `baselines.txt` records each case's tolerant score; `cargo test -p compat` fails when
//!   a case gets worse. `cargo run -p compat --release -- --update-baselines` rewrites it.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage};
use png_renderer::PngRenderer;
use zpl_parser::Parser;
use zpl_state_manager::StateManager;

/// A mismatched dot within this many dots (horizontally, vertically or diagonally) of a dot
/// of the same colour in the other image is a rounding or anti-aliasing shift, not an error.
pub const TOLERANCE: usize = 2;

/// Luma below this is black, in both renders.
pub const THRESHOLD: u8 = 128;

/// How much a tolerant score may exceed its baseline before the regression test fails;
/// baselines are stored with two decimals.
pub const BASELINE_SLACK: f64 = 0.01;

pub fn cases_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("cases")
}

pub fn baselines_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("baselines.txt")
}

pub fn out_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("out")
}

/// One label file of the corpus.
#[derive(Debug, Clone)]
pub struct Case {
    pub name: String,
    pub path: PathBuf,
    pub zpl: String,
    pub width_in: f32,
    pub height_in: f32,
    pub dpmm: u32,
    /// Labelary does not render this feature, so its reference is not a target.
    pub unsupported: bool,
}

impl Case {
    pub fn load(path: &Path) -> Result<Case, String> {
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        // UTF-8, or else one character per byte (Latin-1), as the CLI reads labels.
        let zpl = String::from_utf8(bytes).unwrap_or_else(|e| e.into_bytes().into_iter().map(char::from).collect());
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
        let (width_in, height_in, dpmm) =
            parse_header(&zpl).ok_or_else(|| format!("{}: missing `^FX compat: WxH` header", path.display()))?;
        let unsupported = zpl.contains("labelary-unsupported");
        Ok(Case { name, path: path.to_path_buf(), zpl, width_in, height_in, dpmm, unsupported })
    }

    /// Labelary's renders of this case, label 1 first. Empty until fetched.
    pub fn references(&self) -> Vec<PathBuf> {
        let dir = self.path.parent().unwrap_or(Path::new("."));
        (1..)
            .map(|n| dir.join(reference_file_name(&self.name, n)))
            .take_while(|p| p.exists())
            .collect()
    }

    /// Our renders of every label, binarized, each `width` x `height` dots.
    pub fn render(&self, width: u32, height: u32) -> Result<Vec<Bitmap>, String> {
        let commands = Parser::new(&self.zpl).parse().map_err(|e| format!("parse error: {e:?}"))?;
        let pages = StateManager::new().process_pages(commands);
        Ok(pages
            .iter()
            .map(|page| Bitmap::from_rgb(&PngRenderer::new(width, height).render(&page.instructions)))
            .collect())
    }
}

/// `<name>.labelary.png` for label 1, `<name>.labelary.<n>.png` after.
pub fn reference_file_name(name: &str, n: usize) -> String {
    if n == 1 { format!("{name}.labelary.png") } else { format!("{name}.labelary.{n}.png") }
}

/// `(width, height, dpmm)` from a `compat: 4x3` or `compat: 4x3 dpmm=12` comment.
fn parse_header(zpl: &str) -> Option<(f32, f32, u32)> {
    let rest = &zpl[zpl.find("compat:")? + "compat:".len()..];
    let line = rest.split(['^', '~', '\n']).next()?;
    let mut words = line.split_whitespace();
    let (w, h) = words.next()?.split_once('x')?;
    let mut dpmm = 8;
    for word in words {
        if let Some(value) = word.strip_prefix("dpmm=") {
            dpmm = value.parse().ok()?;
        }
    }
    Some((w.parse().ok()?, h.parse().ok()?, dpmm))
}

/// Every `.zpl` case in `dir`, sorted by name.
pub fn load_cases(dir: &Path) -> Result<Vec<Case>, String> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "zpl"))
        .collect();
    paths.sort();
    paths.iter().map(|p| Case::load(p)).collect()
}

/// A 1-bit image; `true` is black.
#[derive(Debug, Clone, PartialEq)]
pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub dots: Vec<bool>,
}

impl Bitmap {
    pub fn blank(width: usize, height: usize) -> Bitmap {
        Bitmap { width, height, dots: vec![false; width * height] }
    }

    pub fn from_rgb(image: &RgbImage) -> Bitmap {
        let dots = image
            .pixels()
            .map(|Rgb([r, g, b])| (299 * *r as u32 + 587 * *g as u32 + 114 * *b as u32) / 1000 < THRESHOLD as u32)
            .collect();
        Bitmap { width: image.width() as usize, height: image.height() as usize, dots }
    }

    pub fn open(path: &Path) -> Result<Bitmap, String> {
        let image = image::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(Bitmap::from_rgb(&image.to_rgb8()))
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.dots[y * self.width + x]
    }

    /// Black wherever a black dot lies within `radius` dots (a square neighbourhood).
    fn dilate(&self, radius: usize) -> Bitmap {
        let (w, h) = (self.width, self.height);
        let mut rows = vec![false; w * h];
        for y in 0..h {
            for x in 0..w {
                let (from, to) = (x.saturating_sub(radius), (x + radius).min(w - 1));
                rows[y * w + x] = (from..=to).any(|i| self.dots[y * w + i]);
            }
        }
        let mut dots = vec![false; w * h];
        for y in 0..h {
            let (from, to) = (y.saturating_sub(radius), (y + radius).min(h - 1));
            for x in 0..w {
                dots[y * w + x] = (from..=to).any(|j| rows[j * w + x]);
            }
        }
        Bitmap { width: w, height: h, dots }
    }

    /// This bitmap at `width` x `height`: cropped, or padded with white.
    fn resized(&self, width: usize, height: usize) -> Bitmap {
        let mut out = Bitmap::blank(width, height);
        for y in 0..height.min(self.height) {
            for x in 0..width.min(self.width) {
                out.dots[y * width + x] = self.get(x, y);
            }
        }
        out
    }
}

/// Dot counts of one comparison.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Counts {
    pub dots: u64,
    /// Black in either image.
    pub ink: u64,
    pub only_theirs: u64,
    pub only_ours: u64,
    /// Mismatches with no dot of the same colour within [`TOLERANCE`] in the other image.
    pub far_theirs: u64,
    pub far_ours: u64,
}

impl Counts {
    pub fn add(&mut self, other: Counts) {
        self.dots += other.dots;
        self.ink += other.ink;
        self.only_theirs += other.only_theirs;
        self.only_ours += other.only_ours;
        self.far_theirs += other.far_theirs;
        self.far_ours += other.far_ours;
    }

    /// Mismatched dots, as a percentage of all dots.
    pub fn raw(&self) -> f64 {
        percent(self.only_theirs + self.only_ours, self.dots)
    }

    /// Mismatched dots beyond the tolerance, as a percentage of the dots black in either
    /// image, so a small label element counts as much as a page-filling one.
    pub fn tolerant(&self) -> f64 {
        percent(self.far_theirs + self.far_ours, self.ink)
    }
}

fn percent(part: u64, whole: u64) -> f64 {
    if whole == 0 { 0.0 } else { 100.0 * part as f64 / whole as f64 }
}

/// Compares `ours` with `theirs` (same size) and draws the overlay.
pub fn compare(ours: &Bitmap, theirs: &Bitmap) -> (Counts, RgbImage) {
    assert_eq!((ours.width, ours.height), (theirs.width, theirs.height));
    let (near_ours, near_theirs) = (ours.dilate(TOLERANCE), theirs.dilate(TOLERANCE));
    // A white mismatch is tolerated when the other image has white nearby: dilate the inverse.
    let invert = |b: &Bitmap| Bitmap { dots: b.dots.iter().map(|d| !d).collect(), ..b.clone() };
    let (white_near_ours, white_near_theirs) = (invert(ours).dilate(TOLERANCE), invert(theirs).dilate(TOLERANCE));

    let mut counts = Counts { dots: ours.dots.len() as u64, ..Counts::default() };
    let mut image = RgbImage::from_pixel(ours.width as u32, ours.height as u32, Rgb([255, 255, 255]));
    for i in 0..ours.dots.len() {
        let (o, t) = (ours.dots[i], theirs.dots[i]);
        let colour = match (o, t) {
            (true, true) => Some([0, 0, 0]),
            (false, false) => None,
            (false, true) => {
                counts.only_theirs += 1;
                // Labelary's black dot is fine if we have black nearby and they have white
                // nearby where we are white (a shifted edge, not a missing element).
                if near_ours.dots[i] && white_near_theirs.dots[i] {
                    Some([255, 170, 170])
                } else {
                    counts.far_theirs += 1;
                    Some([220, 0, 0])
                }
            }
            (true, false) => {
                counts.only_ours += 1;
                if near_theirs.dots[i] && white_near_ours.dots[i] {
                    Some([160, 180, 255])
                } else {
                    counts.far_ours += 1;
                    Some([0, 60, 230])
                }
            }
        };
        if o || t {
            counts.ink += 1;
        }
        if let Some(c) = colour {
            image.put_pixel((i % ours.width) as u32, (i / ours.width) as u32, Rgb(c));
        }
    }
    (counts, image)
}

/// The outcome of one case.
#[derive(Debug)]
pub struct CaseResult {
    pub name: String,
    pub unsupported: bool,
    pub our_pages: usize,
    pub their_pages: usize,
    pub counts: Counts,
    /// One overlay per compared label.
    pub diffs: Vec<RgbImage>,
    /// Our renders, for inspection.
    pub renders: Vec<RgbImage>,
}

/// Scores `case` against its references, `None` when it has none yet. Labels only one side
/// produced are compared with a blank label.
pub fn run_case(case: &Case) -> Result<Option<CaseResult>, String> {
    let references = case.references();
    if references.is_empty() {
        return Ok(None);
    }
    let theirs: Vec<Bitmap> = references.iter().map(|p| Bitmap::open(p)).collect::<Result<_, _>>()?;
    let (width, height) = (theirs[0].width, theirs[0].height);
    let ours = case.render(width as u32, height as u32)?;

    let mut counts = Counts::default();
    let (mut diffs, mut renders) = (Vec::new(), Vec::new());
    for i in 0..ours.len().max(theirs.len()) {
        let blank = Bitmap::blank(width, height);
        let o = ours.get(i).unwrap_or(&blank);
        let t = theirs.get(i).map(|t| t.resized(width, height)).unwrap_or_else(|| blank.clone());
        let (c, diff) = compare(o, &t);
        counts.add(c);
        diffs.push(diff);
        renders.push(to_rgb(o));
    }
    Ok(Some(CaseResult {
        name: case.name.clone(),
        unsupported: case.unsupported,
        our_pages: ours.len(), their_pages: theirs.len(), counts, diffs, renders }))
}

fn to_rgb(bitmap: &Bitmap) -> RgbImage {
    RgbImage::from_fn(bitmap.width as u32, bitmap.height as u32, |x, y| {
        if bitmap.get(x as usize, y as usize) { Rgb([0, 0, 0]) } else { Rgb([255, 255, 255]) }
    })
}

/// Recorded tolerant scores by case name.
pub fn load_baselines(path: &Path) -> Result<BTreeMap<String, f64>, String> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|line| {
            let (name, score) =
                line.split_once(char::is_whitespace).ok_or_else(|| format!("bad baseline line: {line}"))?;
            let score = score.trim().parse().map_err(|_| format!("bad baseline score: {line}"))?;
            Ok((name.to_string(), score))
        })
        .collect()
}

pub fn write_baselines(path: &Path, scores: &BTreeMap<String, f64>) -> Result<(), String> {
    let mut text = String::from(
        "# Tolerant mismatch (% of inked dots) per case; `cargo test -p compat` fails above these.\n\
         # Rewrite with: cargo run -p compat --release -- --update-baselines\n",
    );
    for (name, score) in scores {
        // Rounded up, so the stored value never sits below the measured one.
        text.push_str(&format!("{name} {:.2}\n", (score * 100.0).ceil() / 100.0));
    }
    fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_header() {
        assert_eq!(parse_header("^XA^FX compat: 4x3^FS"), Some((4.0, 3.0, 8)));
        assert_eq!(parse_header("^XA\n^FX compat: 2.5x1 dpmm=12\n"), Some((2.5, 1.0, 12)));
        assert_eq!(parse_header("^XA^XZ"), None);
    }

    #[test]
    fn one_dot_shift_is_tolerated() {
        let mut ours = Bitmap::blank(20, 20);
        let mut theirs = Bitmap::blank(20, 20);
        for y in 5..15 {
            for x in 5..10 {
                ours.dots[y * 20 + x] = true;
                theirs.dots[y * 20 + x + 1] = true;
            }
        }
        let (counts, _) = compare(&ours, &theirs);
        assert_eq!(counts.only_ours + counts.only_theirs, 20);
        assert_eq!(counts.tolerant(), 0.0);

        // A missing element is not.
        let (counts, _) = compare(&Bitmap::blank(20, 20), &theirs);
        assert_eq!(counts.tolerant(), 100.0);
    }
}
