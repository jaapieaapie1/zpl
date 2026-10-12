use std::sync::Arc;

use ab_glyph::{FontArc, FontRef, FontVec, PxScale};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use zpl_parser::command::{
    BlockJustification, Code128Mode, DiagonalDirection, FontName, Justification, LineColor, LinearKind,
    Orientation, TextLayout,
};
use zpl_state_manager::{DrawInstruction, PageSetup, TextAnchor, TextBlock, DEFAULT_DPMM};
use zpl_symbols::linear::{self, LinearSymbol, Symbology};
use zpl_symbols::matrix::{self, Matrix};

pub mod code128;
mod layout;
mod symbols;

pub use layout::DEJAVU_SANS_BOLD;

/// Font files, shared with pdf-renderer so the binary holds one copy of each.
pub static TRIUMVIRATE: &[u8] = include_bytes!("../fonts/ATTriumvirate-BoldCd.otf");
pub static DEJAVU_SANS_MONO: &[u8] = include_bytes!("../fonts/DejaVuSansMono.ttf");

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
}

/// Bitmap font matrices on a `dpmm` printhead (H, W, intercharacter gap, baseline,
/// uppercase only); see docs/commands/other/FONTS.md and pdf-renderer's text.rs. Only E and
/// H differ between resolutions; their gap and baseline scale from the 8 dots/mm values.
fn bitmap_cell(font: FontName, dpmm: u32) -> Option<(u32, u32, u32, u32, bool)> {
    let scaled = |(h, w): (u32, u32), (h8, w8, gap8, base8): (u32, u32, u32, u32), uppercase| {
        let gap = ((gap8 * w) as f32 / w8 as f32).round() as u32;
        let base = ((base8 * h) as f32 / h8 as f32).round() as u32;
        (h, w, gap, base, uppercase)
    };
    Some(match font {
        FontName::FontA => (9, 5, 1, 7, false),
        FontName::FontB => (11, 7, 2, 11, true),
        FontName::FontC | FontName::FontD => (18, 10, 2, 14, false),
        FontName::FontE => {
            let size = match dpmm {
                0..=6 => (21, 10),
                7..=8 => (28, 15),
                _ => (42, 20),
            };
            scaled(size, (28, 15, 5, 23), false)
        }
        FontName::FontF => (26, 13, 3, 21, false),
        FontName::FontG => (60, 40, 8, 48, false),
        FontName::FontH => {
            let size = match dpmm {
                0..=6 => (17, 11),
                7..=8 => (21, 13),
                _ => (34, 22),
            };
            scaled(size, (21, 13, 6, 21), true)
        }
        _ => return None,
    })
}

