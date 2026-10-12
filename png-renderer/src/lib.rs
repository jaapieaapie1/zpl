use std::sync::Arc;

use ab_glyph::{FontArc, FontRef, FontVec, PxScale};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use zpl_parser::command::{
    BlockJustification, Code128Mode, DiagonalDirection, FieldDirection, FontName, Justification, LineColor,
    LinearKind, Orientation, TextLayout,
};
use zpl_state_manager::{DrawInstruction, PageSetup, TextAnchor, TextBlock, DEFAULT_DPMM};
use zpl_symbols::linear::{self, LinearSymbol, Symbology};
use zpl_symbols::matrix::{self, Matrix};

pub mod code128;
mod layout;
pub mod qr;
mod symbols;

pub use layout::DEJAVU_SANS_BOLD;

/// Font files, shared with pdf-renderer so the binary holds one copy of each.
pub static TRIUMVIRATE: &[u8] = include_bytes!("../fonts/ATTriumvirate-BoldCd.otf");
pub static DEJAVU_SANS_MONO: &[u8] = include_bytes!("../fonts/DejaVuSansMono.ttf");
/// Bitmap fonts E (OCR-B) and H (OCR-A); see fonts/LICENSE_OCR.
pub static OCR_B: &[u8] = include_bytes!("../fonts/OCRB.otf");
pub static OCR_A: &[u8] = include_bytes!("../fonts/OCRA.ttf");

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

pub struct PngRenderer {
    width: u32,
    height: u32,
    debug: bool,
    /// Printer resolution, for the bitmap font matrices and MaxiCode.
    dpmm: u32,
    /// Printer dots per format dot (^JM / ^MU, see [`PageSetup::scale`]): the page is drawn
    /// in format dots and then enlarged to printer dots.
    scale: (u32, u32),
    /// ^POI: the finished page is turned 180 degrees.
    inverted: bool,
    /// ^PMY: the finished page is mirrored left to right.
    mirrored: bool,
    /// Area drawn by the instruction being rendered: [x0, y0, x1, y1), clipped to the page.
    #[cfg(feature = "source-map")]
    drawn: std::cell::Cell<Option<[i64; 4]>>,
}

/// The page area an instruction drew on, in dots.
#[cfg(feature = "source-map")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Where a text field goes: rotation, ^FO / ^FT justification and which point the origin is.
#[derive(Clone, Copy)]
struct Placement {
    orientation: Orientation,
    justification: Justification,
    anchor: TextAnchor,
}

/// Text font of a field at its size: the scalable font 0 (Triumvirate), a downloaded
/// TrueType/OpenType font, or a monospaced stand-in for the bitmap fonts A–H.
struct TextFont {
    font: FontArc,
    /// The font's bytes (for ^PA shaping) and cap height in em.
    data: layout::FontData,
    cap_em: f32,
    scale: PxScale,
    /// Distance from ab_glyph's draw origin (ascent line) down to the cap line.
    y_offset: i32,
    /// Cap height in dots: distance from the top of capitals to the baseline.
    cap: i32,
    /// Line height in dots (em size, or the bitmap cell height).
    line_h: u32,
    /// Bitmap fonts: (character pitch, pen offset into the cell) in dots.
    cell: Option<(f32, f32)>,
    /// Uppercase-only bitmap font (B, H).
    uppercase: bool,
    /// ^TB line pitch in line heights: Labelary spaces font 0 lines about 1.26 apart, the
    /// bitmap fonts a little over one (E exactly one).
    block_pitch: f32,
}

/// One of Zebra's bitmap fonts A–H as Labelary draws it: an outline font ([`bitmap_face`])
/// at a fixed size, one character per cell. Dots, at magnification 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitmapCell {
    /// Matrix (H x W) the magnifications are multiples of (docs/commands/other/FONTS.md).
    pub height: u32,
    pub width: u32,
    /// Distance from one character's cell to the next: the width plus the intercharacter gap.
    pub pitch: f32,
    /// Baseline, down from the top of the cell.
    pub baseline: u32,
    /// Em size of the outline font, both ways.
    pub em: f32,
    /// Uppercase-only character set (B prints lowercase as capitals, H leaves it blank).
    pub uppercase: bool,
}

/// Cap height of DejaVu Sans Mono and OCR-A, in em.
const DEJAVU_CAP_EM: f32 = 1493.0 / 2048.0;
const OCR_A_CAP_EM: f32 = 0.74;

/// Bitmap font `font` on a `dpmm` printhead; `None` for the scalable fonts. DejaVu Sans Mono
/// and OCR-A capitals reach from the baseline to the top of the cell, OCR-B's em is the
/// matrix height. Only E and H change with the resolution, and not quite as the Zebra
/// tables say: their pitch and baseline are Labelary's (compat/cases/font_eh_*).
pub fn bitmap_cell(font: FontName, dpmm: u32) -> Option<BitmapCell> {
    let cell = |height, width, pitch, baseline, em, uppercase| BitmapCell { height, width, pitch, baseline, em, uppercase };
    let dejavu = |height, width, gap, baseline, uppercase| {
        cell(height, width, (width + gap) as f32, baseline, baseline as f32 / DEJAVU_CAP_EM, uppercase)
    };
    let ocr_a = |height, width, pitch, baseline| cell(height, width, pitch, baseline, baseline as f32 / OCR_A_CAP_EM, true);
    Some(match font {
        FontName::FontA => dejavu(9, 5, 1, 7, false),
        FontName::FontB => dejavu(11, 7, 2, 11, true),
        FontName::FontC | FontName::FontD => dejavu(18, 10, 2, 14, false),
        FontName::FontE => match dpmm {
            0..=6 => cell(21, 10, 15.0, 16, 21.0, false),
            7..=8 => cell(28, 15, 20.0, 23, 28.0, false),
            _ => cell(42, 20, 26.0, 33, 42.0, false),
        },
        FontName::FontF => dejavu(26, 13, 3, 21, false),
        FontName::FontG => dejavu(60, 40, 8, 48, false),
        FontName::FontH => match dpmm {
            0..=6 => ocr_a(17, 11, 15.4, 17),
            7..=8 => ocr_a(21, 13, 19.0, 21),
            _ => ocr_a(34, 22, 28.0, 30),
        },
        _ => return None,
    })
}

/// Bytes a 2D symbol encodes for its field data, as Labelary does: characters up to U+00FF
/// as that one byte (^FH escapes arrive as such, see the state manager), others as UTF-8.
pub fn symbol_bytes(data: &str) -> std::borrow::Cow<'_, [u8]> {
    if data.is_ascii() {
        return std::borrow::Cow::Borrowed(data.as_bytes());
    }
    let mut out = Vec::with_capacity(data.len());
    for c in data.chars() {
        match u8::try_from(c) {
            Ok(b) => out.push(b),
            Err(_) => out.extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }
    out.into()
}

/// Whether Labelary prints `c` in bitmap font `font` (compat/cases/ci28_bitmap_*); other
/// characters are blank. OCR-A (H) has ASCII only, OCR-B (E) ASCII and a few national
/// characters, the others the Latin-1, Windows-1252 and code page 850 repertoire.
pub fn bitmap_prints(font: FontName, c: char) -> bool {
    match font {
        _ if c.is_ascii() => true,
        FontName::FontH => false,
        FontName::FontE => "ÄÖÜäöüßÆØæø§¤".contains(c),
        _ => {
            matches!(c, '\u{a0}'..='\u{ff}' | '\u{2500}'..='\u{25a0}')
                || "ŒœŠšŽžŸƒˆ˜–—‘’‚“”„†‡•…‰‹›€™ı‗".contains(c)
        }
    }
}

/// ^GS symbol matrix (dots, every resolution).
pub const SYMBOL_CELL: u32 = 24;

/// Ink box (W x H dots, magnification 1) Labelary draws ^GS symbol `c` in, at the top left
/// of the cell: ® and © 15 x 15, ™ 19 x 10, the UL and CSA marks (circled L and c here)
/// the cell's height.
pub fn symbol_ink(c: char) -> (u32, u32) {
    match c {
        '\u{00AE}' | '\u{00A9}' => (15, 15),
        '\u{2122}' => (19, 10),
        '\u{24D2}' => (22, 24),
        _ => (24, 24),
    }
}

/// Font and glyph a ^GS symbol is drawn with: Triumvirate's ® and ©, DejaVu Sans Bold's ™
/// (closer to Labelary's), and for the UL and CSA marks (no font here has them, nor the
/// circled letters standing in for them) DejaVu Sans Bold's large circle.
pub fn symbol_glyph(c: char) -> (&'static [u8], char) {
    match c {
        '\u{00AE}' | '\u{00A9}' => (TRIUMVIRATE, c),
        '\u{2122}' => (DEJAVU_SANS_BOLD, c),
        _ => (DEJAVU_SANS_BOLD, '\u{25EF}'),
    }
}

/// Outline font Labelary draws bitmap font `font` with: OCR-B for E, OCR-A for H, DejaVu
/// Sans Mono for the others.
pub fn bitmap_face(font: FontName) -> &'static [u8] {
    match font {
        FontName::FontE => OCR_B,
        FontName::FontH => OCR_A,
        _ => DEJAVU_SANS_MONO,
    }
}

/// Text in a built-in font as Labelary prints it: soft hyphens not at all, and hyphens and
/// en dashes as the wide minus sign of a font that has one (Triumvirate, DejaVu Sans Mono).
pub fn dashes(text: &str, minus: bool) -> std::borrow::Cow<'_, str> {
    if !text.contains(['-', '\u{2013}', '\u{ad}']) {
        return std::borrow::Cow::Borrowed(text);
    }
    text.chars()
        .filter(|&c| c != '\u{ad}')
        .map(|c| if minus && matches!(c, '-' | '\u{2013}') { '\u{2212}' } else { c })
        .collect()
}

/// ^FB text with its soft hyphens (`\-`) as plain dashes; `\\` (a backslash) is kept for
/// the line breaking to unescape.
pub fn soft_hyphens(text: &str) -> std::borrow::Cow<'_, str> {
    if !text.contains("\\-") {
        return std::borrow::Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, chars.peek()) {
            ('\\', Some('-')) => {}
            ('\\', Some('\\')) => {
                out.push(c);
                out.extend(chars.next());
            }
            _ => out.push(c),
        }
    }
    out.into()
}

/// Smallest font 0 size (dots); docs/commands/zpl/A.md.
const MIN_SCALABLE: u32 = 10;

/// Fonts P–V: glyph size against the H x W matrix (see `TextFont::new`).
const FIXED_EM_H: f32 = 0.82;
const FIXED_EM_W: f32 = 0.945;

/// Fonts P–V: fixed-size font 0 (H x W).
fn fixed_scalable(font: FontName) -> Option<(u32, u32)> {
    Some(match font {
        // The tables say 20 x 18; Labelary draws P as if 17 wide (compat/cases/font_p).
        FontName::FontP => (20, 17),
        FontName::FontQ => (28, 24),
        FontName::FontR => (35, 31),
        FontName::FontS => (40, 35),
        FontName::FontT => (48, 42),
        FontName::FontU => (59, 53),
        FontName::FontV => (80, 71),
        _ => return None,
    })
}

