//! `DrawSymbol`: stacked symbols (DataBar, composites, MicroPDF417, Codablock, Code 49, TLC39),
//! postal codes and MaxiCode. Geometry mirrors pdf-renderer's `symbols.rs`; shapes are drawn
//! into an unrotated frame, then rotated and placed like the other barcodes.

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{ImageBuffer, RgbImage};
use imageproc::drawing;
use zpl_parser::command::{Code49Interpretation, Orientation, PostalKind};
use zpl_state_manager::{DrawInstruction, SymbolSpec};
use zpl_symbols::{RowHeight, Stacked, composite, databar, maxicode, micropdf, postal, stacked};

use crate::{BLACK, PngRenderer, WHITE};

impl PngRenderer {
    pub(crate) fn render_symbol(&self, image: &mut RgbImage, instruction: &DrawInstruction) {
        let DrawInstruction::DrawSymbol { x, y, data, orientation, bottom, spec } = instruction else { return };
        let at = Place { x: *x, y: *y, orientation: *orientation, bottom: *bottom };
        let frame = match *spec {
            SymbolSpec::MaxiCode { mode, symbol_number, total_symbols } => {
                maxicode::encode(data, mode, symbol_number, total_symbols).ok().map(|s| maxicode_frame(&s, self.dpmm))
            }
            SymbolSpec::DataBar { kind, module_width, separator_height, height, segments_per_row } => {
                data_bar(kind, data, segments_per_row).map(|(symbol, composite)| {
                    // ^BR e only sizes the linear part of composites; plain DataBar rows have
                    // their symbology-defined heights (the row weights, in modules).
                    let bars = if composite { Bars::Shared(height) } else { Bars::Modules };
                    let heights = Heights { module: module_width, separator: Some(separator_height), bars };
                    stacked_frame(&symbol, &heights, None)
                })
            }
            SymbolSpec::MicroPdf417 { module_width, row_height, mode } => {
                micropdf::micro_pdf417(data.as_bytes(), mode).ok().map(|symbol| {
                    let heights = Heights { module: module_width, separator: None, bars: Bars::PerRow(row_height) };
                    stacked_frame(&symbol, &heights, None)
                })
            }
            SymbolSpec::Codablock { module_width, row_height, security, columns, rows, mode } => {
                let mode = match mode {
                    'A' => stacked::CodablockMode::A,
                    'E' => stacked::CodablockMode::E,
                    _ => stacked::CodablockMode::F,
                };
                stacked::codablock(data, mode, columns, rows, security).ok().map(|symbol| {
                    let heights = Heights { module: module_width, separator: None, bars: Bars::PerRow(row_height) };
                    stacked_frame(&symbol, &heights, None)
                })
            }
            SymbolSpec::Code49 { module_width, row_height, interpretation, start_mode } => {
                stacked::code49(data, start_mode).ok().map(|(symbol, hri)| {
                    let heights = Heights { module: module_width, separator: None, bars: Bars::PerRow(row_height) };
                    let above = match interpretation {
                        Code49Interpretation::None => None,
                        Code49Interpretation::Above => Some(true),
                        Code49Interpretation::Below => Some(false),
                    };
                    stacked_frame(&symbol, &heights, above.map(|a| (hri.as_str(), a)))
                })
            }
            SymbolSpec::Tlc39 { code39_module_width, code39_ratio, code39_height, micro_module_width, micro_row_height } => {
                micropdf::tlc39(data).ok().map(|symbol| {
                    tlc39_frame(&symbol, code39_module_width, code39_ratio, code39_height, micro_module_width, micro_row_height)
                })
            }
            SymbolSpec::Postal { kind, module_width, height, print_interpretation_line, print_above } => {
                let kind = match kind {
                    PostalKind::Postnet => postal::PostalKind::Postnet,
                    PostalKind::Planet => postal::PostalKind::Planet,
                    PostalKind::UspsImb => postal::PostalKind::UspsImb,
                };
                postal::encode(kind, data).ok().map(|symbol| {
                    postal_frame(&symbol, module_width, height, print_interpretation_line.then_some(print_above))
                })
            }
        };
        if let Some(frame) = frame {
            // ^FT: y is the bottom edge of the (unrotated) frame.
            let y = if at.bottom { at.y.saturating_sub(frame.height()) } else { at.y };
            self.place_symbol(image, &frame, at.x, y, at.orientation, 0, 0);
        }
    }
}

