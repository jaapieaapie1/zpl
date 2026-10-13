//! Barcodes (Code 128, Code 39) and QR codes as vector paths.
//!
//! Bars and QR modules are emitted as filled rectangles (`re` ... `f`). Geometry and placement
//! mirror `png_renderer`'s `render_code128`, `render_code39` and `render_qr_code`: each symbol
//! is laid out in a local, unrotated frame (origin top-left, y down, in dots) exactly like the
//! PNG renderer's temporary image, and a `cm` maps that frame to where the PNG renderer places
//! the rotated image.

use crate::text;
use crate::{Ctx, FontId};
use crate::matrix;
use png_renderer::{GUARD_EXTENSION_BELOW, code128, guarded_line, qr};
use zpl_parser::command::{Code128Mode, LinearKind, Orientation};
use zpl_symbols::linear::{self, LinearSymbol, MsiCheck, Symbology};
use zpl_state_manager::DrawInstruction;

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    match instruction {
        DrawInstruction::DrawBarcode {
            x, y, bottom, right, data, orientation, height, module_width,
            print_interpretation_line, print_interpretation_line_above, mode, ..
        } => render_code128(
            ctx, (*x, *y, *bottom, *right), data, *orientation, *height, *module_width,
            *print_interpretation_line, *print_interpretation_line_above, *mode,
        ),
        DrawInstruction::DrawLinear {
            x, y, bottom, right, kind, data, orientation, height, module_width, wide_bar_ratio,
            print_interpretation_line, print_interpretation_line_above,
        } => render_linear(
            ctx, (*x, *y, *bottom, *right), *kind, data, *orientation, *height, *module_width, *wide_bar_ratio,
            *print_interpretation_line, *print_interpretation_line_above,
        ),
        DrawInstruction::DrawQrCode { x, y, data, magnification, mask, bottom, right, .. } => {
            render_qr_code(ctx, (*x, *y, *bottom, *right), data, *magnification, *mask)
        }
        DrawInstruction::DrawDataMatrix { .. }
        | DrawInstruction::DrawPdf417 { .. }
        | DrawInstruction::DrawAztec { .. } => matrix::render(ctx, instruction),
        _ => {}
    }
}

/// Symbol origin: `(x, y, bottom, right)`, ^FT and right justification (z=1) flags as in
/// `DrawInstruction::DrawBarcode`.
pub(crate) type Origin = (u32, u32, bool, bool);

/// Transform for a symbol whose bars are the local box `bars` (x, y, w, h): ^FO puts the
/// rotated bars' top-left on (`x`, `y`), ^FT (`bottom`) the bars' bottom-left corner, rotated
/// with the symbol; right justified the rotated bars end at `x` (not left of the page edge).
/// The interpretation line follows the bars (as the PNG renderer's `place_bars`).
pub(crate) fn bars_placement(orientation: Orientation, (x, y, bottom, right): Origin, bars: [f32; 4]) -> [f32; 6] {
    let [bx, by, bw, bh] = bars;
    let (x, y) = (x as f32, y as f32);
    // Top-left of the rotated bars on the page.
    let (left, top) = match (bottom, orientation) {
        (false, _) | (true, Orientation::Rotated90) => (x, y),
        (true, Orientation::Normal) => (x, y - bh),
        (true, Orientation::Rotated180) => (x - bw, y),
        (true, Orientation::Rotated270) => (x - bh, y - bw),
    };
    let left = match orientation {
        _ if !right => left,
        Orientation::Normal | Orientation::Rotated180 => (left - bw).max(0.0),
        Orientation::Rotated90 | Orientation::Rotated270 => (left - bh).max(0.0),
    };
    match orientation {
        Orientation::Normal => [1.0, 0.0, 0.0, 1.0, left - bx, top - by],
        // Clockwise: local (u, v) -> (left + by + bh - v, top + u - bx)
        Orientation::Rotated90 => [0.0, 1.0, -1.0, 0.0, left + by + bh, top - bx],
        Orientation::Rotated180 => [-1.0, 0.0, 0.0, -1.0, left + bx + bw, top + by + bh],
        // Counter-clockwise: local (u, v) -> (left + v - by, top + bx + bw - u)
        Orientation::Rotated270 => [0.0, -1.0, 1.0, 0.0, left - by, top + bx + bw],
    }
}