/// Integer magnifications (vertical, horizontal), nearest multiple of the matrix, 1 to 10.
/// A size of 0 was not given and takes the other one's factor (see pdf-renderer's `magnification`).
fn magnification(height: u32, width: u32, base_h: u32, base_w: u32) -> (u32, u32) {
    let mag = |v: u32, base: u32| (v > 0).then(|| ((v as f32 / base as f32).round() as u32).clamp(1, 10));
    match (mag(height, base_h), mag(width, base_w)) {
        (Some(h), Some(w)) => (h, w),
        (Some(m), None) | (None, Some(m)) => (m, m),
        (None, None) => (1, 1),
    }
}

/// Downloaded fonts parsed recently, so each field does not copy and reparse the bytes.
fn custom_font(data: &Arc<Vec<u8>>) -> Option<FontArc> {
    use std::cell::RefCell;
    type Cache = Vec<(Arc<Vec<u8>>, Option<FontArc>)>;
    thread_local! {
        static CACHE: RefCell<Cache> = const { RefCell::new(Vec::new()) };
    }
    CACHE.with_borrow_mut(|cache| {
        if let Some((_, font)) = cache.iter().find(|(d, _)| Arc::ptr_eq(d, data)) {
            return font.clone();
        }
        let font = FontVec::try_from_vec(data.to_vec()).ok().map(FontArc::new);
        if cache.len() >= 8 {
            cache.remove(0);
        }
        cache.push((Arc::clone(data), font.clone()));
        font
    })
}

impl TextFont {
    fn new(name: FontName, custom: Option<&Arc<Vec<u8>>>, height: u32, width: u32, dpmm: u32) -> Self {
        use ab_glyph::Font;
        if let (None, Some(cell)) = (custom, bitmap_cell(name, dpmm)) {
            let data = bitmap_face(name);
            let font = FontArc::try_from_slice(data).expect("Error loading font");
            let (mh, mw) = magnification(height, width, cell.height, cell.width);
            let upem = font.units_per_em().unwrap_or(2048.0);
            // Outline bounds may be y-down; 'H' sits on the baseline, so its extent is the cap height.
            let cap_unscaled =
                font.outline(font.glyph_id('H')).map_or(upem * 0.73, |o| (o.bounds.max.y - o.bounds.min.y).abs());
            let (em_y, em_x) = (cell.em * mh as f32, cell.em * mw as f32);
            let px_per_em = font.height_unscaled() / upem;
            let scale = PxScale { x: em_x * px_per_em, y: em_y * px_per_em };
            let baseline = (cell.baseline * mh) as f32;
            let y_offset = (font.ascent_unscaled() / upem * em_y - baseline).round() as i32;
            // Every glyph's advance is centred on the matrix width.
            let advance = font.h_advance_unscaled(font.glyph_id('M')) / upem * em_x;
            return Self {
                font,
                data: layout::FontData::Static(data),
                cap_em: cap_unscaled / upem,
                scale,
                y_offset,
                cap: baseline.round() as i32,
                line_h: cell.height * mh,
                cell: Some((cell.pitch * mw as f32, ((cell.width * mw) as f32 - advance) / 2.0)),
                uppercase: cell.uppercase,
                block_pitch: match name {
                    FontName::FontA => 1.074,
                    FontName::FontE => 1.0,
                    _ => 1.05,
                },
            };
        }

        let custom_font = custom.and_then(|data| custom_font(data).map(|font| (font, data)));
        let data = match &custom_font {
            Some((_, data)) => layout::FontData::Shared(Arc::clone(data)),
            None => layout::FontData::Static(TRIUMVIRATE),
        };
        let (font, cap_em) = match custom_font.map(|(font, _)| font) {
            Some(font) => {
                let upem = font.units_per_em().unwrap_or(1000.0);
                let cap = font
                    .outline(font.glyph_id('H'))
                    .map(|o| (o.bounds.max.y - o.bounds.min.y).abs() / upem)
                    .filter(|&c| c > 0.0)
                    .unwrap_or(0.7);
                (font, cap)
            }
            None => (FontArc::try_from_slice(TRIUMVIRATE).expect("Error loading font"), 0.75),
        };
        let fixed = fixed_scalable(name).filter(|_| custom.is_none());
        let (height, width) = match fixed {
            Some((h, w)) => {
                let (mh, mw) = magnification(height, width, h, w);
                (h * mh, w * mw)
            }
            None => (height, width),
        };

        // Zebra sizes scalable fonts in em units, ab_glyph's PxScale is ascent - descent.
        // A size given alone sets both.
        let units_per_em = font.units_per_em().unwrap_or(1000.0);
        let px_per_em = font.height_unscaled() / units_per_em;
        let height = if height == 0 { width } else { height };
        let width = if width == 0 { height } else { width };
        // Font 0 is at least 10 dots each way (Labelary).
        let (height, width) = match custom {
            None => (height.max(MIN_SCALABLE), width.max(MIN_SCALABLE)),
            Some(_) => (height, width),
        };
        // Fonts P–V: Labelary draws font 0 smaller than the matrix (its glyphs about 82% as
        // tall and 94.5% as wide) on the baseline of a full-size one.
        let (em_h, em_w) = match fixed {
            Some(_) => (height as f32 * FIXED_EM_H, width as f32 * FIXED_EM_W),
            None => (height as f32, width as f32),
        };
        let scale = PxScale { x: em_w * px_per_em, y: em_h * px_per_em };

        // Zebra places the top of capitals at the field origin, ab_glyph places the ascent there
        let cap = (height as f32 * cap_em).round() as i32;
        let y_offset = (font.ascent_unscaled() / units_per_em * em_h - height as f32 * cap_em).round() as i32;
        let line_h = height;
        Self { font, data, cap_em, scale, y_offset, cap, line_h, cell: None, uppercase: false, block_pitch: 1.26 }
    }

    /// ^GS `symbol` in the magnified 24 x 24 cell, the cell as the line height: its outline
    /// stretched over [`symbol_ink`] at the top left of the cell, drawn as a one-character cell.
    fn symbol(height: u32, width: u32, symbol: char) -> Self {
        use ab_glyph::Font;
        let (data, glyph) = symbol_glyph(symbol);
        let font = FontArc::try_from_slice(data).expect("Error loading font");
        let (mh, mw) = magnification(height, width, SYMBOL_CELL, SYMBOL_CELL);
        let (cell_h, cell_w) = (SYMBOL_CELL * mh, SYMBOL_CELL * mw);
        let upem = font.units_per_em().unwrap_or(1000.0);
        let px_per_em = font.height_unscaled() / upem;
        // Outline bounds may be y-down: the top is the larger y either way.
        let ink = font.outline(font.glyph_id(glyph)).map(|o| o.bounds);
        let (x_min, y_max, ink_w, ink_h) = ink.map_or((0.0, upem * 0.75, upem, upem), |b| {
            (b.min.x, b.max.y.max(b.min.y), b.width(), (b.max.y - b.min.y).abs())
        });
        let (w, h) = symbol_ink(symbol);
        let (em_w, em_h) = ((w * mw) as f32 * upem / ink_w, (h * mh) as f32 * upem / ink_h);
        let scale = PxScale { x: em_w * px_per_em, y: em_h * px_per_em };
        let y_offset = ((font.ascent_unscaled() - y_max) / upem * em_h).round() as i32;
        // ^FT puts the bottom of the cell on the origin.
        let cap = cell_h as i32;
        let cell = Some((cell_w as f32, -x_min / upem * em_w));
        let data = layout::FontData::Static(data);
        Self { font, data, cap_em: 0.75, scale, y_offset, cap, line_h: cell_h, cell, uppercase: false, block_pitch: 1.26 }
    }

    fn has_glyph(&self, c: char) -> bool {
        use ab_glyph::Font;
        self.font.glyph_id(c).0 != 0
    }

    /// Whether `text` has a character this font has no glyph for (Triumvirate covers ASCII).
    fn lacks_glyph(&self, text: &str) -> bool {
        use ab_glyph::Font;
        !text.is_ascii() && text.chars().any(|c| self.font.glyph_id(c).0 == 0)
    }

    /// Distance between ^TB lines in dots.
    fn text_block_pitch(&self) -> f32 {
        self.line_h as f32 * self.block_pitch
    }

    /// Advance width of `text` in dots.
    fn measure(&self, text: &str) -> f32 {
        use ab_glyph::{Font, ScaleFont};
        if let Some((pitch, _)) = self.cell {
            return pitch * text.chars().count() as f32;
        }
        let scaled = self.font.as_scaled(self.scale);
        text.chars().map(|c| scaled.h_advance(self.font.glyph_id(c))).sum()
    }

    /// Draws one line with the top of its capitals at `top`.
    fn draw(&self, image: &mut RgbImage, x: i32, top: i32, line: &str) {
        let y = top - self.y_offset;
        match self.cell {
            None => drawing::draw_text_mut(image, BLACK, x, y, self.scale, &self.font, line),
            Some((pitch, offset)) => {
                let mut buf = [0u8; 4];
                for (i, c) in line.chars().enumerate() {
                    let cx = (x as f32 + i as f32 * pitch + offset).round() as i32;
                    drawing::draw_text_mut(image, BLACK, cx, y, self.scale, &self.font, c.encode_utf8(&mut buf));
                }
            }
        }
    }
}

/// Space between the bars and an interpretation line (dots, matches Labelary): below them a
/// fixed 6 (EAN/UPC: twice the module, their guard bars reaching into the digits), above
/// them 8 plus twice the module.
fn line_gap(module: u32, above: bool, guards: bool) -> f32 {
    match (above, guards) {
        (true, _) => (8 + 2 * module) as f32,
        (false, true) => (2 * module) as f32,
        (false, false) => 6.0,
    }
}

/// How far EAN/UPC guard bars reach below the others when the digits are printed above.
const GUARD_EXTENSION_ABOVE: u32 = 13;

/// Interpretation line font for 1D barcodes: monospaced; pitch and digit height scale with
/// the module width up to 10 dots (matches Labelary).
struct Interpretation {
    font: FontRef<'static>,
    scale: PxScale,
    char_advance: f32,
    digit_height: f32,
    ascent: f32,
    /// Negative, as ab_glyph reports it.
    descent: f32,
}

impl Interpretation {
    fn new(bar_width: u32) -> Self {
        use ab_glyph::{Font, ScaleFont};

        let font_data: &'static [u8] = include_bytes!("../fonts/DejaVuSansMono.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Error loading font");
        let bar_width = bar_width.min(10);
        let char_advance = bar_width as f32 * 143.0 / 24.0;
        let digit_height = (bar_width * 7) as f32;
        let zero = font.glyph_id('0');
        let zero_height_unscaled = font
            .outline(zero)
            .map(|o| (o.bounds.max.y - o.bounds.min.y).abs())
            .unwrap_or(font.ascent_unscaled());
        let scale = PxScale {
            x: char_advance * font.height_unscaled() / font.h_advance_unscaled(zero),
            y: digit_height * font.height_unscaled() / zero_height_unscaled,
        };
        let scaled = font.as_scaled(scale);
        let (ascent, descent) = (scaled.ascent(), scaled.descent());
        Self { font, scale, char_advance, digit_height, ascent, descent }
    }