#[derive(Clone, Copy)]
struct Place {
    x: u32,
    y: u32,
    orientation: Orientation,
    bottom: bool,
}

/// ^BR types 1-6 are plain DataBar unless the data has a `|` composite separator; 7-12 are
/// always composite.
fn data_bar(kind: u8, data: &str, segments_per_row: u32) -> Option<(Stacked, bool)> {
    use databar::DataBarKind as K;
    let bar_kind = match kind {
        1 => Some(K::Omnidirectional),
        2 => Some(K::Truncated),
        3 => Some(K::Stacked),
        4 => Some(K::StackedOmnidirectional),
        5 => Some(K::Limited),
        6 => Some(K::Expanded),
        _ => None,
    };
    let linear = match (bar_kind, kind) {
        (Some(k), _) if !data.contains('|') => {
            return databar::encode(k, data, segments_per_row).ok().map(|b| (b.symbol, false));
        }
        (Some(k), _) => composite::CompositeLinear::DataBar(k),
        (None, 7) => composite::CompositeLinear::UpcA,
        (None, 8) => composite::CompositeLinear::UpcE,
        (None, 9) => composite::CompositeLinear::Ean13,
        (None, 10) => composite::CompositeLinear::Ean8,
        (None, 11) => composite::CompositeLinear::Gs1_128 { cc_c: false },
        _ => composite::CompositeLinear::Gs1_128 { cc_c: true },
    };
    composite::encode(linear, data, segments_per_row).ok().map(|c| (c.symbol, true))
}

enum Bars {
    /// `RowHeight::Bar` rows are `weight` modules tall.
    Modules,
    /// Every `RowHeight::Bar` row is this many dots tall.
    PerRow(u32),
    /// `RowHeight::Bar` rows split this many dots by weight.
    Shared(u32),
}

struct Heights {
    module: u32,
    /// Height of 1-module separator rows (^BR d); `None` = one module.
    separator: Option<u32>,
    bars: Bars,
}

impl Heights {
    fn rows(&self, symbol: &Stacked) -> Vec<u32> {
        let total_weight: u32 =
            symbol.rows.iter().map(|r| if let RowHeight::Bar { weight } = r.height { weight } else { 0 }).sum();
        symbol
            .rows
            .iter()
            .map(|r| match (r.height, &self.bars, self.separator) {
                (RowHeight::Modules(1), _, Some(separator)) => separator,
                (RowHeight::Modules(n), _, _) => n * self.module,
                (RowHeight::Bar { weight }, Bars::Modules, _) => weight * self.module,
                (RowHeight::Bar { .. }, Bars::PerRow(h), _) => *h,
                (RowHeight::Bar { weight }, Bars::Shared(h), _) => (h * weight).div_ceil(total_weight.max(1)),
            })
            .map(|h| h.max(1))
            .collect()
    }
}

/// White frame of at least 1 x 1 dots.
fn blank(w: u32, h: u32) -> RgbImage {
    ImageBuffer::from_pixel(w.max(1), h.max(1), WHITE)
}

/// Fills the dots covered by a rect whose edges are rounded to whole dots, clipped to `img`.
fn fill_rect(img: &mut RgbImage, x: f32, y: f32, w: f32, h: f32) {
    let clamp = |v: f32, max: u32| (v.round().max(0.0) as u32).min(max);
    let (x0, x1) = (clamp(x, img.width()), clamp(x + w, img.width()));
    let (y0, y1) = (clamp(y, img.height()), clamp(y + h, img.height()));
    for py in y0..y1 {
        for px in x0..x1 {
            img.put_pixel(px, py, BLACK);
        }
    }
}

/// Draws the dark runs of every row from `top` down; returns the bottom edge.
fn draw_rows(img: &mut RgbImage, symbol: &Stacked, heights: &Heights, top: u32) -> u32 {
    let module = heights.module.max(1);
    let mut y = top;
    for (row, h) in symbol.rows.iter().zip(heights.rows(symbol)) {
        let mut col = 0;
        while col < row.modules.len() {
            if !row.modules[col] {
                col += 1;
                continue;
            }
            let start = col;
            while col < row.modules.len() && row.modules[col] {
                col += 1;
            }
            fill_rect(img, (start as u32 * module) as f32, y as f32, ((col - start) as u32 * module) as f32, h as f32);
        }
        y += h;
    }
    y
}