/// Fill `(x, y, w, h)` rects in black inside `matrix`, wrapped in `q`/`Q`.
pub(crate) fn fill_rects(ctx: &mut Ctx, matrix: [f32; 6], rects: &[(f32, f32, f32, f32)]) {
    if rects.is_empty() {
        return;
    }
    ctx.content.save_state();
    ctx.content.transform(matrix);
    ctx.content.set_fill_gray(ctx.ink);
    for &(x, y, w, h) in rects {
        ctx.content.rect(x, y, w, h);
    }
    ctx.content.fill_nonzero();
    ctx.content.restore_state();
}

/// Space between the bars and an interpretation line (dots, matches Labelary): below them a
/// fixed 6 (EAN/UPC: twice the module, their guard bars reaching into the digits), above
/// them 8 plus twice the module.
pub(crate) fn line_gap(module: u32, above: bool, guards: bool) -> f32 {
    match (above, guards) {
        (true, _) => (8 + 2 * module) as f32,
        (false, true) => (2 * module) as f32,
        (false, false) => 6.0,
    }
}

/// How far EAN/UPC guard bars reach below the others when the digits are printed above.
const GUARD_EXTENSION_ABOVE: u32 = 13;

/// Interpretation line layout for a 1D symbol (all in dots).
pub(crate) struct Interpretation {
    pub(crate) font: FontId,
    char_advance: f32,
    pub(crate) digit_height: f32,
    pub(crate) size_x: f32,
    pub(crate) size_y: f32,
    descent: f32,
}

pub(crate) fn interpretation(bar_width: u32) -> Interpretation {
    interpretation_in(FontId::DejaVuMono, bar_width)
}

/// [`interpretation`] in `font`.
fn interpretation_in(font: FontId, bar_width: u32) -> Interpretation {
    // Same constants as the PNG renderer (Labelary-compatible): no larger than at module 10.
    let bar_width = bar_width.min(10);
    interpretation_sized_in(font, bar_width as f32 * 143.0 / 24.0, (bar_width * 7) as f32)
}

/// The interpretation font `font` unstretched, its digits `digit_height` dots tall.
fn interpretation_upright(font: FontId, digit_height: f32) -> Interpretation {
    let adv_em = Some(text::char_advance(font, '0')).filter(|v| *v > 0.0).unwrap_or(1233.0 / 2048.0);
    let zero_h_em = Some(text::glyph_height(font, '0')).filter(|v| *v > 0.0).unwrap_or(1493.0 / 2048.0);
    interpretation_sized_in(font, digit_height * adv_em / zero_h_em, digit_height)
}

/// DejaVu Sans Mono stretched to `char_advance` dots per character and `digit_height` dots
/// tall digits.
pub(crate) fn interpretation_sized(char_advance: f32, digit_height: f32) -> Interpretation {
    interpretation_sized_in(FontId::DejaVuMono, char_advance, digit_height)
}

/// [`interpretation_sized`] in `font`.
fn interpretation_sized_in(font: FontId, char_advance: f32, digit_height: f32) -> Interpretation {
    // DejaVu Sans Mono fallbacks (em units) in case metrics are unavailable.
    let adv_em = Some(text::char_advance(font, '0')).filter(|v| *v > 0.0).unwrap_or(1233.0 / 2048.0);
    let zero_h_em = Some(text::glyph_height(font, '0')).filter(|v| *v > 0.0).unwrap_or(1493.0 / 2048.0);
    let descent_em = Some(text::metrics(font).descent.abs()).filter(|v| *v > 0.0).unwrap_or(483.0 / 2048.0);

    let size_x = char_advance / adv_em;
    let size_y = digit_height / zero_h_em;
    Interpretation { font, char_advance, digit_height, size_x, size_y, descent: descent_em * size_y }
}

impl Interpretation {
    /// Height of the text and its gap to the bars, in whole dots.
    pub(crate) fn area(&self, gap: f32, above: bool) -> u32 {
        let below = if above { 0.0 } else { self.descent };
        (gap + self.digit_height + below).ceil() as u32
    }
}

