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
use png_renderer::code128;
use zpl_parser::command::{Code128Mode, LinearKind, Orientation};
use zpl_symbols::linear::{self, LinearSymbol, MsiCheck, Symbology};
use zpl_state_manager::DrawInstruction;

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    match instruction {
        DrawInstruction::DrawBarcode {
            x, y, bottom, data, orientation, height, module_width,
            print_interpretation_line, print_interpretation_line_above, mode, ..
        } => render_code128(
            ctx, (*x, *y, *bottom), data, *orientation, *height, *module_width,
            *print_interpretation_line, *print_interpretation_line_above, *mode,
        ),
        DrawInstruction::DrawLinear {
            x, y, bottom, kind, data, orientation, height, module_width, wide_bar_ratio,
            print_interpretation_line, print_interpretation_line_above,
        } => render_linear(
            ctx, (*x, *y, *bottom), *kind, data, *orientation, *height, *module_width, *wide_bar_ratio,
            *print_interpretation_line, *print_interpretation_line_above,
        ),
        DrawInstruction::DrawQrCode { x, y, data, magnification, mask, bottom, .. } => {
            render_qr_code(ctx, *x, *y, data, *magnification, *mask, *bottom)
        }
        DrawInstruction::DrawDataMatrix { .. }
        | DrawInstruction::DrawPdf417 { .. }
        | DrawInstruction::DrawAztec { .. } => matrix::render(ctx, instruction),
        _ => {}
    }
}

/// Transform mapping a local `w` x `h` frame onto the page the way the PNG renderer rotates
/// its temp image (`imageops::rotate*`) and overlays it at its (saturating) offset.
pub(crate) fn placement(orientation: Orientation, x: u32, y: u32, w: u32, h: u32) -> [f32; 6] {
    let (w_f, h_f) = (w as f32, h as f32);
    match orientation {
        Orientation::Normal => [1.0, 0.0, 0.0, 1.0, x as f32, y as f32],
        // rotate90 (clockwise): local (u, v) -> (h - v, u)
        Orientation::Rotated90 => [0.0, 1.0, -1.0, 0.0, x as f32 + h_f, y as f32],
        // rotate180: local (u, v) -> (w - u, h - v)
        Orientation::Rotated180 => [-1.0, 0.0, 0.0, -1.0, x as f32 + w_f, y as f32 + h_f],
        // rotate270 (counter-clockwise): local (u, v) -> (v, w - u); image is h wide, w tall
        Orientation::Rotated270 => {
            let ox = x.saturating_sub(h) as f32;
            let oy = y.saturating_sub(w) as f32;
            [0.0, -1.0, 1.0, 0.0, ox, oy + w_f]
        }
    }
}