fn stacked_size(symbol: &Stacked, heights: &Heights) -> (u32, u32) {
    (symbol.width * heights.module.max(1), heights.rows(symbol).iter().sum())
}

/// Stacked symbol, optionally with an interpretation line above (`true`) or below it.
fn stacked_frame(symbol: &Stacked, heights: &Heights, text: Option<(&str, bool)>) -> RgbImage {
    let size = (heights.module.max(1) * 9) as f32;
    let line = text.map(|_| (size * 1.3).ceil() as u32).unwrap_or(0);
    let above = matches!(text, Some((_, true)));
    let (w, h) = stacked_size(symbol, heights);
    let mut img = blank(w, h + line);
    draw_rows(&mut img, symbol, heights, if above { line } else { 0 });
    if let Some((text, above)) = text {
        let baseline = if above { size } else { h as f32 + size * 1.1 };
        centered_text(&mut img, text, w as f32 / 2.0, baseline, size);
    }
    img
}

/// Monospaced (DejaVu Sans Mono) text of em size `size`, centred on `cx`, one '0' advance per
/// character, like pdf-renderer's interpretation lines.
fn centered_text(img: &mut RgbImage, text: &str, cx: f32, baseline: f32, size: f32) {
    if text.is_empty() {
        return;
    }
    static MONO: &[u8] = include_bytes!("../fonts/DejaVuSansMono.ttf");
    let Ok(font) = FontRef::try_from_slice(MONO) else { return };
    let upem = font.units_per_em().unwrap_or(2048.0);
    // ab_glyph's PxScale is ascent - descent, not the em.
    let scale = PxScale::from(size * font.height_unscaled() / upem);
    let scaled = font.as_scaled(scale);
    let advance = scaled.h_advance(font.glyph_id('0'));
    let x = cx - advance * text.chars().count() as f32 / 2.0;
    let top = (baseline - scaled.ascent()).round() as i32;
    let mut buf = [0u8; 4];
    for (i, c) in text.chars().enumerate() {
        let cx = (x + i as f32 * advance).round() as i32;
        drawing::draw_text_mut(img, BLACK, cx, top, scale, &font, c.encode_utf8(&mut buf));
    }
}

/// TLC39: the MicroPDF417 sits directly above the Code 39, left aligned, one micro row apart.
fn tlc39_frame(
    symbol: &micropdf::Tlc39,
    module: u32,
    ratio: f32,
    code39_height: u32,
    micro_module: u32,
    micro_row: u32,
) -> RgbImage {
    let micro_heights = Heights { module: micro_module, separator: None, bars: Bars::PerRow(micro_row) };
    let (micro_w, micro_h) =
        if symbol.micro.rows.is_empty() { (0, 0) } else { stacked_size(&symbol.micro, &micro_heights) };
    let top = micro_h + if micro_h > 0 { micro_row } else { 0 };
    let module = module.max(1);
    let code39_w: u32 = symbol.code39.element_dots(module, ratio).map(|(_, dots)| dots).sum();
    let mut img = blank(code39_w.max(micro_w), top + code39_height);
    if micro_h > 0 {
        draw_rows(&mut img, &symbol.micro, &micro_heights, 0);
    }
    let mut x = 0;
    for (element, dots) in symbol.code39.element_dots(module, ratio) {
        if element.bar {
            fill_rect(&mut img, x as f32, top as f32, dots as f32, code39_height as f32);
        }
        x += dots;
    }
    img
}

/// Height-modulated bars at the USPS pitch; optional interpretation line above (`Some(true)`)
/// or below.
fn postal_frame(symbol: &postal::PostalSymbol, module: u32, height: u32, text: Option<bool>) -> RgbImage {
    let module = module.max(1) as f32;
    let bar_w = (symbol.bar_width * module).max(1.0);
    let pitch = symbol.pitch * module;
    let size = module * 5.0;
    let line = if text.is_some() { (size * 1.4).ceil() } else { 0.0 };
    let top = if text == Some(true) { line } else { 0.0 };
    let h = height as f32;
    let width = ((symbol.bars.len().max(1) - 1) as f32 * pitch + bar_w).ceil() as u32;
    let mut img = blank(width, (h + line) as u32);
    for (i, bar) in symbol.bars.iter().enumerate() {
        fill_rect(&mut img, (i as f32 * pitch).round(), top + bar.top * h, bar_w, (bar.bottom - bar.top) * h);
    }
    if let (Some(above), Some(hri)) = (text, &symbol.hri) {
        let baseline = if above { size } else { h + size * 1.2 };
        centered_text(&mut img, hri, width as f32 / 2.0, baseline, size);
    }
    img
}