    /// Height of the text and its gap to the bars, in whole dots.
    fn area(&self, gap: f32, above: bool) -> u32 {
        let below = if above { 0.0 } else { -self.descent };
        (gap + self.digit_height + below).ceil() as u32
    }

    fn text_width(&self, text: &str) -> f32 {
        self.char_advance * text.chars().count() as f32
    }

    /// Draws `text` one character per pitch, starting at `x` with its baseline at `baseline`.
    fn draw(&self, image: &mut RgbImage, text: &str, x: f32, baseline: f32) {
        let text_y = baseline - self.ascent;
        for (i, ch) in text.chars().enumerate() {
            let ch_x = x + i as f32 * self.char_advance;
            drawing::draw_text_mut(
                image,
                BLACK,
                ch_x.round() as i32,
                text_y.round() as i32,
                self.scale,
                &self.font,
                &ch.to_string(),
            );
        }
    }
}

fn symbology(kind: LinearKind) -> Symbology {
    match kind {
        LinearKind::Code11 { check_digits } => Symbology::Code11 { check_digits },
        LinearKind::Interleaved2of5 { check_digit } => Symbology::Interleaved2of5 { check_digit },
        LinearKind::Code39 { check_digit } => Symbology::Code39 { check_digit },
        LinearKind::Ean8 => Symbology::Ean8,
        LinearKind::UpcE { .. } => Symbology::UpcE,
        LinearKind::Code93 { print_check_digit } => Symbology::Code93 { check_digit: print_check_digit },
        LinearKind::Ean13 => Symbology::Ean13,
        LinearKind::Industrial2of5 => Symbology::Industrial2of5,
        LinearKind::Standard2of5 => Symbology::Standard2of5,
        LinearKind::Codabar { check_digit, .. } => Symbology::Codabar { check_digit },
        LinearKind::Msi { .. } => Symbology::Msi { check: linear::MsiCheck::None },
        LinearKind::Plessey { print_check_digit } => Symbology::Plessey { check_digit: print_check_digit },
        LinearKind::UpcEanExtension => Symbology::UpcEanExtension,
        LinearKind::UpcA { .. } => Symbology::UpcA,
    }
}

fn encode_linear(kind: LinearKind, data: &str) -> Option<LinearSymbol> {
    use zpl_parser::command::MsiCheck as Zpl;
    let mut symbol = match kind {
        LinearKind::Codabar { check_digit, start, stop } => linear::encode_codabar(data, start, stop, check_digit),
        LinearKind::Msi { check, print_check_digit } => {
            let check = match check {
                Zpl::None => linear::MsiCheck::None,
                Zpl::Mod10 => linear::MsiCheck::Mod10,
                Zpl::Mod10Mod10 => linear::MsiCheck::Mod10Mod10,
                Zpl::Mod11Mod10 => linear::MsiCheck::Mod11Mod10,
            };
            linear::encode_msi(data, check, print_check_digit)
        }
        kind => linear::encode(symbology(kind), data),
    }
    .ok()?;
    // UPC-A/UPC-E print the check digit as the last (outside) text segment.
    if matches!(
        kind,
        LinearKind::UpcA { print_check_digit: false } | LinearKind::UpcE { print_check_digit: false }
    ) {
        symbol.text.pop();
    }
    Some(symbol)
}

/// Greedy word wrap of `text` into lines no wider than `width` (the first line) or
/// `width - indent` (the others). `\&` forces a line break; overlong words are split.
#[cfg(test)]
fn wrap_lines(font: &TextFont, text: &str, width: u32, indent: u32) -> Vec<String> {
    wrap_lines_with(&|s: &str| font.measure(s), text, width, indent).into_iter().map(|(line, _)| line).collect()
}

/// [`wrap_lines`] with `measure` giving the width of a line; each line is paired with
/// whether it was broken at a space (Labelary keeps that space when justifying).
fn wrap_lines_with(measure: &dyn Fn(&str) -> f32, text: &str, width: u32, indent: u32) -> Vec<(String, bool)> {
    let mut lines: Vec<(String, bool)> = Vec::new();
    let avail = |lines: &Vec<(String, bool)>| {
        if lines.is_empty() { width as f32 } else { width.saturating_sub(indent).max(1) as f32 }
    };
    for paragraph in text.split("\\&") {
        let mut line = String::new();
        for word in paragraph.split(' ').filter(|w| !w.is_empty()) {
            let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
            if measure(&candidate) <= avail(&lines) {
                line = candidate;
                continue;
            }
            if !line.is_empty() {
                lines.push((std::mem::take(&mut line), true));
            }
            // Split a word that does not fit on a line of its own.
            for c in word.chars() {
                line.push(c);
                if measure(&line) > avail(&lines) && line.chars().count() > 1 {
                    line.pop();
                    lines.push((std::mem::take(&mut line), false));
                    line.push(c);
                }
            }
        }
        lines.push((line, false));
    }
    lines
}