/// Fonts P–V: fixed-size font 0 (H x W).
fn fixed_scalable(font: FontName) -> Option<(u32, u32)> {
    Some(match font {
        FontName::FontP => (20, 18),
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

        let mono: &'static [u8] = DEJAVU_SANS_MONO;

        if let (None, Some((h, w, gap, base, uppercase))) = (custom, bitmap_cell(name, dpmm)) {
            let font = FontArc::try_from_slice(mono).expect("Error loading font");
            let (mh, mw) = magnification(height, width, h, w);
            let upem = font.units_per_em().unwrap_or(2048.0);
            // Outline bounds may be y-down; 'H' sits on the baseline, so its extent is the cap height.
            let cap_unscaled =
                font.outline(font.glyph_id('H')).map_or(upem * 0.73, |o| (o.bounds.max.y - o.bounds.min.y).abs());
            let (ink_min, ink_w) = font
                .outline(font.glyph_id('M'))
                .map_or((0.0, upem * 0.5), |o| (o.bounds.min.x, o.bounds.max.x - o.bounds.min.x));
            // Cap height = character height above the baseline; 'M' fills the cell width.
            let cap = (base * mh) as f32;
            let em_y = cap * upem / cap_unscaled;
            let em_x = (w * mw) as f32 * upem / ink_w;
            let px_per_em = font.height_unscaled() / upem;
            let scale = PxScale { x: em_x * px_per_em, y: em_y * px_per_em };
            let y_offset = ((font.ascent_unscaled() - cap_unscaled) / upem * em_y).round() as i32;
            return Self {
                font,
                data: layout::FontData::Static(mono),
                cap_em: cap_unscaled / upem,
                scale,
                y_offset,
                cap: cap.round() as i32,
                line_h: h * mh,
                cell: Some((((w + gap) * mw) as f32, -ink_min / upem * em_x)),
                uppercase,
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
        let (height, width) = match fixed_scalable(name).filter(|_| custom.is_none()) {
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
        let scale = PxScale {
            x: width as f32 * px_per_em,
            y: height as f32 * px_per_em,
        };

        // Zebra places the top of capital letters at the field origin, ab_glyph places the ascent there
        let cap_height = units_per_em * cap_em;
        let y_offset = ((font.ascent_unscaled() - cap_height) / units_per_em * height as f32).round() as i32;
        let cap = (height as f32 * cap_em).round() as i32;
        Self { font, data, cap_em, scale, y_offset, cap, line_h: height, cell: None, uppercase: false }
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

/// Interpretation line font for 1D barcodes: monospaced; pitch, digit height and gap scale
/// with the module width (matches Labelary).
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
    wrap_lines_with(&|s: &str| font.measure(s), text, width, indent)
}

/// [`wrap_lines`] with `measure` giving the width of a line.
fn wrap_lines_with(measure: &dyn Fn(&str) -> f32, text: &str, width: u32, indent: u32) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let avail = |lines: &Vec<String>| {
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
                lines.push(std::mem::take(&mut line));
            }
            // Split a word that does not fit on a line of its own.
            for c in word.chars() {
                line.push(c);
                if measure(&line) > avail(&lines) && line.chars().count() > 1 {
                    line.pop();
                    lines.push(std::mem::take(&mut line));
                    line.push(c);
                }
            }
        }
        lines.push(line);
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
            #[cfg(feature = "source-map")]
            drawn: std::cell::Cell::new(None),
        }
    }

    /// Renderer for a label's page settings: size, printer resolution and ^JM / ^MU format
    /// scale. The image has one pixel per printer dot.
    pub fn for_page(page: &PageSetup, debug: bool) -> Self {
        Self { dpmm: page.dpmm, scale: page.scale, ..Self::with_debug(page.width, page.height, debug) }
    }

    pub fn render(&self, instructions: &[DrawInstruction]) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::from_pixel(self.width, self.height, WHITE);

        for instruction in instructions {
            self.render_instruction(&mut image, instruction);
        }

        self.to_printer_dots(image)
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
                x, y, text, font, height, width, orientation, justification, anchor, block, custom_font, layout, ..
            } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([255, 0, 0]));
                }
                let font = TextFont::new(*font, custom_font.as_ref(), *height, *width, self.dpmm);
                // Control characters (from ^FH escapes such as _0D) print nothing.
                let text: String = text.chars().filter(|&c| c >= ' ' && c != '\x7f').collect();
                let text = if font.uppercase { text.to_uppercase() } else { text };
                self.render_text(
                    image, *x, *y, &text, &font, *orientation, *justification, *anchor, block.as_ref(), layout,
                );
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
            DrawInstruction::DrawQrCode { x, y, data, magnification, bottom, .. } => {
                self.render_qr_code(image, *x, *y, data, *magnification, *bottom);
            }
            DrawInstruction::DrawDataMatrix {
                x, y, data, orientation, module_height, symbol_height, columns, rows, escape, rectangular, bottom,
            } => {
                if let Ok(m) = matrix::datamatrix(data.as_bytes(), *rows, *columns, *rectangular, *escape) {
                    // Without an explicit module size the ^BY height is the approximate symbol height.
                    let module = module_height.unwrap_or(symbol_height / m.height.max(1)).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, module, *bottom);
                }
            }
            DrawInstruction::DrawPdf417 {
                x, y, data, orientation, row_height, module_width, symbol_height, security, columns, rows,
                truncated, bottom,
            } => {
                if let Ok(m) = matrix::pdf417(data.as_bytes(), *security, *columns, *rows, *truncated) {
                    let module = (*module_width).max(1);
                    // Row height is a multiple of the module width, or the ^BY height shared by all rows.
                    let row = row_height.map(|h| h * module).unwrap_or(symbol_height / m.height.max(1)).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, row, *bottom);
                }
            }
            DrawInstruction::DrawAztec { x, y, data, orientation, magnification, ecc, bottom } => {
                if let Ok(m) = matrix::aztec(data.as_bytes(), *ecc) {
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
        orientation: Orientation,
        justification: Justification,
        anchor: TextAnchor,
        block: Option<&TextBlock>,
        text_layout: &TextLayout,
    ) {
        let height = font.line_h;
        // ^PA bidi/shaping: scalable fonts are laid out as glyph runs (with a fallback font
        // for Hebrew/Arabic); bitmap fonts keep their cells and are only reordered.
        let advanced = layout::active(text_layout, text);
        let runs = advanced && font.cell.is_none();
        let primary = runs.then(|| layout::LayoutFont { font: font.font.clone(), data: font.data.clone(), scale: font.scale });
        let measure = |s: &str| match &primary {
            Some(p) => layout::layout_line(p, font.cap_em, text_layout, s).width,
            None => font.measure(s),
        };
        let display = |s: String| if advanced && font.cell.is_some() { layout::visual_string(text_layout, &s) } else { s };

        // The unrotated text box: as wide as the text (or the ^FB block), one line height per
        // row, with the top of the capitals at the top of each row.
        let mut rows: Vec<(i32, i32, String)> = Vec::new();
        let (box_w, box_h, last_row_top) = match block {
            None => {
                rows.push((0, 0, display(text.to_string())));
                (measure(text).ceil() as u32, height, 0)
            }
            Some(b) => {
                let line_height = (height as i32 + b.line_spacing).max(1);
                let mut lines = wrap_lines_with(&measure, text, b.width, b.hanging_indent);
                if let Some(clip) = b.clip_height {
                    // ^TB: drop lines that do not fit in the block height (keep at least one).
                    let fit = ((clip as i32 - height as i32) / line_height + 1).max(1) as usize;
                    lines.truncate(fit);
                }
                let max_rows = b.max_lines.max(1) as usize;
                let mut last_row = 0;
                for (i, line) in lines.into_iter().enumerate() {
                    // ^FB: lines beyond the maximum overprint the last one.
                    let row = i.min(max_rows - 1);
                    last_row = last_row.max(row);
                    let indent = if i > 0 { b.hanging_indent as f32 } else { 0.0 };
                    let line_w = measure(&line);
                    let free = (b.width as f32 - indent - line_w).max(0.0);
                    let x_off = match b.justification {
                        BlockJustification::Left | BlockJustification::Justified => indent,
                        BlockJustification::Center => indent + free / 2.0,
                        BlockJustification::Right => indent + free,
                    };
                    rows.push((x_off.round() as i32, row as i32 * line_height, display(line)));
                }
                let last_top = last_row as i32 * line_height;
                (b.width.max(1), last_top as u32 + height, last_top)
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

        // ^FO: the rotated box's top-left is the origin. ^FT: the start of the last line's
        // baseline is, so find where that point lands in the rotated box.
        let (w, h) = (box_w as i64, box_h as i64);
        let baseline = (last_row_top + font.cap) as i64;
        let (rotated_w, (px, py)) = match orientation {
            Orientation::Normal => (w, (0, baseline)),
            Orientation::Rotated90 => (h, (h - baseline, 0)),
            Orientation::Rotated180 => (w, (w, h - baseline)),
            Orientation::Rotated270 => (h, (baseline, w)),
        };
        let (mut x, y) = match anchor {
            TextAnchor::Top => (x as i64, y as i64),
            TextAnchor::Baseline => (x as i64 - px, y as i64 - py),
        };
        // ^FO / ^FT justification moves the rotated box left on the page.
        x -= match justification {
            Justification::Left => 0,
            Justification::Right => rotated_w,
            Justification::Auto => rotated_w / 2,
        };

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

    /// Places an unrotated symbol image whose bars start `pad_l` dots from its left edge (with
    /// `pad_r` dots of text overhang on the right) at the field origin, rotated like the bars.
    #[allow(clippy::too_many_arguments)]
    fn place_symbol(
        &self,
        image: &mut RgbImage,
        temp: &RgbImage,
        x: u32,
        y: u32,
        orientation: Orientation,
        pad_l: u32,
        pad_r: u32,
    ) {
        let (x, y) = (x as i64, y as i64);
        match orientation {
            Orientation::Normal => self.overlay_at(image, temp, x - pad_l as i64, y),
            Orientation::Rotated90 => {
                let rotated = image::imageops::rotate90(temp);
                self.overlay_at(image, &rotated, x, y - pad_l as i64);
            }
            Orientation::Rotated180 => {
                let rotated = image::imageops::rotate180(temp);
                self.overlay_at(image, &rotated, x - pad_r as i64, y);
            }
            Orientation::Rotated270 => {
                let rotated = image::imageops::rotate270(temp);
                let adjusted_x = x - rotated.width() as i64;
                let adjusted_y = y - rotated.height() as i64 + pad_l as i64;
                self.overlay_at(image, &rotated, adjusted_x, adjusted_y);
            }
        }
    }

    /// Places the unrotated image of a 1D symbol whose bars are the `bars_w` x `bars_h` box at
    /// (`bars_x`, `bars_y`): ^FO puts the rotated bars' top-left on (`x`, `y`), ^FT (`bottom`)
    /// the bars' bottom-left corner, rotated with the symbol. The interpretation line follows
    /// the bars wherever it lands.
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
        let text_gap = (bar_width * 2) as f32;

        let show_text = print_interpretation_line && !code.text.is_empty();
        let text_width = if show_text { ip.text_width(&code.text).ceil() as u32 } else { 0 };
        let text_area = if show_text { (text_gap + ip.digit_height - ip.descent).ceil() as u32 } else { 0 };

        let pad = text_width.saturating_sub(barcode_width) / 2;
        let total_width = barcode_width + 2 * pad;
        let total_height = height + text_area;
        let (bars_y, baseline) = if print_interpretation_line_above {
            (text_area, ip.digit_height)
        } else {
            (0, height as f32 + text_gap + ip.digit_height)
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

        if show_text {
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
        let text_gap = (module * 2) as f32;
        let show_text = print_line && !symbol.text.is_empty();
        let text_area = if show_text { (text_gap + ip.digit_height - ip.descent).ceil() as u32 } else { 0 };
        let guard_ext = if symbol.has_guards && show_text && !line_above {
            (text_gap + ip.digit_height / 2.0).round() as u32
        } else {
            0
        };
        let bars_y = if line_above { text_area } else { 0 };

        // Text segments (e.g. the EAN-13 leading digit) may stick out of the bars.
        let mut segments = Vec::new();
        let (mut left, mut right) = (0.0f32, width as f32);
        if show_text {
            for seg in &symbol.text {
                let (start, end) = symbol.text_extent_dots(seg, module, ratio);
                let text_x = (start + end - ip.text_width(&seg.text)) / 2.0;
                left = left.min(text_x);
                right = right.max(text_x + ip.text_width(&seg.text));
                segments.push((text_x, seg.text.as_str()));
            }
        }
        let pad_l = (-left).ceil() as u32;
        let pad_r = (right - width as f32).ceil() as u32;

        let total_w = (pad_l + width + pad_r).max(1);
        let total_h = (height + text_area).max(1);
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
            for (text_x, text) in segments {
                ip.draw(&mut temp, text, pad_l as f32 + text_x, baseline);
            }
        }

        self.place_bars(image, &temp, x, y, bottom, orientation, (pad_l, bars_y, width, height));
    }

    /// 2D symbol with `module_w` x `module_h` dot modules; with `bottom` (^FT) `y` is the
    /// symbol's bottom edge.
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
        let y = if bottom { y.saturating_sub(h) } else { y };
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
        self.place_symbol(image, &temp, x, y, orientation, 0, 0);
    }

    fn render_qr_code(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, magnification: u32, bottom: bool) {
        use qrcode::{EcLevel, QrCode};

        // ^FD for QR codes starts with "<error correction><input mode>," e.g. "QA,data"
        let (ec_level, payload) = match data.get(..3) {
            Some(prefix) if prefix.ends_with(',') => {
                let ec_level = match &prefix[..1] {
                    "H" => EcLevel::H,
                    "Q" => EcLevel::Q,
                    "L" => EcLevel::L,
                    _ => EcLevel::M,
                };
                (ec_level, &data[3..])
            }
            _ => (EcLevel::M, data),
        };

        let module_size = magnification.max(1);

        if let Ok(code) = QrCode::with_error_correction_level(payload.as_bytes(), ec_level) {
            let qr_image = code.render::<Rgb<u8>>()
                .quiet_zone(false)
                .module_dimensions(module_size, module_size)
                .build();

            // ^FT: y is the bottom edge. ^FO: Zebra printers draw QR codes 10 dots below the field origin.
            let y = if bottom {
                y.saturating_sub(code.width() as u32 * module_size)
            } else {
                y + 10
            };

            self.note_drawn(x as i64, y as i64, qr_image.width(), qr_image.height());
            for (qr_x, qr_y, pixel) in qr_image.enumerate_pixels() {
                let target_x = x + qr_x;
                let target_y = y + qr_y;

                if target_x < image.width() && target_y < image.height() {
                    image.put_pixel(target_x, target_y, *pixel);
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

    #[allow(clippy::too_many_arguments)]
    fn render_diagonal_line(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor, direction: &DiagonalDirection) {
        self.note_drawn(x as i64, y as i64, width + thickness, height + thickness);
        let rgb_color = self.line_color_to_rgb(color);

        let (x1, y1, x2, y2) = match direction {
            DiagonalDirection::LeftToRight => (x as f32, y as f32, (x + width) as f32, (y + height) as f32),
            DiagonalDirection::RightToLeft => ((x + width) as f32, y as f32, x as f32, (y + height) as f32),
        };

        for t in 0..thickness {
            let offset = t as f32;
            drawing::draw_line_segment_mut(
                image,
                (x1 + offset, y1 + offset),
                (x2 + offset, y2 + offset),
                rgb_color,
            );
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
        assert_eq!(bitmap_cell(FontName::FontE, 6).unwrap().0, 21);
        assert_eq!(bitmap_cell(FontName::FontE, 8).unwrap().0, 28);
        assert_eq!(bitmap_cell(FontName::FontE, 24).unwrap().0, 42);
        assert_eq!(bitmap_cell(FontName::FontH, 12).unwrap().1, 22);
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