/// MaxiCode at its fixed physical size on a `dpmm` printhead: pointy-top hexagons plus the
/// bullseye rings, sampled at dot centres.
fn maxicode_frame(symbol: &maxicode::MaxiCode, dpmm: u32) -> RgbImage {
    let g = symbol.geometry(dpmm as f32);
    let mut img = blank(g.width.ceil() as u32, g.height.ceil() as u32);
    let (w, h) = (img.width() as i32, img.height() as i32);
    let r = g.hex_radius;
    let half_w = r * 0.866_025_4;
    let inv_sqrt3 = 1.0 / 3f32.sqrt();
    for &(cx, cy) in &g.hexagons {
        let (x0, x1) = (((cx - half_w).floor() as i32).max(0), ((cx + half_w).ceil() as i32).min(w));
        let (y0, y1) = (((cy - r).floor() as i32).max(0), ((cy + r).ceil() as i32).min(h));
        for py in y0..y1 {
            let dy = (py as f32 + 0.5 - cy).abs();
            for px in x0..x1 {
                let dx = (px as f32 + 0.5 - cx).abs();
                if dx <= half_w && dy + dx * inv_sqrt3 <= r {
                    img.put_pixel(px as u32, py as u32, BLACK);
                }
            }
        }
    }
    let (cx, cy) = g.center;
    for &(outer, inner) in &g.rings {
        let (x0, x1) = (((cx - outer).floor() as i32).max(0), ((cx + outer).ceil() as i32).min(w));
        let (y0, y1) = (((cy - outer).floor() as i32).max(0), ((cy + outer).ceil() as i32).min(h));
        for py in y0..y1 {
            let dy = py as f32 + 0.5 - cy;
            for px in x0..x1 {
                let dx = px as f32 + 0.5 - cx;
                let d2 = dx * dx + dy * dy;
                if d2 <= outer * outer && d2 > inner * inner {
                    img.put_pixel(px as u32, py as u32, BLACK);
                }
            }
        }
    }
    img
}

#[cfg(test)]
mod tests {
    use super::*;

    fn black(image: &RgbImage, x: u32, y: u32) -> bool {
        image.get_pixel(x, y)[0] < 128
    }

    fn dark_bounds(image: &RgbImage) -> Option<(u32, u32, u32, u32)> {
        let mut b: Option<(u32, u32, u32, u32)> = None;
        for (x, y, p) in image.enumerate_pixels() {
            if p[0] < 128 {
                b = Some(match b {
                    None => (x, y, x, y),
                    Some((x0, y0, x1, y1)) => (x0.min(x), y0.min(y), x1.max(x), y1.max(y)),
                });
            }
        }
        b
    }

    fn symbol(x: u32, y: u32, data: &str, orientation: Orientation, bottom: bool, spec: SymbolSpec) -> DrawInstruction {
        DrawInstruction::DrawSymbol { x, y, data: data.into(), orientation, bottom, spec }
    }

    #[test]
    fn databar_omni_rows_are_module_weighted() {
        let spec = SymbolSpec::DataBar { kind: 1, module_width: 2, separator_height: 1, height: 50, segments_per_row: 22 };
        let image = PngRenderer::new(400, 200).render(&[symbol(10, 10, "0101234567890", Orientation::Normal, false, spec)]);
        let (x0, y0, x1, y1) = dark_bounds(&image).unwrap();
        // 96 modules (light guard modules at both ends) by 33 modules, 2 dots per module.
        assert_eq!(y0, 10);
        assert!(x0 >= 10 && x1 < 10 + 192);
        assert_eq!(y1 - y0 + 1, 66);
    }