#[allow(clippy::too_many_arguments)]
fn render_code128(
    ctx: &mut Ctx, origin: Origin, data: &str, orientation: Orientation, height: u32,
    module_width: u32, print_line: bool, line_above: bool, mode: Code128Mode,
) {
    let code = code128::encode(data, mode);
    let bar_width = module_width.max(1);
    let widths = code.widths();
    let barcode_width: u32 = widths.iter().map(|&w| w as u32 * bar_width).sum();

    let ip = interpretation(bar_width);
    let text_gap = line_gap(bar_width, line_above, false);
    let show_text = print_line && !code.text.is_empty();
    // Mode D sizes its line in font 0 to the bars, as the PNG renderer does.
    let gs1 = (show_text && mode == Code128Mode::Gs1)
        .then(|| code128::Gs1Line::new(&code.text, barcode_width, height, line_above))
        .flatten();
    let n_chars = code.text.chars().count() as f32;
    let text_width = match &gs1 {
        Some(_) => barcode_width,
        None if show_text => (ip.char_advance * n_chars).ceil() as u32,
        None => 0,
    };
    let text_area = match &gs1 {
        Some(line) => line.area,
        None if show_text => ip.area(text_gap, line_above),
        None => 0,
    };

    let pad = text_width.saturating_sub(barcode_width) / 2;
    let (bars_y, baseline) = match &gs1 {
        Some(line) => (line.bars_y, line.baseline),
        None if line_above => (text_area, ip.digit_height),
        None => (0, height as f32 + text_gap + ip.digit_height),
    };

    let mut rects = Vec::new();
    let mut x_pos = pad;
    for (i, &w) in widths.iter().enumerate() {
        let px = w as u32 * bar_width;
        if i % 2 == 0 && height > 0 {
            rects.push((x_pos as f32, bars_y as f32, px as f32, height as f32));
        }
        x_pos += px;
    }

    let bars = [pad as f32, bars_y as f32, barcode_width as f32, height as f32];
    let matrix = bars_placement(orientation, origin, bars);
    fill_rects(ctx, matrix, &rects);

    if show_text {
        ctx.content.save_state();
        ctx.content.transform(matrix);
        ctx.content.set_fill_gray(ctx.ink);
        match gs1 {
            Some(line) => text::show_text(ctx, FontId::Triumvirate, &code.text, line.x, baseline, line.em, line.em),
            None => {
                let text_x = pad as f32 + (barcode_width as f32 - text_width as f32) / 2.0;
                text::show_text(ctx, FontId::DejaVuMono, &code.text, text_x, baseline, ip.size_x, ip.size_y);
            }
        }
        ctx.content.restore_state();
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
        LinearKind::Msi { .. } => Symbology::Msi { check: MsiCheck::None },
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
                Zpl::None => MsiCheck::None,
                Zpl::Mod10 => MsiCheck::Mod10,
                Zpl::Mod10Mod10 => MsiCheck::Mod10Mod10,
                Zpl::Mod11Mod10 => MsiCheck::Mod11Mod10,
            };
            linear::encode_msi(data, check, print_check_digit)
        }
        kind => linear::encode(symbology(kind), data),
    }
    .ok()?;
    // UPC-A/UPC-E print the check digit as the last (outside) text segment.
    if matches!(kind, LinearKind::UpcA { print_check_digit: false } | LinearKind::UpcE { print_check_digit: false }) {
        symbol.text.pop();
    }
    Some(symbol)
}

