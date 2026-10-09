//! Barcodes (Code 128, Code 39) and QR codes as vector paths.
//!
//! Bars and QR modules are emitted as filled rectangles (`re` ... `f`). Geometry and placement
//! mirror `png_renderer`'s `render_code128`, `render_code39` and `render_qr_code`: each symbol
//! is laid out in a local, unrotated frame (origin top-left, y down, in dots) exactly like the
//! PNG renderer's temporary image, and a `cm` maps that frame to where the PNG renderer places
//! the rotated image.

use crate::text;
use crate::{Ctx, FontId};
use png_renderer::code128;
use zpl_parser::command::{Code128Mode, Orientation};
use zpl_state_manager::DrawInstruction;

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    match instruction {
        DrawInstruction::DrawBarcode {
            x, y, data, orientation, height, module_width,
            print_interpretation_line, print_interpretation_line_above, mode, ..
        } => render_code128(
            ctx, *x, *y, data, *orientation, *height, *module_width,
            *print_interpretation_line, *print_interpretation_line_above, *mode,
        ),
        DrawInstruction::DrawCode39 { x, y, data, orientation, height, module_width, .. } => {
            render_code39(ctx, *x, *y, data, *orientation, *height, *module_width)
        }
        DrawInstruction::DrawQrCode { x, y, data, magnification, mask, .. } => {
            render_qr_code(ctx, *x, *y, data, *magnification, *mask)
        }
        _ => {}
    }
}

/// Transform mapping a local `w` x `h` frame onto the page the way the PNG renderer rotates
/// its temp image (`imageops::rotate*`) and overlays it at its (saturating) offset.
/// `pad` is the Code 128 interpretation-line padding the PNG renderer subtracts from the origin.
fn placement(orientation: Orientation, x: u32, y: u32, w: u32, h: u32, pad: u32) -> [f32; 6] {
    let (w_f, h_f) = (w as f32, h as f32);
    match orientation {
        Orientation::Normal => [1.0, 0.0, 0.0, 1.0, x.saturating_sub(pad) as f32, y as f32],
        // rotate90 (clockwise): local (u, v) -> (h - v, u)
        Orientation::Rotated90 => [0.0, 1.0, -1.0, 0.0, x as f32 + h_f, y.saturating_sub(pad) as f32],
        // rotate180: local (u, v) -> (w - u, h - v)
        Orientation::Rotated180 => [-1.0, 0.0, 0.0, -1.0, x.saturating_sub(pad) as f32 + w_f, y as f32 + h_f],
        // rotate270 (counter-clockwise): local (u, v) -> (v, w - u); image is h wide, w tall
        Orientation::Rotated270 => {
            let ox = x.saturating_sub(h) as f32;
            let oy = y.saturating_sub(w).saturating_sub(pad) as f32;
            [0.0, -1.0, 1.0, 0.0, ox, oy + w_f]
        }
    }
}

/// Fill `(x, y, w, h)` rects in black inside `matrix`, wrapped in `q`/`Q`.
fn fill_rects(ctx: &mut Ctx, matrix: [f32; 6], rects: &[(f32, f32, f32, f32)]) {
    if rects.is_empty() {
        return;
    }
    ctx.content.save_state();
    ctx.content.transform(matrix);
    ctx.content.set_fill_gray(0.0);
    for &(x, y, w, h) in rects {
        ctx.content.rect(x, y, w, h);
    }
    ctx.content.fill_nonzero();
    ctx.content.restore_state();
}

/// Interpretation line layout for a Code 128 symbol (all in dots).
struct Interpretation {
    char_advance: f32,
    digit_height: f32,
    size_x: f32,
    size_y: f32,
    descent: f32,
}