    #[test]
    fn bottom_anchor_and_rotation_keep_size() {
        let spec = SymbolSpec::MicroPdf417 { module_width: 2, row_height: 6, mode: 0 };
        let top = PngRenderer::new(400, 400).render(&[symbol(50, 50, "Hello", Orientation::Normal, false, spec.clone())]);
        let (_, y0, _, y1) = dark_bounds(&top).unwrap();
        let h = y1 - y0 + 1;
        let bottom = PngRenderer::new(400, 400).render(&[symbol(50, 300, "Hello", Orientation::Normal, true, spec.clone())]);
        assert_eq!(dark_bounds(&bottom).unwrap().3, 299);
        let rotated = PngRenderer::new(400, 400).render(&[symbol(50, 50, "Hello", Orientation::Rotated90, false, spec)]);
        let (rx0, _, rx1, _) = dark_bounds(&rotated).unwrap();
        assert_eq!(rx1 - rx0 + 1, h);
    }

    #[test]
    fn maxicode_has_light_bullseye_centre_and_dark_ring() {
        let spec = SymbolSpec::MaxiCode { mode: 4, symbol_number: 1, total_symbols: 1 };
        let image = PngRenderer::new(300, 300).render(&[symbol(0, 0, "Hello MaxiCode", Orientation::Normal, false, spec)]);
        let code = maxicode::encode("Hello MaxiCode", 4, 1, 1).unwrap();
        let g = code.geometry(8.0);
        let (cx, cy) = g.center;
        assert!(!black(&image, cx as u32, cy as u32));
        let (outer, inner) = g.rings[0];
        let r = (outer + inner) / 2.0;
        assert!(black(&image, (cx + r) as u32, cy as u32));
        let (x0, y0, x1, y1) = dark_bounds(&image).unwrap();
        assert!(x1 - x0 < g.width.ceil() as u32 && y1 - y0 < g.height.ceil() as u32);
        assert!(x0 < 8 && y0 < 8);
    }

    #[test]
    fn postal_bars_follow_pitch_with_text_below() {
        let spec = SymbolSpec::Postal {
            kind: PostalKind::Postnet, module_width: 2, height: 40, print_interpretation_line: true, print_above: false,
        };
        let image = PngRenderer::new(400, 100).render(&[symbol(0, 0, "12345", Orientation::Normal, false, spec)]);
        // First bar is the full-height frame bar; the next starts at round(pitch * 2).
        assert!(black(&image, 0, 1) && black(&image, 1, 39));
        assert!(!black(&image, 2, 39) && black(&image, 5, 39));
        // Interpretation text below the bars.
        assert!((0..400).any(|x| (41..60).any(|y| black(&image, x, y))));
    }

    #[test]
    fn code49_text_above_shifts_bars_down() {
        let spec = |interpretation| SymbolSpec::Code49 { module_width: 2, row_height: 6, interpretation, start_mode: None };
        let none = PngRenderer::new(300, 200)
            .render(&[symbol(0, 0, "CODE49", Orientation::Normal, false, spec(Code49Interpretation::None))]);
        let above = PngRenderer::new(300, 200)
            .render(&[symbol(0, 0, "CODE49", Orientation::Normal, false, spec(Code49Interpretation::Above))]);
        let (_, _, _, none_bottom) = dark_bounds(&none).unwrap();
        let (_, _, _, above_bottom) = dark_bounds(&above).unwrap();
        assert_eq!(above_bottom, none_bottom + (18.0f32 * 1.3).ceil() as u32);
    }

    #[test]
    fn tlc39_micro_above_code39() {
        let spec = SymbolSpec::Tlc39 {
            code39_module_width: 2, code39_ratio: 2.0, code39_height: 40, micro_module_width: 2, micro_row_height: 4,
        };
        let image = PngRenderer::new(600, 200).render(&[symbol(0, 0, "123456,ABCd1234", Orientation::Normal, false, spec)]);
        let (x0, y0, _, y1) = dark_bounds(&image).unwrap();
        assert_eq!((x0, y0), (0, 0));
        // Code 39 occupies the bottom 40 dots, below a one-row gap.
        assert!(black(&image, 0, y1) && black(&image, 0, y1 - 39));
        assert!((0..600).all(|x| !black(&image, x, y1 - 40)));
        assert!((0..600).any(|x| black(&image, x, y1 - 44)));
    }
}