/// All 1D symbologies except Code 128, placed by their bars (`bars_placement`); the
/// interpretation line (DejaVu Sans Mono at the Code 128 proportions, EAN/UPC from some
/// module widths OCR-B) goes below, or above
/// when requested. EAN/UPC digits below the bars follow `png_renderer::guarded_line`.
#[allow(clippy::too_many_arguments)]
fn render_linear(
    ctx: &mut Ctx, origin: Origin, kind: LinearKind, data: &str, orientation: Orientation, height: u32,
    module_width: u32, ratio: f32, print_line: bool, line_above: bool,
) {
    let Some(symbol) = encode_linear(kind, data) else { return };
    let module = module_width.max(1);
    let width = symbol.width_dots(module, ratio);

    // The UPC/EAN add-on keeps its digits close above the bars.
    let add_on = matches!(kind, LinearKind::UpcEanExtension);
    let show_text = print_line && !symbol.text.is_empty();
    let guarded = (show_text && symbol.has_guards && !line_above && !add_on).then(|| guarded_line(&symbol, module));
    // OCR-B or font A by the module (docs/commands/zpl/BU.md, `png_renderer::ean_upc_ocr_b`).
    let font = match symbol.has_guards && !add_on && png_renderer::ean_upc_ocr_b(module, ctx.dpmm) {
        true => FontId::OcrB,
        false => FontId::DejaVuMono,
    };
    let ip = match &guarded {
        Some(line) => interpretation_upright(font, line.digit_height),
        None => interpretation_in(font, module),
    };
    let text_gap = match &guarded {
        Some(line) => line.gap,
        None if add_on => (2 * module) as f32,
        None => line_gap(module, line_above, symbol.has_guards),
    };
    let text_area = if show_text { ip.area(text_gap, line_above && !add_on) } else { 0 };
    let guard_ext = match (symbol.has_guards && show_text, line_above) {
        (false, _) => 0,
        (true, true) => GUARD_EXTENSION_ABOVE,
        (true, false) => GUARD_EXTENSION_BELOW,
    };
    let bars_y = if line_above { text_area } else { 0 };

    let mut rects = Vec::new();
    let mut x_pos = 0;
    for (element, dots) in symbol.element_dots(module, ratio) {
        if element.bar && height > 0 {
            let h = height + if element.guard { guard_ext } else { 0 };
            rects.push((x_pos as f32, bars_y as f32, dots as f32, h as f32));
        }
        x_pos += dots;
    }

    let matrix = bars_placement(orientation, origin, [0.0, bars_y as f32, width as f32, height as f32]);
    fill_rects(ctx, matrix, &rects);

    if show_text {
        let baseline = if line_above { ip.digit_height } else { height as f32 + text_gap + ip.digit_height };
        ctx.content.save_state();
        ctx.content.transform(matrix);
        ctx.content.set_fill_gray(ctx.ink);
        if let Some(line) = &guarded {
            for &(c, x) in &line.digits {
                let digit = c.encode_utf8(&mut [0; 4]).to_string();
                let x = x - ip.char_advance / 2.0;
                text::show_text(ctx, ip.font, &digit, x, baseline, ip.size_x, ip.size_y);
            }
        } else if line_above && symbol.has_guards {
            // Above the bars EAN/UPC digits are one line centred over the symbol.
            let text: String = symbol.text.iter().map(|seg| seg.text.as_str()).collect();
            let text_x = (width as f32 - ip.char_advance * text.chars().count() as f32) / 2.0;
            text::show_text(ctx, ip.font, &text, text_x, baseline, ip.size_x, ip.size_y);
        } else {
            for seg in &symbol.text {
                let (start, end) = symbol.text_extent_dots(seg, module, ratio);
                let text_w = ip.char_advance * seg.text.chars().count() as f32;
                let text_x = (start + end - text_w) / 2.0;
                text::show_text(ctx, ip.font, &seg.text, text_x, baseline, ip.size_x, ip.size_y);
            }
        }
        ctx.content.restore_state();
    }
}

/// ^BQ, module for module as the PNG renderer draws it (`png_renderer::qr`).
fn render_qr_code(ctx: &mut Ctx, (x, y, bottom, right): Origin, data: &str, magnification: u32, mask: Option<u32>) {
    let Some(code) = qr::encode(data, mask) else { return };
    let size = code.size;
    let module = magnification.max(1) as f32;

    let mut rects = Vec::new();
    let mut row_bits = Vec::with_capacity(size as usize);
    for row in 0..size {
        row_bits.clear();
        row_bits.extend((0..size).map(|col| code.get(col, row) as u8));
        for (start, len) in runs(&row_bits) {
            rects.push((start as f32 * module, row as f32 * module, len as f32 * module, module));
        }
    }

    let (left, top) = qr::top_left(x, y, size, magnification.max(1), bottom, right);
    let matrix = [1.0, 0.0, 0.0, 1.0, left as f32, top as f32];
    fill_rects(ctx, matrix, &rects);
}