fn interpretation(bar_width: u32) -> Interpretation {
    let font = FontId::DejaVuMono;
    // Same constants as the PNG renderer (Labelary-compatible).
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

#[allow(clippy::too_many_arguments)]
fn render_code128(
    ctx: &mut Ctx, x: u32, y: u32, data: &str, orientation: Orientation, height: u32,
    module_width: u32, print_line: bool, line_above: bool, mode: Code128Mode,
) {
    let code = code128::encode(data, mode);
    let bar_width = module_width.max(1);
    let widths = code.widths();
    let barcode_width: u32 = widths.iter().map(|&w| w as u32 * bar_width).sum();

    let ip = interpretation(bar_width);
    let text_gap = (bar_width * 2) as f32;
    let show_text = print_line && !code.text.is_empty();
    let n_chars = code.text.chars().count() as f32;
    let text_width = if show_text { (ip.char_advance * n_chars).ceil() as u32 } else { 0 };
    let text_area = if show_text { (text_gap + ip.digit_height + ip.descent).ceil() as u32 } else { 0 };

    let pad = text_width.saturating_sub(barcode_width) / 2;
    let total_width = (barcode_width + 2 * pad).max(1);
    let total_height = (height + text_area).max(1);
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

    let matrix = placement(orientation, x, y, total_width, total_height, pad);
    fill_rects(ctx, matrix, &rects);

    if show_text {
        let text_x = pad as f32 + (barcode_width as f32 - text_width as f32) / 2.0;
        ctx.content.save_state();
        ctx.content.transform(matrix);
        ctx.content.set_fill_gray(0.0);
        text::show_text(ctx, FontId::DejaVuMono, &code.text, text_x, baseline, ip.size_x, ip.size_y);
        ctx.content.restore_state();
    }
}

fn render_code39(ctx: &mut Ctx, x: u32, y: u32, data: &str, orientation: Orientation, height: u32, module_width: u32) {
    use barcoders::sym::code39::Code39;

    let Ok(barcode) = Code39::new(data) else { return };
    let encoded = barcode.encode();
    let bar_width = module_width.max(1);
    let barcode_width = encoded.len() as u32 * bar_width;

    let rects: Vec<_> = runs(&encoded)
        .into_iter()
        .filter(|_| height > 0)
        .map(|(start, len)| ((start as u32 * bar_width) as f32, 0.0, (len as u32 * bar_width) as f32, height as f32))
        .collect();

    let matrix = placement(orientation, x, y, barcode_width, height, 0);
    fill_rects(ctx, matrix, &rects);
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

fn render_qr_code(ctx: &mut Ctx, x: u32, y: u32, data: &str, magnification: u32, mask: Option<u32>) {
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

    // Zebra printers draw QR codes 10 dots below the field origin.
    let matrix = [1.0, 0.0, 0.0, 1.0, x as f32, (y + 10) as f32];
    fill_rects(ctx, matrix, &rects);
}

/// Runs of consecutive `1`s as `(start, len)`.
fn runs(bits: &[u8]) -> Vec<(usize, usize)> {
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
    use pdf_writer::Content;

    fn new_ctx() -> Ctx {
        Ctx {
            content: Content::new(),
            fonts: Default::default(),
            images: Vec::new(),
            uses_difference: false,
        }
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
            x: 100, y: 50, data: data.into(), orientation, height: 80, module_width: 2,
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
        let c = content_of(DrawInstruction::DrawBarcode {
            x: 500, y: 600, data: "12345678".into(), orientation: Orientation::Rotated270,
            height: 80, module_width: 2, wide_bar_ratio: 3.0, print_interpretation_line: false,
            print_interpretation_line_above: false, check_digit: false, mode: Code128Mode::Auto,
        });
        assert!(c.contains(&format!("0 -1 1 0 {} {} cm", 500 - h, 600)));
    }

    #[test]
    fn code39_merges_adjacent_modules() {
        let c = content_of(DrawInstruction::DrawCode39 {
            x: 10, y: 20, data: "A".into(), orientation: Orientation::Normal, check_digit: false,
            height: 50, module_width: 1, print_interpretation_line: false,
            print_interpretation_line_above: false,
        });
        // "*A*": 3 characters, 5 bars each.
        assert_eq!(count_op(&c, "re"), 15);
        assert_eq!(count_op(&c, "f"), 1);
    }

    #[test]
    fn qr_parses_prefix_and_offsets() {
        assert_eq!(parse_qr_data("HA,hello"), (qr_core::EcLevel::H, "hello"));
        assert_eq!(parse_qr_data("hello"), (qr_core::EcLevel::M, "hello"));

        let c = content_of(DrawInstruction::DrawQrCode {
            x: 30, y: 40, data: "QA,hello".into(), orientation: Orientation::Normal,
            model: zpl_parser::command::QrModel::Model2, magnification: 4,
            error_correction: zpl_parser::command::QrErrorCorrection::Standard, mask: None,
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