/// Transform for a 1D symbol whose bars are the local box `bars` (x, y, w, h): ^FO puts the
/// rotated bars' top-left on (`x`, `y`), ^FT (`bottom`) the bars' bottom-left corner, rotated
/// with the symbol; the interpretation line follows the bars (as the PNG renderer's `place_bars`).
pub(crate) fn bars_placement(orientation: Orientation, (x, y, bottom): (u32, u32, bool), bars: [f32; 4]) -> [f32; 6] {
    let [bx, by, bw, bh] = bars;
    let (x, y) = (x as f32, y as f32);
    // Top-left of the rotated bars on the page.
    let (left, top) = match (bottom, orientation) {
        (false, _) | (true, Orientation::Rotated90) => (x, y),
        (true, Orientation::Normal) => (x, y - bh),
        (true, Orientation::Rotated180) => (x - bw, y),
        (true, Orientation::Rotated270) => (x - bh, y - bw),
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
fn line_gap(module: u32, above: bool, guards: bool) -> f32 {
    match (above, guards) {
        (true, _) => (8 + 2 * module) as f32,
        (false, true) => (2 * module) as f32,
        (false, false) => 6.0,
    }
}

/// How far EAN/UPC guard bars reach below the others when the digits are printed above.
const GUARD_EXTENSION_ABOVE: u32 = 13;

/// Interpretation line layout for a 1D symbol (all in dots).
struct Interpretation {
    char_advance: f32,
    digit_height: f32,
    size_x: f32,
    size_y: f32,
    descent: f32,
}

fn interpretation(bar_width: u32) -> Interpretation {
    let font = FontId::DejaVuMono;
    // Same constants as the PNG renderer (Labelary-compatible): no larger than at module 10.
    let bar_width = bar_width.min(10);
    let char_advance = bar_width as f32 * 143.0 / 24.0;
    let digit_height = (bar_width * 7) as f32;

    // DejaVu Sans Mono fallbacks (em units) in case metrics are unavailable.
    let adv_em = Some(text::char_advance(font, '0')).filter(|v| *v > 0.0).unwrap_or(1233.0 / 2048.0);
    let zero_h_em = Some(text::glyph_height(font, '0')).filter(|v| *v > 0.0).unwrap_or(1493.0 / 2048.0);
    let descent_em = Some(text::metrics(font).descent.abs()).filter(|v| *v > 0.0).unwrap_or(483.0 / 2048.0);

    let size_x = char_advance / adv_em;
    let size_y = digit_height / zero_h_em;
    Interpretation { char_advance, digit_height, size_x, size_y, descent: descent_em * size_y }
}

impl Interpretation {
    /// Height of the text and its gap to the bars, in whole dots.
    fn area(&self, gap: f32, above: bool) -> u32 {
        let below = if above { 0.0 } else { self.descent };
        (gap + self.digit_height + below).ceil() as u32
    }
}

#[allow(clippy::too_many_arguments)]
fn render_code128(
    ctx: &mut Ctx, origin: (u32, u32, bool), data: &str, orientation: Orientation, height: u32,
    module_width: u32, print_line: bool, line_above: bool, mode: Code128Mode,
) {
    let code = code128::encode(data, mode);
    let bar_width = module_width.max(1);
    let widths = code.widths();
    let barcode_width: u32 = widths.iter().map(|&w| w as u32 * bar_width).sum();

    let ip = interpretation(bar_width);
    let text_gap = line_gap(bar_width, line_above, false);
    let show_text = print_line && !code.text.is_empty();
    let n_chars = code.text.chars().count() as f32;
    let text_width = if show_text { (ip.char_advance * n_chars).ceil() as u32 } else { 0 };
    let text_area = if show_text { ip.area(text_gap, line_above) } else { 0 };

    let pad = text_width.saturating_sub(barcode_width) / 2;
    let (bars_y, baseline) = if line_above {
        (text_area, ip.digit_height)
    } else {
        (0, height as f32 + text_gap + ip.digit_height)
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
        let text_x = pad as f32 + (barcode_width as f32 - text_width as f32) / 2.0;
        ctx.content.save_state();
        ctx.content.transform(matrix);
        ctx.content.set_fill_gray(ctx.ink);
        text::show_text(ctx, FontId::DejaVuMono, &code.text, text_x, baseline, ip.size_x, ip.size_y);
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
/// interpretation line (DejaVu Sans Mono at the Code 128 proportions) goes below, or above
/// when requested. EAN/UPC guard bars extend halfway into the digits.
#[allow(clippy::too_many_arguments)]
fn render_linear(
    ctx: &mut Ctx, origin: (u32, u32, bool), kind: LinearKind, data: &str, orientation: Orientation, height: u32,
    module_width: u32, ratio: f32, print_line: bool, line_above: bool,
) {
    let Some(symbol) = encode_linear(kind, data) else { return };
    let module = module_width.max(1);
    let width = symbol.width_dots(module, ratio);

    let ip = interpretation(module);
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
        if line_above && symbol.has_guards {
            // Above the bars EAN/UPC digits are one line centred over the symbol.
            let text: String = symbol.text.iter().map(|seg| seg.text.as_str()).collect();
            let text_x = (width as f32 - ip.char_advance * text.chars().count() as f32) / 2.0;
            text::show_text(ctx, FontId::DejaVuMono, &text, text_x, baseline, ip.size_x, ip.size_y);
        } else {
            for seg in &symbol.text {
                let (start, end) = symbol.text_extent_dots(seg, module, ratio);
                let text_w = ip.char_advance * seg.text.chars().count() as f32;
                let text_x = (start + end - text_w) / 2.0;
                text::show_text(ctx, FontId::DejaVuMono, &seg.text, text_x, baseline, ip.size_x, ip.size_y);
            }
        }
        ctx.content.restore_state();
    }
}

/// Parse the `<ec><input mode>,` prefix of QR ^FD data.
fn parse_qr_data(data: &str) -> (qr_core::EcLevel, &str) {
    use qr_core::EcLevel;
    match data.get(..3) {
        Some(prefix) if prefix.ends_with(',') => {
            let ec = match &prefix[..1] {
                "H" => EcLevel::H,
                "Q" => EcLevel::Q,
                "L" => EcLevel::L,
                _ => EcLevel::M,
            };
            (ec, &data[3..])
        }
        _ => (EcLevel::M, data),
    }
}

fn render_qr_code(ctx: &mut Ctx, x: u32, y: u32, data: &str, magnification: u32, mask: Option<u32>, bottom: bool) {
    let (ec, payload) = parse_qr_data(data);
    let mut options = qr_encode::Options::new(ec);
    // ^BQ mask 0-7 forces a mask; anything else lets the encoder pick by ISO penalty score.
    options.mask = mask.filter(|&m| m <= 7).map(|m| m as u8);
    let Ok(code) = qr_encode::encode(payload.as_bytes(), options) else { return };
    let size = code.size();
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

    // Zebra printers draw QR codes 10 dots below the field origin; ^FT puts the bottom there.
    let top = if bottom { y.saturating_sub(size * magnification.max(1)) } else { y + 10 };
    let matrix = [1.0, 0.0, 0.0, 1.0, x as f32, top as f32];
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
            x: 100, y: 50, bottom: false, data: data.into(), orientation, height: 80, module_width: 2,
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
    fn rotations_use_expected_matrices() {
        let code = code128::encode("12345678", Code128Mode::Auto);
        let w: u32 = code.widths().iter().map(|&m| m as u32 * 2).sum();
        let h = 80;
        let c = content_of(code128("12345678", Orientation::Rotated90, false));
        assert!(c.contains(&format!("0 1 -1 0 {} 50 cm", 100 + h)));
        let c = content_of(code128("12345678", Orientation::Rotated180, false));
        assert!(c.contains(&format!("-1 0 0 -1 {} {} cm", 100 + w, 50 + h)));
        let rotated270 = |bottom| DrawInstruction::DrawBarcode {
            x: 500, y: 600, bottom, data: "12345678".into(), orientation: Orientation::Rotated270,
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
            x: 10, y: 20, bottom: false, kind, data: data.into(), orientation: Orientation::Normal, height: 50,
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
        assert_eq!(count_op(&c, "Tj"), 3);
    }

    #[test]
    fn invalid_data_draws_nothing() {
        assert!(content_of(linear(LinearKind::Ean13, "abc", true)).is_empty());
    }

    #[test]
    fn qr_parses_prefix_and_offsets() {
        assert_eq!(parse_qr_data("HA,hello"), (qr_core::EcLevel::H, "hello"));
        assert_eq!(parse_qr_data("hello"), (qr_core::EcLevel::M, "hello"));

        let c = content_of(DrawInstruction::DrawQrCode {
            x: 30, y: 40, data: "QA,hello".into(), orientation: Orientation::Normal,
            model: zpl_parser::command::QrModel::Model2, magnification: 4,
            error_correction: zpl_parser::command::QrErrorCorrection::Standard, mask: None,
            bottom: false,
        });
        assert!(c.contains("1 0 0 1 30 50 cm"));
        // Top-left finder pattern: first row starts with a 7-module dark run.
        assert!(c.contains("0 0 28 4 re"));
        let code = qr_encode::encode(b"hello", qr_encode::Options::new(qr_core::EcLevel::Q)).unwrap();
        let size = code.size();
        let dark = (0..size).flat_map(|y| (0..size).map(move |x| (x, y))).filter(|&(x, y)| code.get(x, y)).count();
        assert!(count_op(&c, "re") < dark, "runs must be merged");
    }

    #[test]
    fn runs_finds_dark_spans() {
        assert_eq!(runs(&[1, 1, 0, 1, 0, 0, 1, 1, 1]), vec![(0, 2), (3, 1), (6, 3)]);
        assert!(runs(&[0, 0]).is_empty());
    }
}