/// Runs of consecutive `1`s as `(start, len)`.
pub(crate) fn runs(bits: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bits.len() {
        if bits[i] == 1 {
            let start = i;
            while i < bits.len() && bits[i] == 1 {
                i += 1;
            }
            out.push((start, i - start));
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_ctx() -> Ctx {
        Ctx::new()
    }

    fn content_of(instruction: DrawInstruction) -> String {
        let mut ctx = new_ctx();
        render(&mut ctx, &instruction);
        String::from_utf8(ctx.content.finish().to_vec()).unwrap()
    }

    fn count_op(content: &str, op: &str) -> usize {
        content.lines().filter(|l| l.split_whitespace().last() == Some(op)).count()
    }

    fn code128(data: &str, orientation: Orientation, line: bool) -> DrawInstruction {
        DrawInstruction::DrawBarcode {
            x: 100, y: 50, bottom: false, right: false, data: data.into(), orientation, height: 80, module_width: 2,
            wide_bar_ratio: 3.0, print_interpretation_line: line,
            print_interpretation_line_above: false, check_digit: false, mode: Code128Mode::Auto,
        }
    }

    #[test]
    fn code128_emits_one_rect_per_bar() {
        let code = code128::encode("12345678", Code128Mode::Auto);
        let bars = code.widths().len().div_ceil(2);
        // Start C + 4 data + check = 6 symbols * 3 bars + stop (4 bars) = 22.
        assert_eq!(bars, 22);

        let c = content_of(code128("12345678", Orientation::Normal, false));
        assert_eq!(count_op(&c, "re"), bars);
        assert_eq!(count_op(&c, "f"), 1);
        assert_eq!(count_op(&c, "q"), 1);
        assert_eq!(count_op(&c, "Q"), 1);
        assert!(c.contains("0 g"));
        assert!(c.contains("1 0 0 1 100 50 cm"));
        // First bar: start-C pattern begins with a 2-module bar -> 4 dots wide, 80 tall.
        assert!(c.contains("0 0 4 80 re"));
    }

    #[test]
    fn code128_total_width_matches_modules() {
        let code = code128::encode("ABC", Code128Mode::Auto);
        let modules: u32 = code.widths().iter().map(|&w| w as u32).sum();
        // (start + 3 data + check) * 11 + stop 13
        assert_eq!(modules, 5 * 11 + 13);
    }

    #[test]
    fn code128_interpretation_line_keeps_bars_and_pads_wide_text() {
        // With the line below, bars stay at the top of the frame and unpadded.
        let c = content_of(code128("12345678", Orientation::Normal, true));
        assert_eq!(count_op(&c, "re"), 22);
        assert!(c.contains("0 0 4 80 re"));

        // Long subset-C runs make the text wider than the bars -> PNG-style padding shifts
        // the frame left by `pad` and the bars right by `pad`.
        let data = "1".repeat(200);
        let code = code128::encode(&data, Code128Mode::Auto);
        let bw: u32 = code.widths().iter().map(|&w| w as u32 * 2).sum();
        let tw = (interpretation(2).char_advance * code.text.chars().count() as f32).ceil() as u32;
        let pad = (tw - bw) / 2;
        assert!(pad > 0);
        let c = content_of(code128(&data, Orientation::Normal, true));
        assert!(c.contains(&format!("1 0 0 1 {} 50 cm", 100 - pad)));
        assert!(c.contains(&format!("{pad} 0 4 80 re")));
    }

    #[test]
    fn code128_mode_d_line_is_font_0_sized_to_the_bars() {
        let mut instruction = code128("(10)A1", Orientation::Normal, true);
        if let DrawInstruction::DrawBarcode { mode, .. } = &mut instruction {
            *mode = Code128Mode::Gs1;
        }
        let code = code128::encode("(10)A1", Code128Mode::Gs1);
        let bw: u32 = code.widths().iter().map(|&w| w as u32 * 2).sum();
        let line = code128::Gs1Line::new(&code.text, bw, 80, false).unwrap();
        let c = content_of(instruction);
        // Bars unpadded at the origin, text at the line's size.
        assert!(c.contains("1 0 0 1 100 50 cm"));
        assert!(c.contains(&format!("{} 0 0 -{} ", line.em, line.em)), "{c}");
    }

    #[test]
    fn rotations_use_expected_matrices() {
        let code = code128::encode("12345678", Code128Mode::Auto);
        let w: u32 = code.widths().iter().map(|&m| m as u32 * 2).sum();
        let h = 80;
        let c = content_of(code128("12345678", Orientation::Rotated90, false));
        assert!(c.contains(&format!("0 1 -1 0 {} 50 cm", 100 + h)));
        let c = content_of(code128("12345678", Orientation::Rotated180, false));
        assert!(c.contains(&format!("-1 0 0 -1 {} {} cm", 100 + w, 50 + h)));
        let rotated270 = |bottom| DrawInstruction::DrawBarcode {
            x: 500, y: 600, bottom, right: false, data: "12345678".into(), orientation: Orientation::Rotated270,
            height: 80, module_width: 2, wide_bar_ratio: 3.0, print_interpretation_line: false,
            print_interpretation_line_above: false, check_digit: false, mode: Code128Mode::Auto,
        };
        // ^FO: the rotated bars start at the origin and run down and right of it.
        assert!(content_of(rotated270(false)).contains(&format!("0 -1 1 0 500 {} cm", 600 + w)));
        // ^FT: the unrotated bottom-left corner is the origin, so the bars end on it.
        assert!(content_of(rotated270(true)).contains(&format!("0 -1 1 0 {} 600 cm", 500 - h)));
        let mut inverted = code128("12345678", Orientation::Rotated180, false);
        if let DrawInstruction::DrawBarcode { bottom, .. } = &mut inverted {
            *bottom = true;
        }
        assert!(content_of(inverted).contains(&format!("-1 0 0 -1 100 {} cm", 50 + h)));
    }

    fn linear(kind: LinearKind, data: &str, line: bool) -> DrawInstruction {
        DrawInstruction::DrawLinear {
            x: 10, y: 20, bottom: false, right: false, kind, data: data.into(), orientation: Orientation::Normal, height: 50,
            module_width: 1, wide_bar_ratio: 3.0, print_interpretation_line: line,
            print_interpretation_line_above: false,
        }
    }

    #[test]
    fn code39_draws_bars_and_text() {
        let c = content_of(linear(LinearKind::Code39 { check_digit: false }, "A", false));
        // "*A*": 3 characters, 5 bars each.
        assert_eq!(count_op(&c, "re"), 15);
        assert_eq!(count_op(&c, "f"), 1);
        let c = content_of(linear(LinearKind::Code39 { check_digit: false }, "A", true));
        assert_eq!(count_op(&c, "Tj"), 1);
    }

    #[test]
    fn ean13_guards_extend_and_text_splits() {
        let c = content_of(linear(LinearKind::Ean13, "400638133393", true));
        // 3 guard patterns of 2/2/2 bars plus 12 digits of 2 bars each.
        assert_eq!(count_op(&c, "re"), 30);
        assert!(c.lines().any(|l| l.ends_with(" re") && !l.ends_with(" 50 re")), "guards are taller");
        // One text run per digit (Labelary's layout, `png_renderer::guarded_line`).
        assert_eq!(count_op(&c, "Tj"), 13);
    }

    #[test]
    fn invalid_data_draws_nothing() {
        assert!(content_of(linear(LinearKind::Ean13, "abc", true)).is_empty());
    }

    #[test]
    fn qr_sits_on_the_origin_and_merges_runs() {
        let c = content_of(DrawInstruction::DrawQrCode {
            x: 30, y: 40, data: "QA,hello".into(), orientation: Orientation::Normal,
            model: zpl_parser::command::QrModel::Model2, magnification: 4,
            error_correction: zpl_parser::command::QrErrorCorrection::Standard, mask: None,
            bottom: false,
            right: false,
        });
        assert!(c.contains("1 0 0 1 30 40 cm"));
        // Top-left finder pattern: first row starts with a 7-module dark run.
        assert!(c.contains("0 0 28 4 re"));
        let code = qr::encode("QA,hello", None).unwrap();
        let dark = (0..code.size).flat_map(|y| (0..code.size).map(move |x| (x, y))).filter(|&(x, y)| code.get(x, y)).count();
        assert!(count_op(&c, "re") < dark, "runs must be merged");
    }

    #[test]
    fn runs_finds_dark_spans() {
        assert_eq!(runs(&[1, 1, 0, 1, 0, 0, 1, 1, 1]), vec![(0, 2), (3, 1), (6, 3)]);
        assert!(runs(&[0, 0]).is_empty());
    }
}