impl PngRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_debug(width, height, false)
    }

    pub fn with_debug(width: u32, height: u32, debug: bool) -> Self {
        Self {
            width,
            height,
            debug,
            dpmm: DEFAULT_DPMM,
            scale: (1, 1),
            inverted: false,
            mirrored: false,
            #[cfg(feature = "source-map")]
            drawn: std::cell::Cell::new(None),
        }
    }

    /// Renderer for a label's page settings: size, printer resolution, ^JM / ^MU format
    /// scale and ^PO / ^PM. The image has one pixel per printer dot.
    pub fn for_page(page: &PageSetup, debug: bool) -> Self {
        Self {
            dpmm: page.dpmm,
            scale: page.scale,
            inverted: page.inverted,
            mirrored: page.mirrored,
            ..Self::with_debug(page.width, page.height, debug)
        }
    }

    pub fn render(&self, instructions: &[DrawInstruction]) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::from_pixel(self.width, self.height, WHITE);

        for instruction in instructions {
            self.render_instruction(&mut image, instruction);
        }

        self.orient(&mut image);
        self.to_printer_dots(image)
    }

    /// ^POI turns the finished page upside down, ^PMY mirrors it left to right; both together
    /// flip it top to bottom. Nothing to do (and no copy) when neither is set.
    fn orient(&self, image: &mut RgbImage) {
        use image::imageops;
        match (self.inverted, self.mirrored) {
            (false, false) => {}
            (true, false) => imageops::rotate180_in_place(image),
            (false, true) => imageops::flip_horizontal_in_place(image),
            (true, true) => imageops::flip_vertical_in_place(image),
        }
    }

    /// Enlarges a page drawn in format dots to printer dots (nearest neighbour, as the
    /// printer burns each format dot as a block of dots); a no-op at scale 1.
    fn to_printer_dots(&self, image: RgbImage) -> RgbImage {
        let (num, den) = self.scale;
        if num == den {
            return image;
        }
        let (w, h) = (self.printer_dots(image.width()), self.printer_dots(image.height()));
        let src = |v: u32| (v as u64 * den as u64 / num as u64) as u32;
        let cols: Vec<u32> = (0..w).map(|x| src(x).min(image.width() - 1)).collect();
        ImageBuffer::from_fn(w, h, |x, y| *image.get_pixel(cols[x as usize], src(y).min(image.height() - 1)))
    }

    /// Format dots to printer dots, rounded.
    fn printer_dots(&self, v: u32) -> u32 {
        let (num, den) = self.scale;
        ((v as u64 * num as u64 + den as u64 / 2) / den as u64) as u32
    }

    /// Like [`PngRenderer::render`], plus the area each instruction drew on (`None` when it
    /// drew nothing on the page).
    #[cfg(feature = "source-map")]
    pub fn render_with_bounds(&self, instructions: &[DrawInstruction]) -> (RgbImage, Vec<Option<Bounds>>) {
        let mut image: RgbImage = ImageBuffer::from_pixel(self.width, self.height, WHITE);
        let mut bounds = Vec::with_capacity(instructions.len());

        for instruction in instructions {
            self.drawn.set(None);
            self.render_instruction(&mut image, instruction);
            bounds.push(self.drawn.take().map(|[x0, y0, x1, y1]| Bounds {
                x: x0 as u32,
                y: y0 as u32,
                width: (x1 - x0) as u32,
                height: (y1 - y0) as u32,
            }));
        }

        // ^PO / ^PM move what each instruction drew along with the page.
        let (w, h) = (image.width(), image.height());
        for b in bounds.iter_mut().flatten() {
            if self.inverted != self.mirrored {
                b.x = w - b.x - b.width;
            }
            if self.inverted {
                b.y = h - b.y - b.height;
            }
        }
        self.orient(&mut image);

        if self.scale.0 != self.scale.1 {
            for b in bounds.iter_mut().flatten() {
                let (x1, y1) = (self.printer_dots(b.x + b.width), self.printer_dots(b.y + b.height));
                (b.x, b.y) = (self.printer_dots(b.x), self.printer_dots(b.y));
                (b.width, b.height) = (x1 - b.x, y1 - b.y);
            }
        }
        (self.to_printer_dots(image), bounds)
    }

    /// Records that the current instruction drew in the `width` x `height` area at (`x`, `y`).
    /// Compiles to nothing without the `source-map` feature.
    #[inline(always)]
    fn note_drawn(&self, x: i64, y: i64, width: u32, height: u32) {
        #[cfg(feature = "source-map")]
        {
            let x0 = x.max(0);
            let y0 = y.max(0);
            let x1 = (x + width as i64).min(self.width as i64);
            let y1 = (y + height as i64).min(self.height as i64);
            if x0 >= x1 || y0 >= y1 {
                return;
            }
            let area = match self.drawn.get() {
                Some([a, b, c, d]) => [a.min(x0), b.min(y0), c.max(x1), d.max(y1)],
                None => [x0, y0, x1, y1],
            };
            self.drawn.set(Some(area));
        }
        #[cfg(not(feature = "source-map"))]
        let _ = (x, y, width, height);
    }

    fn render_instruction(&self, image: &mut RgbImage, instruction: &DrawInstruction) {
        match instruction {
            DrawInstruction::DrawText {
                x, y, text, font, height, width, orientation, justification, anchor, block, direction, char_gap,
                custom_font, layout,
            } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([255, 0, 0]));
                }
                let name = *font;
                let font = match text.chars().next() {
                    Some(symbol) if name == FontName::Symbol => TextFont::symbol(*height, *width, symbol),
                    _ => TextFont::new(name, custom_font.as_ref(), *height, *width, self.dpmm),
                };
                // Control characters (from ^FH escapes such as _0D) print nothing.
                let text: String = text.chars().filter(|&c| c >= ' ' && c != '\x7f').collect();
                // Uppercase-only fonts: B capitalises, OCR-A (H) leaves lowercase blank (Labelary).
                let text = match (font.uppercase, name) {
                    (false, _) => text,
                    (true, FontName::FontH) => text.chars().map(|c| if c.is_lowercase() { ' ' } else { c }).collect(),
                    (true, _) => text.to_uppercase(),
                };
                // Bitmap fonts leave the characters they lack blank.
                let text = match font.cell {
                    Some(_) if custom_font.is_none() && name != FontName::Symbol && !text.is_ascii() => text
                        .chars()
                        .map(|c| if bitmap_prints(name, c) && font.has_glyph(c) { c } else { ' ' })
                        .collect(),
                    _ => text,
                };
                // ^FB: `\-` is a soft hyphen, which Labelary prints as a dash.
                let text = match block {
                    Some(b) if b.clip_height.is_none() => soft_hyphens(&text).into_owned(),
                    _ => text,
                };
                let text = match name {
                    FontName::Symbol => text.chars().map(|c| symbol_glyph(c).1).collect(),
                    _ => text,
                };
                let text = match (custom_font, name) {
                    (None, n) if n != FontName::Symbol => dashes(&text, font.has_glyph('\u{2212}')).into_owned(),
                    _ => text,
                };
                let placement = Placement { orientation: *orientation, justification: *justification, anchor: *anchor };
                let flow = (*direction, *char_gap);
                self.render_text(image, *x, *y, &text, &font, placement, block.as_ref(), flow, layout);
            }
            DrawInstruction::DrawBarcode {
                x, y, bottom, data, orientation, height, module_width,
                print_interpretation_line, print_interpretation_line_above, mode, ..
            } => {
                self.render_code128(
                    image, *x, *y, *bottom, data, *orientation, *height, *module_width,
                    *print_interpretation_line, *print_interpretation_line_above, *mode,
                );
            }
            DrawInstruction::DrawLinear {
                x, y, bottom, kind, data, orientation, height, module_width, wide_bar_ratio,
                print_interpretation_line, print_interpretation_line_above,
            } => {
                self.render_linear(
                    image, *x, *y, *bottom, *kind, data, *orientation, *height, *module_width, *wide_bar_ratio,
                    *print_interpretation_line, *print_interpretation_line_above,
                );
            }
            DrawInstruction::DrawQrCode { x, y, data, magnification, orientation, bottom, .. } => {
                self.render_qr_code(image, *x, *y, data, *magnification, *orientation, *bottom);
            }
            DrawInstruction::DrawDataMatrix {
                x, y, data, orientation, module_height, symbol_height, columns, rows, escape, rectangular, bottom,
            } => {
                if let Ok(m) = matrix::datamatrix(&symbol_bytes(data), *rows, *columns, *rectangular, *escape) {
                    // Without an explicit module size the ^BY height is the approximate symbol height.
                    let module = module_height.unwrap_or(symbol_height / m.height.max(1)).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, module, *bottom);
                }
            }
            DrawInstruction::DrawPdf417 {
                x, y, data, orientation, row_height, module_width, symbol_height, security, columns, rows,
                truncated, bottom,
            } => {
                if let Ok(m) = matrix::pdf417(&symbol_bytes(data), *security, *columns, *rows, *truncated) {
                    let module = (*module_width).max(1);
                    // Row height in dots, or the ^BY height shared by all rows.
                    let row = row_height.unwrap_or(symbol_height / m.height.max(1)).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, row, *bottom);
                }
            }
            DrawInstruction::DrawAztec { x, y, data, orientation, magnification, ecc, bottom } => {
                if let Ok(m) = matrix::aztec(&symbol_bytes(data), *ecc) {
                    let module = (*magnification).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, module, *bottom);
                }
            }
            DrawInstruction::DrawBox { x, y, width, height, thickness, color, .. } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 255, 0]));
                }
                self.render_box(image, *x, *y, *width, *height, *thickness, color);
            }
            DrawInstruction::DrawCircle { x, y, diameter, thickness, color } => {
                self.render_ellipse(image, *x, *y, *diameter, *diameter, *thickness, color);
            }
            DrawInstruction::DrawEllipse { x, y, width, height, thickness, color } => {
                self.render_ellipse(image, *x, *y, *width, *height, *thickness, color);
            }
            DrawInstruction::DrawDiagonalLine { x, y, width, height, thickness, color, direction } => {
                self.render_diagonal_line(image, *x, *y, *width, *height, *thickness, color, direction);
            }
            DrawInstruction::DrawGraphicField { x, y, bytes_per_row, data, magnification_x, magnification_y } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 0, 255]));
                }
                self.render_graphic_field(image, *x, *y, *bytes_per_row, data, *magnification_x, *magnification_y);
            }
            DrawInstruction::DrawSymbol { .. } => self.render_symbol(image, instruction),
            DrawInstruction::Reverse(inner) => self.render_reverse(image, inner),
        }
    }

    /// ^FR / ^LR: draw `inner` on a blank page, then invert every target pixel it blackened.
    fn render_reverse(&self, image: &mut RgbImage, inner: &DrawInstruction) {
        // No debug markers on the scratch page: they would be inverted too.
        let plain = PngRenderer { dpmm: self.dpmm, ..PngRenderer::new(image.width(), image.height()) };
        let mut scratch: RgbImage = ImageBuffer::from_pixel(image.width(), image.height(), WHITE);
        plain.render_instruction(&mut scratch, inner);
        #[cfg(feature = "source-map")]
        if let Some([x0, y0, x1, y1]) = plain.drawn.get() {
            self.note_drawn(x0, y0, (x1 - x0) as u32, (y1 - y0) as u32);
        }
        for (target, drawn) in image.pixels_mut().zip(scratch.pixels()) {
            let luma = (drawn[0] as u32 * 299 + drawn[1] as u32 * 587 + drawn[2] as u32 * 114) / 1000;
            if luma < 128 {
                *target = Rgb([255 - target[0], 255 - target[1], 255 - target[2]]);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_text(
        &self,
        image: &mut RgbImage,
        x: u32,
        y: u32,
        text: &str,
        font: &TextFont,
        placement: Placement,
        block: Option<&TextBlock>,
        (direction, char_gap): (FieldDirection, u32),
        text_layout: &TextLayout,
    ) {
        let Placement { orientation, justification, anchor } = placement;
        let height = font.line_h;
        // ^PA bidi/shaping: scalable fonts are laid out as glyph runs (with a fallback font
        // for Hebrew/Arabic); bitmap fonts keep their cells and are only reordered.
        let advanced = layout::active(text_layout, text);
        // Characters the font lacks (Triumvirate has no Cyrillic, little Greek) are drawn
        // from the fallback font, which takes the glyph-run path ^PA text takes.
        let runs = font.cell.is_none() && (advanced || font.lacks_glyph(text));
        let primary = runs.then(|| layout::LayoutFont { font: font.font.clone(), data: font.data.clone(), scale: font.scale });
        let measure = |s: &str| match &primary {
            Some(p) => layout::layout_line(p, font.cap_em, text_layout, s).width,
            None => font.measure(s),
        };
        let display = |s: String| if advanced && font.cell.is_some() { layout::visual_string(text_layout, &s) } else { s };

        // The unrotated text box: as wide as the text (or the ^FB block), one line height per
        // row, with the top of the capitals at the top of each row.
        let mut rows: Vec<(i32, i32, String)> = Vec::new();
        // ^FPR: how far the text is moved back along its direction. Labelary ends the line
        // (its first character, drawn last on the right) an 'M' minus the gap after the
        // origin, so a bitmap font's first cell starts at the origin.
        let mut back = 0.0;
        // ^FB: the block is as tall as its maximum number of lines; rows left empty below
        // the text still count when placing it (rotated, or by ^FT).
        let mut unused = 0i64;
        let per_char = !advanced && (char_gap > 0 || direction != FieldDirection::Horizontal);
        let (box_w, box_h, last_row_top) = match block {
            None if per_char && direction == FieldDirection::Vertical => {
                // ^FPV: one character per line, left-aligned; Labelary ignores the gap here.
                let mut widest = 0.0f32;
                for (i, c) in text.chars().enumerate() {
                    let c = c.to_string();
                    widest = widest.max(measure(&c));
                    rows.push((0, (i as u32 * height) as i32, c));
                }
                let n = rows.len().max(1) as u32;
                (widest.ceil() as u32, n * height, ((n - 1) * height) as i32)
            }
            None if per_char => {
                // ^FPH / ^FPR with a gap between the characters, drawn one by one.
                let gap = char_gap as f32;
                let mut pen = 0.0f32;
                let chars: Vec<char> = match direction {
                    FieldDirection::Reverse => text.chars().rev().collect(),
                    _ => text.chars().collect(),
                };
                for (i, c) in chars.into_iter().enumerate() {
                    let c = c.to_string();
                    if i > 0 {
                        pen += gap;
                    }
                    let advance = measure(&c);
                    rows.push((pen.round() as i32, 0, c));
                    pen += advance;
                }
                if direction == FieldDirection::Reverse {
                    back = pen - (measure("M") - gap);
                }
                (pen.ceil() as u32, height, 0)
            }
            None => {
                rows.push((0, 0, display(text.to_string())));
                (measure(text).ceil() as u32, height, 0)
            }
            Some(b) => {
                let line_height = (height as i32 + b.line_spacing).max(1);
                // ^TB lines are further apart than ^FB ones (Labelary).
                let pitch = font.text_block_pitch();
                let row_top = |row: usize| match b.clip_height {
                    Some(_) => (row as f32 * pitch).round() as i32,
                    None => row as i32 * line_height,
                };
                let mut lines = wrap_lines_with(&measure, text, b.width, b.hanging_indent);
                if let Some(clip) = b.clip_height {
                    // ^TB: lines starting below the block are dropped; the last one is cut
                    // off at the block's bottom edge.
                    let fit = (0..lines.len()).take_while(|&i| row_top(i) < clip as i32).count().max(1);
                    lines.truncate(fit);
                }
                let max_rows = b.max_lines.max(1) as usize;
                let mut last_row = 0;
                let space = measure(" ");
                let last = lines.len().saturating_sub(1);
                for (i, (line, wrapped)) in lines.into_iter().enumerate() {
                    // ^FB: lines beyond the maximum overprint the last one.
                    let row = i.min(max_rows - 1);
                    last_row = last_row.max(row);
                    let top = row_top(row);
                    let indent = if i > 0 { b.hanging_indent as f32 } else { 0.0 };
                    let room = b.width as f32 - indent;
                    let line_w = measure(&line);
                    // Labelary justifies wrapped lines as if they still ended in the space
                    // they broke at, and centers the last line as if it ended in one too.
                    let free = match b.justification {
                        BlockJustification::Center if wrapped || i == last => room - line_w - space,
                        BlockJustification::Right if wrapped => room - line_w - space,
                        _ => room - line_w,
                    }
                    .max(0.0);
                    let gaps = line.matches(' ').count();
                    if b.justification == BlockJustification::Justified && wrapped && gaps > 0 && !advanced {
                        // Words placed one by one with the slack spread over the spaces.
                        let extra = free / gaps as f32;
                        let mut pen = indent;
                        for (k, word) in line.split(' ').enumerate() {
                            if k > 0 {
                                pen += space + extra;
                            }
                            rows.push((pen.round() as i32, top, word.to_string()));
                            pen += measure(word);
                        }
                        continue;
                    }
                    let x_off = match b.justification {
                        BlockJustification::Left | BlockJustification::Justified => indent,
                        BlockJustification::Center => indent + free / 2.0,
                        BlockJustification::Right => indent + free,
                    };
                    rows.push((x_off.round() as i32, top, display(line)));
                }
                let last_top = row_top(last_row);
                if b.clip_height.is_none() {
                    unused = (max_rows - 1 - last_row) as i64 * line_height as i64;
                }
                let box_h = b.clip_height.unwrap_or(last_top as u32 + height);
                (b.width.max(1), box_h, last_top)
            }
        };
        if box_w == 0 || box_h == 0 {
            return;
        }

        // Drawn with a margin so accents and descenders outside the box are kept.
        let margin = height / 2 + 2;
        let mut temp: RgbImage = ImageBuffer::from_pixel(box_w + 2 * margin, box_h + 2 * margin, WHITE);
        for (x_off, top, line) in &rows {
            let (x_off, top) = (x_off + margin as i32, top + margin as i32);
            match &primary {
                Some(p) => {
                    use ab_glyph::{Font, ScaleFont};
                    let laid = layout::layout_line(p, font.cap_em, text_layout, line);
                    let baseline = (top - font.y_offset) as f32 + font.font.as_scaled(font.scale).ascent();
                    layout::draw_line(&mut temp, p, &laid, x_off as f32, baseline);
                }
                None => font.draw(&mut temp, x_off, top, line),
            }
        }
        if let Some(clip) = block.and_then(|b| b.clip_height) {
            let bottom = (margin + clip).min(temp.height());
            for y in bottom..temp.height() {
                for x in 0..temp.width() {
                    temp.put_pixel(x, y, WHITE);
                }
            }
        }

        // ^FO: the rotated box's top-left is the origin. ^FT: the start of the last line's
        // baseline is, so find where that point lands in the rotated box.
        let (w, h) = (box_w as i64, box_h as i64 + unused);
        let baseline = (last_row_top + font.cap) as i64 + unused;
        let (rotated_w, (px, py)) = match orientation {
            Orientation::Normal => (w, (0, baseline)),
            Orientation::Rotated90 => (h, (h - baseline, 0)),
            Orientation::Rotated180 => (w, (w, h - baseline)),
            Orientation::Rotated270 => (h, (baseline, w)),
        };
        let (mut x, mut y) = match anchor {
            TextAnchor::Top => (x as i64, y as i64),
            TextAnchor::Baseline => (x as i64 - px, y as i64 - py),
        };
        // Labelary moves ^FPR text back only where that is left or up on the page.
        let back = back.round() as i64;
        match orientation {
            Orientation::Normal => x -= back,
            Orientation::Rotated90 => y -= back,
            Orientation::Rotated180 | Orientation::Rotated270 => {}
        }
        // ^FO / ^FT right justification moves the rotated box left on the page; auto is
        // left for left-to-right text (Labelary).
        x -= match justification {
            Justification::Left | Justification::Auto => 0,
            Justification::Right => rotated_w,
        };
        // Only the rows with text were drawn: where they sit in the full rotated box.
        match orientation {
            Orientation::Rotated90 => x += unused,
            Orientation::Rotated180 => y += unused,
            Orientation::Normal | Orientation::Rotated270 => {}
        }

        let rotated = match orientation {
            Orientation::Normal => temp,
            Orientation::Rotated90 => image::imageops::rotate90(&temp),
            Orientation::Rotated180 => image::imageops::rotate180(&temp),
            Orientation::Rotated270 => image::imageops::rotate270(&temp),
        };
        if self.debug && x >= 0 && y >= 0 {
            let (bw, bh) = (rotated.width() - 2 * margin, rotated.height() - 2 * margin);
            self.draw_debug_box(image, x as u32, y as u32, bw, bh, Rgb([255, 0, 0]));
        }
        self.overlay_at(image, &rotated, x - margin as i64, y - margin as i64);
    }

    fn overlay_image(&self, target: &mut RgbImage, source: &RgbImage, x: u32, y: u32) {
        self.overlay_at(target, source, x as i64, y as i64);
    }

    /// Copies the non-white pixels of `source` onto `target` at (`x`, `y`), clipping at the edges.
    fn overlay_at(&self, target: &mut RgbImage, source: &RgbImage, x: i64, y: i64) {
        // Ink box of what was copied: text and symbol images carry white padding.
        #[cfg(feature = "source-map")]
        let mut ink = [i64::MAX, i64::MAX, i64::MIN, i64::MIN];
        for (src_x, src_y, pixel) in source.enumerate_pixels() {
            let target_x = x + src_x as i64;
            let target_y = y + src_y as i64;

            if target_x >= 0
                && target_y >= 0
                && target_x < target.width() as i64
                && target_y < target.height() as i64
                && (pixel[0] < 250 || pixel[1] < 250 || pixel[2] < 250)
            {
                target.put_pixel(target_x as u32, target_y as u32, *pixel);
                #[cfg(feature = "source-map")]
                {
                    ink = [ink[0].min(target_x), ink[1].min(target_y), ink[2].max(target_x), ink[3].max(target_y)];
                }
            }
        }
        #[cfg(feature = "source-map")]
        if ink[0] <= ink[2] {
            self.note_drawn(ink[0], ink[1], (ink[2] - ink[0] + 1) as u32, (ink[3] - ink[1] + 1) as u32);
        }
    }

    /// Places the unrotated image of a symbol whose bars are the `bars_w` x `bars_h` box at
    /// (`bars_x`, `bars_y`): ^FO puts the rotated bars' top-left on (`x`, `y`), ^FT (`bottom`)
    /// the bars' bottom-left corner, rotated with the symbol. The interpretation line follows
    /// the bars wherever it lands. 2D symbols are all bars.
    #[allow(clippy::too_many_arguments)]
    fn place_bars(
        &self,
        image: &mut RgbImage,
        temp: &RgbImage,
        x: u32,
        y: u32,
        bottom: bool,
        orientation: Orientation,
        (bars_x, bars_y, bars_w, bars_h): (u32, u32, u32, u32),
    ) {
        let (x, y) = (x as i64, y as i64);
        let (bx, by, bw, bh) = (bars_x as i64, bars_y as i64, bars_w as i64, bars_h as i64);
        let (tw, th) = (temp.width() as i64, temp.height() as i64);
        // Top-left of the rotated bars on the page.
        let (left, top) = match (bottom, orientation) {
            (false, _) | (true, Orientation::Rotated90) => (x, y),
            (true, Orientation::Normal) => (x, y - bh),
            (true, Orientation::Rotated180) => (x - bw, y),
            (true, Orientation::Rotated270) => (x - bh, y - bw),
        };
        match orientation {
            Orientation::Normal => self.overlay_at(image, temp, left - bx, top - by),
            Orientation::Rotated90 => {
                let rotated = image::imageops::rotate90(temp);
                self.overlay_at(image, &rotated, left - (th - by - bh), top - bx);
            }
            Orientation::Rotated180 => {
                let rotated = image::imageops::rotate180(temp);
                self.overlay_at(image, &rotated, left - (tw - bx - bw), top - (th - by - bh));
            }
            Orientation::Rotated270 => {
                let rotated = image::imageops::rotate270(temp);
                self.overlay_at(image, &rotated, left - by, top - (tw - bx - bw));
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_code128(&self, image: &mut RgbImage, x: u32, y: u32, bottom: bool, data: &str, orientation: Orientation, height: u32, module_width: u32, print_interpretation_line: bool, print_interpretation_line_above: bool, mode: Code128Mode) {
        let code = code128::encode(data, mode);
        let bar_width = module_width.max(1);
        let widths = code.widths();
        let barcode_width: u32 = widths.iter().map(|&w| w as u32 * bar_width).sum();

        let ip = Interpretation::new(bar_width);
        let text_gap = line_gap(bar_width, print_interpretation_line_above, false);

        let show_text = print_interpretation_line && !code.text.is_empty();
        // Mode D sizes its line in font 0 to the bars (see `code128::Gs1Line`).
        let gs1 = (show_text && mode == Code128Mode::Gs1)
            .then(|| code128::Gs1Line::new(&code.text, barcode_width, height, print_interpretation_line_above))
            .flatten();
        let text_width = match &gs1 {
            Some(_) => barcode_width,
            None if show_text => ip.text_width(&code.text).ceil() as u32,
            None => 0,
        };
        let text_area = match &gs1 {
            Some(line) => line.area,
            None if show_text => ip.area(text_gap, print_interpretation_line_above),
            None => 0,
        };

        let pad = text_width.saturating_sub(barcode_width) / 2;
        let total_width = barcode_width + 2 * pad;
        let total_height = height + text_area;
        let (bars_y, baseline) = match &gs1 {
            Some(line) => (line.bars_y, line.baseline),
            None if print_interpretation_line_above => (text_area, ip.digit_height),
            None => (0, height as f32 + text_gap + ip.digit_height),
        };

        let mut temp_image: RgbImage = ImageBuffer::from_pixel(total_width.max(1), total_height.max(1), WHITE);

        let mut x_pos = pad;
        for (i, &width) in widths.iter().enumerate() {
            let pixel_width = width as u32 * bar_width;
            if i % 2 == 0 {
                for px in x_pos..x_pos + pixel_width {
                    for py in bars_y..bars_y + height {
                        temp_image.put_pixel(px, py, BLACK);
                    }
                }
            }
            x_pos += pixel_width;
        }

        if let Some(line) = &gs1 {
            use ab_glyph::{Font, ScaleFont};
            let font = FontRef::try_from_slice(TRIUMVIRATE).expect("Error loading font");
            let scale = PxScale::from(line.em * font.height_unscaled() / font.units_per_em().unwrap_or(1000.0));
            let top = baseline - font.as_scaled(scale).ascent();
            drawing::draw_text_mut(&mut temp_image, BLACK, line.x.round() as i32, top.round() as i32, scale, &font, &code.text);
        } else if show_text {
            let text_x = pad as f32 + (barcode_width as f32 - text_width as f32) / 2.0;
            ip.draw(&mut temp_image, &code.text, text_x, baseline);
        }

        self.place_bars(image, &temp_image, x, y, bottom, orientation, (pad, bars_y, barcode_width, height));
    }

    /// All 1D symbologies except Code 128, placed by their bars (see `place_bars`); the
    /// interpretation line goes below, or above when requested. EAN/UPC guard bars extend
    /// halfway into the digits.
    #[allow(clippy::too_many_arguments)]
    fn render_linear(
        &self,
        image: &mut RgbImage,
        x: u32,
        y: u32,
        bottom: bool,
        kind: LinearKind,
        data: &str,
        orientation: Orientation,
        height: u32,
        module_width: u32,
        ratio: f32,
        print_line: bool,
        line_above: bool,
    ) {
        let Some(symbol) = encode_linear(kind, data) else { return };
        let module = module_width.max(1);
        let width = symbol.width_dots(module, ratio);

        let ip = Interpretation::new(module);
        // The UPC/EAN add-on keeps its digits close above the bars.
        let add_on = matches!(kind, LinearKind::UpcEanExtension);
        let text_gap = if add_on { (2 * module) as f32 } else { line_gap(module, line_above, symbol.has_guards) };
        let show_text = print_line && !symbol.text.is_empty();
        let text_area = if show_text { ip.area(text_gap, line_above && !add_on) } else { 0 };
        let guard_ext = match (symbol.has_guards && show_text, line_above) {
            (false, _) => 0,
            (true, true) => GUARD_EXTENSION_ABOVE,
            (true, false) => (text_gap + ip.digit_height / 2.0).round() as u32,
        };
        let bars_y = if line_above { text_area } else { 0 };

        // Text segments (e.g. the EAN-13 leading digit) may stick out of the bars. Above
        // the bars EAN/UPC digits are one line centred over the symbol.
        let mut segments: Vec<(f32, std::borrow::Cow<str>)> = Vec::new();
        let (mut left, mut right) = (0.0f32, width as f32);
        if show_text {
            if line_above && symbol.has_guards {
                let text: String = symbol.text.iter().map(|seg| seg.text.as_str()).collect();
                segments.push(((width as f32 - ip.text_width(&text)) / 2.0, text.into()));
            } else {
                for seg in &symbol.text {
                    let (start, end) = symbol.text_extent_dots(seg, module, ratio);
                    segments.push(((start + end - ip.text_width(&seg.text)) / 2.0, seg.text.as_str().into()));
                }
            }
            for (text_x, text) in &segments {
                left = left.min(*text_x);
                right = right.max(text_x + ip.text_width(text));
            }
        }
        let pad_l = (-left).ceil() as u32;
        let pad_r = (right - width as f32).ceil() as u32;

        let total_w = (pad_l + width + pad_r).max(1);
        // Guard bars below a line printed above stick out under the others.
        let below = if line_above { guard_ext } else { 0 };
        let total_h = (height + text_area + below).max(1);
        let mut temp: RgbImage = ImageBuffer::from_pixel(total_w, total_h, WHITE);

        let mut x_pos = pad_l;
        for (element, dots) in symbol.element_dots(module, ratio) {
            if element.bar {
                let bar_h = height + if element.guard { guard_ext } else { 0 };
                for px in x_pos..(x_pos + dots).min(total_w) {
                    for py in bars_y..(bars_y + bar_h).min(total_h) {
                        temp.put_pixel(px, py, BLACK);
                    }
                }
            }
            x_pos += dots;
        }

        if show_text {
            let baseline = if line_above { ip.digit_height } else { height as f32 + text_gap + ip.digit_height };
            for (text_x, text) in &segments {
                ip.draw(&mut temp, text, pad_l as f32 + text_x, baseline);
            }
        }

        self.place_bars(image, &temp, x, y, bottom, orientation, (pad_l, bars_y, width, height));
    }

    /// 2D symbol with `module_w` x `module_h` dot modules. Like the bars of a 1D symbol (and
    /// as Labelary does), ^FO puts the rotated symbol's top-left on the origin and ^FT
    /// (`bottom`) the unrotated symbol's bottom-left corner, turned with the symbol.
    #[allow(clippy::too_many_arguments)]
    fn render_matrix(
        &self,
        image: &mut RgbImage,
        m: &Matrix,
        x: u32,
        y: u32,
        orientation: Orientation,
        module_w: u32,
        module_h: u32,
        bottom: bool,
    ) {
        let (w, h) = (m.width * module_w, m.height * module_h);
        if w == 0 || h == 0 {
            return;
        }
        let mut temp: RgbImage = ImageBuffer::from_pixel(w, h, WHITE);
        for row in 0..m.height {
            for col in 0..m.width {
                if m.get(col, row) {
                    for py in row * module_h..(row + 1) * module_h {
                        for px in col * module_w..(col + 1) * module_w {
                            temp.put_pixel(px, py, BLACK);
                        }
                    }
                }
            }
        }
        self.place_bars(image, &temp, x, y, bottom, orientation, (0, 0, w, h));
    }

    /// ^BQ: always upright; the orientation only moves it (see [`qr::top_left`]).
    #[allow(clippy::too_many_arguments)]
    fn render_qr_code(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, magnification: u32, orientation: Orientation, bottom: bool) {
        let Some(code) = qr::encode(data) else { return };
        let module = magnification.max(1);
        let side = code.size * module;
        let (x, y) = qr::top_left(x, y, code.size, module, orientation, bottom);
        self.note_drawn(x as i64, y as i64, side, side);
        for row in 0..code.size {
            for col in 0..code.size {
                if !code.get(col, row) {
                    continue;
                }
                let (px, py) = (x + col * module, y + row * module);
                for ty in py..(py + module).min(image.height()) {
                    for tx in px..(px + module).min(image.width()) {
                        image.put_pixel(tx, ty, BLACK);
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_box(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor) {
        self.note_drawn(x as i64, y as i64, width, height);
        let rgb_color = self.line_color_to_rgb(color);

        for t in 0..thickness {
            let new_width = width.saturating_sub(2 * t);
            let new_height = height.saturating_sub(2 * t);

            if new_width == 0 || new_height == 0 {
                break;
            }

            let rect = imageproc::rect::Rect::at(x as i32 + t as i32, y as i32 + t as i32)
                .of_size(new_width, new_height);
            drawing::draw_hollow_rect_mut(image, rect, rgb_color);
        }
    }

    /// Ring of `thickness` dots inside the `width` x `height` box; filled when the thickness
    /// reaches half the smaller side. Also used for circles.
    #[allow(clippy::too_many_arguments)]
    fn render_ellipse(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor) {
        if width == 0 || height == 0 {
            return;
        }
        self.note_drawn(x as i64, y as i64, width, height);
        let rgb_color = self.line_color_to_rgb(color);
        let (a, b) = (width as f64 / 2.0, height as f64 / 2.0);
        let (cx, cy) = (x as f64 + a, y as f64 + b);
        let t = thickness as f64;
        let (ia, ib) = (a - t, b - t);
        let filled = ia <= 0.0 || ib <= 0.0;

        for py in y..(y + height).min(image.height()) {
            let dy = py as f64 + 0.5 - cy;
            for px in x..(x + width).min(image.width()) {
                let dx = px as f64 + 0.5 - cx;
                let outer = (dx / a).powi(2) + (dy / b).powi(2) <= 1.0;
                if !outer {
                    continue;
                }
                let inner = !filled && (dx / ia).powi(2) + (dy / ib).powi(2) < 1.0;
                if !inner {
                    image.put_pixel(px, py, rgb_color);
                }
            }
        }
    }

    /// ^GD: a parallelogram whose left edge runs corner to corner of the `width` x `height`
    /// box, `thickness` dots wide on every row (so it sticks out `thickness` dots on the
    /// right), as Labelary draws it: each row starts at the first dot right of the edge
    /// where it crosses the row's top.
    #[allow(clippy::too_many_arguments)]
    fn render_diagonal_line(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor, direction: &DiagonalDirection) {
        if thickness == 0 {
            return;
        }
        self.note_drawn(x as i64, y as i64, width + thickness, height.max(1));
        let rgb_color = self.line_color_to_rgb(color);
        let h = height.max(1) as f32;
        let slope = width as f32 / h;
        for row in 0..height.max(1) {
            let py = y + row;
            if py >= image.height() {
                break;
            }
            let dy = row as f32;
            let left = x as f32
                + match direction {
                    DiagonalDirection::LeftToRight => dy * slope,
                    DiagonalDirection::RightToLeft => width as f32 - dy * slope,
                };
            let from = left.ceil().max(0.0) as u32;
            for px in from..(from + thickness).min(image.width()) {
                image.put_pixel(px, py, rgb_color);
            }
        }
    }

    /// 1-bit bitmap, MSB first; each source dot becomes `mag_x` x `mag_y` pixels.
    #[allow(clippy::too_many_arguments)]
    fn render_graphic_field(&self, image: &mut RgbImage, x: u32, y: u32, bytes_per_row: u32, data: &[u8], mag_x: u32, mag_y: u32) {
        let (mag_x, mag_y) = (mag_x.max(1), mag_y.max(1));
        let width = bytes_per_row * 8;
        let height = (data.len() as u32).checked_div(bytes_per_row).unwrap_or(0);

        if width == 0 || height == 0 {
            return;
        }

        let mut temp_image: RgbImage = ImageBuffer::from_pixel(width * mag_x, height * mag_y, WHITE);

        for (row, row_bytes) in data.chunks(bytes_per_row as usize).take(height as usize).enumerate() {
            for (byte_offset, byte) in row_bytes.iter().enumerate() {
                for bit in 0..8 {
                    if byte & (1 << (7 - bit)) != 0 {
                        let src_x = (byte_offset * 8 + bit) as u32;
                        for py in row as u32 * mag_y..(row as u32 + 1) * mag_y {
                            for px in src_x * mag_x..(src_x + 1) * mag_x {
                                temp_image.put_pixel(px, py, BLACK);
                            }
                        }
                    }
                }
            }
        }

        if self.debug {
            self.draw_debug_box(image, x, y, temp_image.width(), temp_image.height(), Rgb([0, 0, 255]));
        }

        self.overlay_image(image, &temp_image, x, y);
    }

    fn line_color_to_rgb(&self, color: &LineColor) -> Rgb<u8> {
        match color {
            LineColor::Black => BLACK,
            LineColor::White => WHITE,
        }
    }

    fn draw_origin_cross(&self, image: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
        let size = 10;
        // Draw horizontal line
        for dx in 0..size {
            let px = x.saturating_sub(size / 2) + dx;
            if px < image.width() && y < image.height() {
                image.put_pixel(px, y, color);
            }
        }
        // Draw vertical line
        for dy in 0..size {
            let py = y.saturating_sub(size / 2) + dy;
            if x < image.width() && py < image.height() {
                image.put_pixel(x, py, color);
            }
        }
    }

    fn draw_debug_box(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, color: Rgb<u8>) {
        let rect = imageproc::rect::Rect::at(x as i32, y as i32).of_size(width, height);
        drawing::draw_hollow_rect_mut(image, rect, color);
    }
}

/// A page rendered on media of a given size, the way Labelary lays it out: the page keeps
/// its ^PW width, centred on wider media (the offset rounded up) and clipped by narrower
/// media at the right; it is cropped or padded with white at the bottom. ^POI and ^PMY
/// apply to the page, before it is placed.
pub fn place_on_media(page: RgbImage, width: u32, height: u32) -> RgbImage {
    if page.dimensions() == (width, height) {
        return page;
    }
    let offset = width.saturating_sub(page.width()).div_ceil(2);
    let mut media: RgbImage = ImageBuffer::from_pixel(width, height, WHITE);
    image::imageops::replace(&mut media, &page, offset as i64, 0);
    media
}

pub fn save_png(image: &RgbImage, path: &str) -> Result<(), image::ImageError> {
    image.save(path)
}

/// Encodes `image` in memory in the format implied by `path`'s extension, like
/// [`save_png`] does, without writing it to disk.
pub fn encode_image(image: &RgbImage, path: &str) -> Result<Vec<u8>, image::ImageError> {
    encode_as(image, image::ImageFormat::from_path(path)?)
}

/// Encodes `image` as PNG in memory.
pub fn encode_png(image: &RgbImage) -> Result<Vec<u8>, image::ImageError> {
    encode_as(image, image::ImageFormat::Png)
}

fn encode_as(image: &RgbImage, format: image::ImageFormat) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut bytes), format)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn black(image: &RgbImage, x: u32, y: u32) -> bool {
        image.get_pixel(x, y)[0] < 128
    }

    #[test]
    fn reverse_inverts_underlying_pixels() {
        let solid = DrawInstruction::DrawBox {
            x: 0, y: 0, width: 10, height: 10, thickness: 10, color: LineColor::Black, rounding: 0,
        };
        let reversed = DrawInstruction::Reverse(Box::new(DrawInstruction::DrawBox {
            x: 5, y: 0, width: 10, height: 10, thickness: 10, color: LineColor::Black, rounding: 0,
        }));
        let image = PngRenderer::new(20, 10).render(&[solid, reversed]);
        assert!(black(&image, 2, 5));
        assert!(!black(&image, 7, 5));
        assert!(black(&image, 12, 5));
        assert!(!black(&image, 17, 5));
    }

    #[test]
    fn ellipse_ring_and_fill() {
        let ring = DrawInstruction::DrawEllipse {
            x: 0, y: 0, width: 40, height: 20, thickness: 2, color: LineColor::Black,
        };
        let image = PngRenderer::new(40, 20).render(&[ring]);
        assert!(black(&image, 0, 10));
        assert!(!black(&image, 20, 10));
        let filled = DrawInstruction::DrawEllipse {
            x: 0, y: 0, width: 40, height: 20, thickness: 10, color: LineColor::Black,
        };
        let image = PngRenderer::new(40, 20).render(&[filled]);
        assert!(black(&image, 20, 10));
    }

    #[test]
    fn diagonal_thickness_is_horizontal_and_leans_like_labelary() {
        let gd = |direction| DrawInstruction::DrawDiagonalLine {
            x: 10, y: 10, width: 100, height: 50, thickness: 6, color: LineColor::Black, direction,
        };
        // L leans left (\): starts top-left, ends bottom-right, sticking out by the thickness.
        let image = PngRenderer::new(200, 100).render(&[gd(DiagonalDirection::LeftToRight)]);
        let row = |y| (0..200).filter(|&x| black(&image, x, y)).collect::<Vec<_>>();
        assert_eq!(row(10), (10..16).collect::<Vec<_>>());
        assert_eq!(row(59), (108..114).collect::<Vec<_>>());
        assert!(row(60).is_empty());
        // R leans right (/).
        let image = PngRenderer::new(200, 100).render(&[gd(DiagonalDirection::RightToLeft)]);
        let row = |y| (0..200).filter(|&x| black(&image, x, y)).collect::<Vec<_>>();
        assert_eq!(row(10), (110..116).collect::<Vec<_>>());
        assert_eq!(row(59), (12..18).collect::<Vec<_>>());
    }

    #[test]
    fn inverted_and_mirrored_pages() {
        let gb = DrawInstruction::DrawBox { x: 0, y: 0, width: 4, height: 2, thickness: 2, color: LineColor::Black, rounding: 0 };
        let page = |inverted, mirrored| {
            let setup = PageSetup { width: 10, height: 6, inverted, mirrored, ..PageSetup::default() };
            PngRenderer::for_page(&setup, false).render(std::slice::from_ref(&gb))
        };
        let corner = |image: &RgbImage| (black(image, 0, 0), black(image, 9, 0), black(image, 0, 5), black(image, 9, 5));
        assert_eq!(corner(&page(false, false)), (true, false, false, false));
        assert_eq!(corner(&page(true, false)), (false, false, false, true));
        assert_eq!(corner(&page(false, true)), (false, true, false, false));
        assert_eq!(corner(&page(true, true)), (false, false, true, false));
    }

    #[test]
    fn narrow_page_is_centred_on_media() {
        let page: RgbImage = ImageBuffer::from_pixel(401, 3, BLACK);
        let media = place_on_media(page, 812, 6);
        // (812 - 401) / 2 rounded up.
        assert!(!black(&media, 205, 0) && black(&media, 206, 0) && black(&media, 606, 2) && !black(&media, 607, 0));
        assert!(!black(&media, 300, 3));
        // A page wider than the media is clipped on the right.
        let media = place_on_media(ImageBuffer::from_pixel(1000, 3, BLACK), 812, 3);
        assert!(black(&media, 0, 0) && black(&media, 811, 0));
    }

    #[test]
    fn graphic_field_magnification() {
        let gf = DrawInstruction::DrawGraphicField {
            x: 0, y: 0, bytes_per_row: 1, data: vec![0x80], magnification_x: 3, magnification_y: 2,
        };
        let image = PngRenderer::new(10, 10).render(&[gf]);
        assert!(black(&image, 2, 1));
        assert!(!black(&image, 3, 1));
        assert!(!black(&image, 2, 2));
    }

    #[test]
    fn linear_ean13_draws_bars() {
        let ean = DrawInstruction::DrawLinear {
            x: 20, y: 0, bottom: false, kind: LinearKind::Ean13, data: "590123412345".into(),
            orientation: Orientation::Normal, height: 50, module_width: 2, wide_bar_ratio: 3.0,
            print_interpretation_line: true, print_interpretation_line_above: false,
        };
        let image = PngRenderer::new(300, 100).render(&[ean]);
        // Start guard: bar at module 0, space at module 1.
        assert!(black(&image, 20, 10));
        assert!(!black(&image, 22, 10));
        // Leading digit sits left of the bars.
        assert!((0..20).any(|x| (50..80).any(|y| black(&image, x, y))));
    }

    /// Rightmost dark column of `zpl` rendered on a 400 x 100 page, for every font kind.
    fn ink_right(zpl_field: &str) -> Vec<Option<u32>> {
        ["^A0N,40,40", "^ADN,36,20", "^AEN,28,15"]
            .iter()
            .map(|font| {
                let zpl = format!("^XA^PW400^LL100^FO10,10{font}{zpl_field}^XZ");
                let instructions =
                    zpl_state_manager::StateManager::new().process(zpl_parser::Parser::new(&zpl).parse().unwrap());
                let image = PngRenderer::new(400, 100).render(&instructions);
                (0..400).rev().find(|&x| (0..100).any(|y| image.get_pixel(x, y)[0] < 128))
            })
            .collect()
    }

    #[test]
    fn line_breaks_and_control_characters_print_nothing() {
        let plain = ink_right("^FDVAN:^FS");
        assert!(plain.iter().all(Option::is_some));
        assert_eq!(ink_right("^FDVAN:\n^FS"), plain);
        assert_eq!(ink_right("^FDVAN:\r\n^FS"), plain);
        // ^FH escapes survive the parser; the renderer skips them.
        assert_eq!(ink_right("^FH^FDVAN:_0D_0A_09_7F^FS"), plain);
        assert_eq!(ink_right("^FDab\r\ncd^FS"), ink_right("^FDabcd^FS"));
    }

    #[cfg(feature = "source-map")]
    #[test]
    fn bounds_cover_each_instruction() {
        use zpl_state_manager::StateManager;
        let zpl = "^XA^PW400^LL300^FO10,20^GB100,50,3^FS^FO350,250^GB100,100,100^FS\
                   ^FO50,100^A0N,40,40^FDHi^FS^FO200,10^BQN,2,3^FDQA,x^FS^FO20,200^FR^GB30,30,30^FS^XZ";
        let mut state = StateManager::new();
        let instructions = state.process(zpl_parser::Parser::new(zpl).parse().unwrap());
        let (_, bounds) = PngRenderer::new(400, 300).render_with_bounds(&instructions);
        assert_eq!(bounds.len(), 5);
        assert_eq!(bounds[0], Some(Bounds { x: 10, y: 20, width: 100, height: 50 }));
        // Clipped at the page edge.
        assert_eq!(bounds[1], Some(Bounds { x: 350, y: 250, width: 50, height: 50 }));
        let text = bounds[2].unwrap();
        assert!(text.x >= 40 && text.x <= 60 && text.y >= 90 && text.y <= 110 && text.height >= 20, "{text:?}");
        let qr = bounds[3].unwrap();
        assert_eq!((qr.x, qr.y), (200, 20));
        assert_eq!(bounds[4], Some(Bounds { x: 20, y: 200, width: 30, height: 30 }));
    }

    #[test]
    fn page_scale_enlarges_to_printer_dots() {
        let setup = PageSetup { width: 10, height: 6, scale: (2, 1), ..PageSetup::default() };
        let gb = DrawInstruction::DrawBox { x: 2, y: 1, width: 3, height: 2, thickness: 2, color: LineColor::Black, rounding: 0 };
        let image = PngRenderer::for_page(&setup, false).render(&[gb]);
        assert_eq!(image.dimensions(), (20, 12));
        let black = |x, y| image.get_pixel(x, y)[0] == 0;
        assert!(black(4, 2) && black(9, 5) && !black(3, 2) && !black(10, 5) && !black(4, 6));
    }

    #[test]
    fn bitmap_fonts_e_and_h_follow_resolution() {
        assert_eq!(bitmap_cell(FontName::FontE, 6).unwrap().height, 21);
        assert_eq!(bitmap_cell(FontName::FontE, 8).unwrap().height, 28);
        assert_eq!(bitmap_cell(FontName::FontE, 24).unwrap().height, 42);
        assert_eq!(bitmap_cell(FontName::FontH, 12).unwrap().width, 22);
        assert_eq!(bitmap_cell(FontName::FontA, 24), bitmap_cell(FontName::FontA, 8));
    }

    #[test]
    fn wrap_respects_width_and_forced_breaks() {
        let font = TextFont::new(FontName::Font0, None, 30, 30, 8);
        let lines = wrap_lines(&font, "aaa bbb ccc\\&ddd", font.measure("aaa bbb") as u32 + 1, 0);
        assert_eq!(lines, vec!["aaa bbb", "ccc", "ddd"]);
    }

    /// Ink bounding box (x0, y0, x1, y1), inclusive, of a 4x6 inch label at 8 dots/mm,
    /// within `area` (x0, y0, x1, y1, exclusive ends).
    fn ink_box(zpl: &str, area: (u32, u32, u32, u32)) -> Option<(u32, u32, u32, u32)> {
        use zpl_state_manager::StateManager;
        let instructions = StateManager::new().process(zpl_parser::Parser::new(zpl).parse().unwrap());
        let image = PngRenderer::new(812, 1218).render(&instructions);
        let mut found: Option<(u32, u32, u32, u32)> = None;
        for y in area.1..area.3 {
            for x in area.0..area.2 {
                if black(&image, x, y) {
                    let b = found.get_or_insert((x, y, x, y));
                    *b = (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y));
                }
            }
        }
        found
    }

    /// Each field alone on a label; the box is Labelary's (8dpmm, 4x6), within 3 dots.
    fn assert_matches_labelary(cases: &[(&str, (u32, u32, u32, u32))]) {
        for (field, expected) in cases {
            let got = ink_box(&format!("^XA{field}^XZ"), (0, 0, 812, 1218)).expect(field);
            let close = |a: u32, b: u32| a.abs_diff(b) <= 3;
            assert!(
                close(got.0, expected.0) && close(got.1, expected.1) && close(got.2, expected.2) && close(got.3, expected.3),
                "{field}: {got:?}, Labelary {expected:?}"
            );
        }
    }

    #[test]
    fn text_layout_matches_labelary() {
        assert_matches_labelary(&[
            // ^TB: lines 1.26 heights apart, the last one cut off at the block's bottom.
            ("^FO200,20^A0N,40,40^TBN,200,150^FDHHH HHH HHH HHH HHH HHH HHH HHH HHH^FS", (203, 20, 354, 150)),
            // ^FB centring counts a space after the last line, not after a forced break.
            ("^FO20,800^ADN^FB300,5,0,C^FDCC\\&DD^FS", (152, 800, 178, 831)),
            // ^FB is as tall as its maximum lines, used or not: ^FT and rotation see that.
            ("^FT50,400^A0N,30,30^FB400,3,0,C^FDOne line^FS", (197, 318, 293, 339)),
            (
                concat!(
                    "^FO100,20^A0R,30,30^FB400,4,0,L",
                    "^FDThe quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs.^FS"
                ),
                (133, 20, 219, 407),
            ),
            // ^FPV stacks characters one line height apart, ignoring the gap; ^FPH spaces them.
            ("^FO50,20^FPV,10^A0N,30,30^FDVERT^FS", (50, 20, 66, 131)),
            ("^FO250,150^FPH,10^A0N,30,30^FDGap 10^FS", (251, 150, 382, 176)),
            // ^GS: one symbol, in a 24-dot cell magnified to 48.
            ("^FO20,20^GSN,40,40^FDABCDE^FS", (20, 20, 49, 50)),
        ]);
    }

    #[test]
    fn interpretation_lines_match_labelary() {
        assert_matches_labelary(&[
            // Above: 8 + 2 modules over the bars; below: a fixed 6 dots under them.
            ("^FO50,250^BY4^B3N,N,60,Y,Y^FD12^FS", (50, 206, 301, 309)),
            ("^FO20,700^BY5^B3N,N,60,Y^FD12^FS", (20, 700, 334, 800)),
            // EAN-13 above: one centred line; the guard bars still reach 13 dots down.
            ("^FO50,450^BY3^BEN,60,Y,Y^FD590123412345^FS", (50, 415, 334, 522)),
        ]);
    }

    #[test]
    fn rotated_text_matches_labelary() {
        assert_matches_labelary(&[
            // ^FO: the rotated box (advance x line height) has its top-left on the origin.
            ("^FO100,100^A0N,40^FDHHHH^FS", (103, 100, 194, 129)),
            ("^FO500,100^A0R,40^FDHHHH^FS", (510, 103, 539, 194)),
            ("^FO100,400^A0I,40^FDHHHH^FS", (102, 410, 193, 439)),
            ("^FO500,400^A0B,40^FDHHHH^FS", (500, 402, 529, 493)),
            ("^FO500,700^AFR,52^FDHHHH^FS", (510, 700, 551, 822)),
            ("^FO500,950^AFB,52^FDHHHH^FS", (500, 954, 541, 1076)),
            // ^FT: the baseline starts at the origin and turns with the text.
            ("^FT500,100^A0R,40^FDHHHH^FS", (500, 103, 529, 194)),
            ("^FT250,400^A0I,40^FDHHHH^FS", (155, 400, 246, 429)),
            ("^FT500,400^A0B,40^FDHHHH^FS", (470, 305, 499, 396)),
            ("^FT500,1150^AFB,52^FDHHHH^FS", (458, 1027, 499, 1149)),
            // ^FB lays the block out along the text and stacks lines across it.
            ("^FO100,450^A0R,30^FB200,2,0,R^FDHHH\\&HH^FS", (108, 597, 159, 647)),
            ("^FO100,100^A0I,30^FB200,2,0,L^FDHHH\\&HH^FS", (247, 108, 297, 159)),
            ("^FO100,100^A0B,30^FB200,2,0,L^FDHHH\\&HH^FS", (100, 247, 151, 297)),
            ("^FT450,650^A0B,30^FB200,2,0,L^FDHHH\\&HH^FS", (398, 597, 449, 647)),
            // Right-justified fields move the rotated box left by its width on the page.
            ("^FO200,900,1^A0N,30^FDHHH^FS", (147, 900, 197, 921)),
            ("^FO200,900,1^A0R,30^FDHHH^FS", (178, 902, 199, 952)),
            ("^FO550,900,1^A0N,30^FB200,2,0,L^FDHHH\\&HH^FS", (352, 900, 402, 951)),
        ]);
    }

    #[test]
    fn rotated_barcodes_match_labelary() {
        // Bars only: ^FO puts the rotated bars' top-left on the origin, ^FT the unrotated
        // bottom-left corner.
        assert_matches_labelary(&[
            ("^BY2^FO100,150^BCN,80,N^FD12345678^FS", (100, 150, 345, 229)),
            ("^BY2^FO500,150^BCR,80,N^FD12345678^FS", (500, 150, 579, 395)),
            ("^BY2^FO100,650^BCI,80,N^FD12345678^FS", (100, 650, 345, 729)),
            ("^BY2^FO500,650^BCB,80,N^FD12345678^FS", (500, 650, 579, 895)),
            ("^BY2^FT100,150^BCN,80,N^FD12345678^FS", (100, 70, 345, 149)),
            ("^BY2^FT500,150^BCR,80,N^FD12345678^FS", (500, 150, 579, 395)),
            ("^BY2^FT400,650^BCI,80,N^FD12345678^FS", (154, 650, 399, 729)),
            ("^BY2^FT500,650^BCB,80,N^FD12345678^FS", (420, 404, 499, 649)),
            ("^BY2^FO500,650^B3B,N,80,N^FDAB12^FS", (500, 650, 579, 839)),
            ("^BY2^FT500,1000^B3B,N,80,N^FDAB12^FS", (420, 810, 499, 999)),
        ]);
        // The interpretation line stays below the bars in the symbol's frame: left of
        // them for R, above for I.
        assert_matches_labelary(&[
            ("^BY2^FO500,150^BCR,80,Y^FD12345678^FS", (480, 150, 579, 395)),
            ("^BY2^FO100,650^BCI,80,Y^FD12345678^FS", (100, 630, 345, 729)),
        ]);
    }

    #[test]
    fn rotated_2d_symbols_match_labelary() {
        // ^FO: the rotated symbol's top-left on the origin; ^FT: the unrotated bottom-left
        // corner, turned with the symbol.
        assert_matches_labelary(&[
            ("^FO50,350^B0B,5,N,0^FDAZTEC ROTATION^FS", (50, 350, 144, 444)),
            ("^FT300,500^B0N,5,N,0^FDAZTEC ROTATION^FS", (300, 405, 394, 499)),
            ("^FT550,500^B0R,5,N,0^FDAZTEC ROTATION^FS", (550, 500, 644, 594)),
            ("^FT150,800^B0I,5,N,0^FDAZTEC ROTATION^FS", (55, 800, 149, 894)),
            ("^FT450,800^B0B,5,N,0^FDAZTEC ROTATION^FS", (355, 705, 449, 799)),
        ]);
        // QR codes never turn; ^FO R and B only move the 10 dot band, ^FT ends 3 modules up.
        assert_matches_labelary(&[
            ("^FO50,50^BQR,2,4^FDQA,ROTATED^FS", (50, 50, 133, 133)),
            ("^FWB^FO300,400^BQ,2,4^FDQA,FW IGNORED^FS", (310, 400, 393, 483)),
            ("^FT300,200^BQN,2,2^FDQA,FT MAG2^FS", (300, 152, 341, 193)),
            ("^FT300,500^BQB,2,4^FDQA,FT B^FS", (300, 404, 383, 487)),
            ("^FO500,50^BQN,2^FDQA,DEFAULT MAG^FS", (500, 60, 541, 101)),
        ]);
        // Placed like 1D bars: ^FO puts the rotated symbol's top-left on the origin, ^FT the
        // unrotated bottom-left corner, rotated with the symbol.
        assert_matches_labelary(&[
            ("^FO650,50^BXB,6,200^FDROTATE DM^FS", (650, 50, 733, 133)),
            ("^FT400,150^BXN,6,200^FDROTATE DM^FS", (400, 66, 483, 149)),
            ("^FT550,150^BXR,6,200^FDROTATE DM^FS", (550, 150, 633, 233)),
            ("^FT700,150^BXI,6,200^FDROTATE DM^FS", (616, 150, 699, 233)),
            ("^FT700,400^BXB,6,200^FDROTATE DM^FS", (616, 316, 699, 399)),
            ("^BY2^FT100,450^B7N,4,0,2,6^FDPDF FT^FS", (100, 426, 305, 449)),
            ("^BY2^FT500,450^B7I,4,0,2,6^FDPDF FT^FS", (294, 450, 499, 473)),
            ("^BY2^FT100,900^B7R,4,0,2,6^FDPDF FT^FS", (100, 900, 123, 1105)),
            ("^BY2^FT400,900^B7B,4,0,2,6^FDPDF FT^FS", (376, 694, 399, 899)),
            // MaxiCode is 200 x 192 dots at 8 dots/mm.
            ("^FO30,30^BD4^FDABC1234567890123XYZ^FS", (30, 31, 229, 222)),
            ("^FT550,1180^BD4^FDFT MAXI^FS", (550, 986, 749, 1177)),
        ]);
    }

    #[test]
    fn rotated_carrier_label_matches_labelary() {
        // A carrier label printed sideways with ^FWB; boxes from Labelary's render.
        let zpl = "^XA^FWB\
                   ^FO57,57^AF,N,13,26^FB1087,1,0,L,0^FDCart ID^FS\
                   ^FO165,180^AF,N,40,40^FB640,1,0,R,0^FDCARRIER^FS\
                   ^FO600,200^BY4^BC,142,Y,N,N,N^FDACME1000002908928^FS^XZ";
        let close = |got: Option<(u32, u32, u32, u32)>, expected: (u32, u32, u32, u32)| {
            let got = got.unwrap();
            assert!(
                [(got.0, expected.0), (got.1, expected.1), (got.2, expected.2), (got.3, expected.3)]
                    .iter()
                    .all(|(a, b)| a.abs_diff(*b) <= 3),
                "{got:?}, Labelary {expected:?}"
            );
        };
        // Font F 1x1 reading upwards from the bottom of its 1087-dot block (57 + 1087).
        close(ink_box(zpl, (40, 0, 100, 1218)), (57, 1034, 77, 1143));
        // Font F 3x3, right-justified: the text ends at the top of its block, y 180.
        close(ink_box(zpl, (150, 0, 250, 600)), (164, 185, 228, 515));
        // Bars from y 200 down, the interpretation line to their right.
        close(ink_box(zpl, (590, 0, 743, 1218)), (600, 200, 741, 1087));
        let text = ink_box(zpl, (743, 0, 812, 1218)).unwrap();
        assert!(text.0.abs_diff(748) <= 3 && text.2.abs_diff(776) <= 3, "{text:?}");
        // ^FO156,-1257 is y 1257: off the label, so "X" prints nowhere.
        let off_label = "^XA^FWB^FO156,-1257^AF,N,13,26^FB1417,1,0,L,0^FDX^FS^XZ";
        assert_eq!(ink_box(off_label, (0, 0, 812, 1218)), None);
    }
}
