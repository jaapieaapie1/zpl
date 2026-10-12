//! `DrawSymbol`: stacked symbols (DataBar, composites, MicroPDF417, Codablock, Code 49, TLC39),
//! postal codes and MaxiCode.

use crate::barcodes::{fill_rects, placement, runs};
use crate::text;
use crate::{Ctx, FontId};
use zpl_parser::command::{Code49Interpretation, Orientation, PostalKind};
use zpl_state_manager::{DrawInstruction, SymbolSpec};
use zpl_symbols::{RowHeight, Stacked, composite, databar, maxicode, micropdf, postal, stacked};

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    let DrawInstruction::DrawSymbol { x, y, data, orientation, bottom, spec } = instruction else { return };
    let at = Place { x: *x, y: *y, orientation: *orientation, bottom: *bottom };
    match *spec {
        SymbolSpec::MaxiCode { mode, symbol_number, total_symbols } => {
            if let Ok(symbol) = maxicode::encode(data, mode, symbol_number, total_symbols) {
                draw_maxicode(ctx, &symbol, at);
            }
        }
        SymbolSpec::DataBar { kind, module_width, separator_height, height, segments_per_row } => {
            if let Some((symbol, composite)) = data_bar(kind, data, segments_per_row) {
                // ^BR e only sizes the linear part of composites; plain DataBar rows have their
                // symbology-defined heights (the row weights, in modules).
                let bars = if composite { Bars::Shared(height) } else { Bars::Modules };
                let heights = Heights { module: module_width, separator: Some(separator_height), bars };
                draw_stacked(ctx, &symbol, at, &heights);
            }
        }
        SymbolSpec::MicroPdf417 { module_width, row_height, mode } => {
            if let Ok(symbol) = micropdf::micro_pdf417(data.as_bytes(), mode) {
                let heights = Heights { module: module_width, separator: None, bars: Bars::PerRow(row_height) };
                draw_stacked(ctx, &symbol, at, &heights);
            }
        }
        SymbolSpec::Codablock { module_width, row_height, security, columns, rows, mode } => {
            let mode = match mode {
                'A' => stacked::CodablockMode::A,
                'E' => stacked::CodablockMode::E,
                _ => stacked::CodablockMode::F,
            };
            if let Ok(symbol) = stacked::codablock(data, mode, columns, rows, security) {
                let heights = Heights { module: module_width, separator: None, bars: Bars::PerRow(row_height) };
                draw_stacked(ctx, &symbol, at, &heights);
            }
        }
        SymbolSpec::Code49 { module_width, row_height, interpretation, start_mode } => {
            if let Ok((symbol, hri)) = stacked::code49(data, start_mode) {
                let heights = Heights { module: module_width, separator: None, bars: Bars::PerRow(row_height) };
                let above = match interpretation {
                    Code49Interpretation::None => None,
                    Code49Interpretation::Above => Some(true),
                    Code49Interpretation::Below => Some(false),
                };
                draw_stacked_with_text(ctx, &symbol, at, &heights, above.map(|a| (hri.as_str(), a)));
            }
        }
        SymbolSpec::Tlc39 { code39_module_width, code39_ratio, code39_height, micro_module_width, micro_row_height } => {
            if let Ok(symbol) = micropdf::tlc39(data) {
                draw_tlc39(ctx, &symbol, at, code39_module_width, code39_ratio, code39_height, micro_module_width, micro_row_height);
            }
        }
        SymbolSpec::Postal { kind, module_width, height, print_interpretation_line, print_above } => {
            let kind = match kind {
                PostalKind::Postnet => postal::PostalKind::Postnet,
                PostalKind::Planet => postal::PostalKind::Planet,
                PostalKind::UspsImb => postal::PostalKind::UspsImb,
            };
            if let Ok(symbol) = postal::encode(kind, data) {
                let text = print_interpretation_line.then_some(print_above);
                draw_postal(ctx, &symbol, at, module_width, height, text);
            }
        }
    }
}

/// `(x, y, width, height)` in dots.
type Rect = (f32, f32, f32, f32);

#[derive(Clone, Copy)]
struct Place {
    x: u32,
    y: u32,
    orientation: Orientation,
    /// ^FT: `y` is the bottom edge.
    bottom: bool,
}

impl Place {
    /// Transform for a local `w` x `h` frame (y down) at this place.
    fn matrix(&self, w: u32, h: u32) -> [f32; 6] {
        let y = if self.bottom { self.y.saturating_sub(h) } else { self.y };
        placement(self.orientation, self.x, y, w.max(1), h.max(1))
    }
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
            .map(|r| match (r.height, &self.bars) {
                (RowHeight::Modules(1), _) if self.separator.is_some() => self.separator.unwrap_or(1),
                (RowHeight::Modules(n), _) => n * self.module,
                (RowHeight::Bar { weight }, Bars::Modules) => weight * self.module,
                (RowHeight::Bar { .. }, Bars::PerRow(h)) => *h,
                (RowHeight::Bar { weight }, Bars::Shared(h)) => (h * weight).div_ceil(total_weight.max(1)),
            })
            .map(|h| h.max(1))
            .collect()
    }
}

/// Dark runs of every row as rects in a local frame; returns them with the frame size.
fn stacked_rects(symbol: &Stacked, heights: &Heights, top: u32) -> (Vec<Rect>, u32, u32) {
    let module = heights.module.max(1) as f32;
    let mut rects = Vec::new();
    let mut y = top;
    let mut bits = Vec::with_capacity(symbol.width as usize);
    for (row, h) in symbol.rows.iter().zip(heights.rows(symbol)) {
        bits.clear();
        bits.extend(row.modules.iter().map(|&m| m as u8));
        for (start, len) in runs(&bits) {
            rects.push((start as f32 * module, y as f32, len as f32 * module, h as f32));
        }
        y += h;
    }
    (rects, symbol.width * heights.module.max(1), y)
}

fn draw_stacked(ctx: &mut Ctx, symbol: &Stacked, at: Place, heights: &Heights) {
    draw_stacked_with_text(ctx, symbol, at, heights, None);
}

/// Draws `symbol`, optionally with an interpretation line above (`true`) or below it.
fn draw_stacked_with_text(ctx: &mut Ctx, symbol: &Stacked, at: Place, heights: &Heights, text: Option<(&str, bool)>) {
    let size = (heights.module.max(1) * 9) as f32;
    let line = text.map(|_| (size * 1.3).ceil() as u32).unwrap_or(0);
    let above = matches!(text, Some((_, true)));
    let (rects, w, h) = stacked_rects(symbol, heights, if above { line } else { 0 });
    let matrix = at.matrix(w, h + if above { 0 } else { line });
    fill_rects(ctx, matrix, &rects);
    if let Some((text, above)) = text {
        let baseline = if above { size } else { h as f32 + size * 1.1 };
        centered_text(ctx, matrix, text, w as f32 / 2.0, baseline, size);
    }
}

/// Monospaced interpretation-line text centred on `cx` in the frame of `matrix`.
fn centered_text(ctx: &mut Ctx, matrix: [f32; 6], text: &str, cx: f32, baseline: f32, size: f32) {
    if text.is_empty() {
        return;
    }
    let advance = text::char_advance(FontId::DejaVuMono, '0') * size;
    let x = cx - advance * text.chars().count() as f32 / 2.0;
    ctx.content.save_state();
    ctx.content.transform(matrix);
    ctx.content.set_fill_gray(ctx.ink);
    text::show_text(ctx, FontId::DejaVuMono, text, x, baseline, size, size);
    ctx.content.restore_state();
}

/// TLC39: the MicroPDF417 sits directly above the Code 39, left aligned, one micro row apart.
#[allow(clippy::too_many_arguments)]
fn draw_tlc39(
    ctx: &mut Ctx,
    symbol: &micropdf::Tlc39,
    at: Place,
    module: u32,
    ratio: f32,
    code39_height: u32,
    micro_module: u32,
    micro_row: u32,
) {
    let micro_heights = Heights { module: micro_module, separator: None, bars: Bars::PerRow(micro_row) };
    let (mut rects, micro_w, micro_h) = if symbol.micro.rows.is_empty() {
        (Vec::new(), 0, 0)
    } else {
        stacked_rects(&symbol.micro, &micro_heights, 0)
    };
    let gap = if micro_h > 0 { micro_row } else { 0 };
    let top = micro_h + gap;
    let module = module.max(1);
    let mut x = 0;
    for (element, dots) in symbol.code39.element_dots(module, ratio) {
        if element.bar {
            rects.push((x as f32, top as f32, dots as f32, code39_height as f32));
        }
        x += dots;
    }
    let matrix = at.matrix(x.max(micro_w), top + code39_height);
    fill_rects(ctx, matrix, &rects);
}

fn draw_postal(ctx: &mut Ctx, symbol: &postal::PostalSymbol, at: Place, module: u32, height: u32, text: Option<bool>) {
    let module = module.max(1) as f32;
    let bar_w = (symbol.bar_width * module).max(1.0);
    let pitch = symbol.pitch * module;
    let size = module * 5.0;
    let line = if text.is_some() { (size * 1.4).ceil() } else { 0.0 };
    let top = if text == Some(true) { line } else { 0.0 };
    let h = height as f32;
    let rects: Vec<_> = symbol
        .bars
        .iter()
        .enumerate()
        .map(|(i, bar)| ((i as f32 * pitch).round(), top + bar.top * h, bar_w, (bar.bottom - bar.top) * h))
        .collect();
    let width = ((symbol.bars.len().max(1) - 1) as f32 * pitch + bar_w).ceil() as u32;
    let matrix = at.matrix(width, (h + line) as u32);
    fill_rects(ctx, matrix, &rects);
    if let (Some(above), Some(hri)) = (text, &symbol.hri) {
        let baseline = if above { size } else { h + size * 1.2 };
        centered_text(ctx, matrix, hri, width as f32 / 2.0, baseline, size);
    }
}

/// MaxiCode at its fixed physical size on the printhead: pointy-top hexagons plus the
/// bullseye rings.
fn draw_maxicode(ctx: &mut Ctx, symbol: &maxicode::MaxiCode, at: Place) {
    let g = symbol.geometry(ctx.dpmm as f32);
    let matrix = at.matrix(g.width.ceil() as u32, g.height.ceil() as u32);
    ctx.content.save_state();
    ctx.content.transform(matrix);
    ctx.content.set_fill_gray(ctx.ink);
    let r = g.hex_radius;
    let (dx, dy) = (r * 0.866_025_4, r / 2.0);
    for &(cx, cy) in &g.hexagons {
        let c = &mut ctx.content;
        c.move_to(cx, cy - r);
        c.line_to(cx + dx, cy - dy);
        c.line_to(cx + dx, cy + dy);
        c.line_to(cx, cy + r);
        c.line_to(cx - dx, cy + dy);
        c.line_to(cx - dx, cy - dy);
        c.close_path();
    }
    ctx.content.fill_nonzero();
    let (cx, cy) = g.center;
    for &(outer, inner) in &g.rings {
        circle(ctx, cx, cy, outer);
        if inner > 0.0 {
            circle(ctx, cx, cy, inner);
        }
        ctx.content.fill_even_odd();
    }
    ctx.content.restore_state();
}

fn circle(ctx: &mut Ctx, cx: f32, cy: f32, r: f32) {
    const K: f32 = 0.552_284_8;
    let k = r * K;
    let c = &mut ctx.content;
    c.move_to(cx + r, cy);
    c.cubic_to(cx + r, cy + k, cx + k, cy + r, cx, cy + r);
    c.cubic_to(cx - k, cy + r, cx - r, cy + k, cx - r, cy);
    c.cubic_to(cx - r, cy - k, cx - k, cy - r, cx, cy - r);
    c.cubic_to(cx + k, cy - r, cx + r, cy - k, cx + r, cy);
    c.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn content(spec: SymbolSpec, data: &str) -> String {
        let mut ctx = Ctx::new();
        let ins = DrawInstruction::DrawSymbol {
            x: 10,
            y: 20,
            data: data.into(),
            orientation: Orientation::Normal,
            bottom: false,
            spec,
        };
        render(&mut ctx, &ins);
        String::from_utf8(ctx.content.finish().to_vec()).unwrap()
    }

    #[test]
    fn every_symbol_draws() {
        let specs = [
            (SymbolSpec::MaxiCode { mode: 4, symbol_number: 1, total_symbols: 1 }, "Hello"),
            (SymbolSpec::DataBar { kind: 1, module_width: 2, separator_height: 1, height: 25, segments_per_row: 22 }, "0101234567890"),
            (SymbolSpec::DataBar { kind: 9, module_width: 2, separator_height: 1, height: 25, segments_per_row: 22 }, "012345678901|(10)A1"),
            (SymbolSpec::MicroPdf417 { module_width: 2, row_height: 6, mode: 0 }, "1234"),
            (SymbolSpec::Tlc39 { code39_module_width: 2, code39_ratio: 2.0, code39_height: 40, micro_module_width: 2, micro_row_height: 4 }, "123456,ABC123"),
            (SymbolSpec::Codablock { module_width: 2, row_height: 16, security: true, columns: 6, rows: 3, mode: 'F' }, "Codablock data"),
            (SymbolSpec::Code49 { module_width: 2, row_height: 20, interpretation: Code49Interpretation::Below, start_mode: None }, "CODE49"),
            (SymbolSpec::Postal { kind: PostalKind::Postnet, module_width: 2, height: 30, print_interpretation_line: true, print_above: false }, "12345"),
        ];
        for (spec, data) in specs {
            let c = content(spec.clone(), data);
            assert!(c.contains(" re") || c.contains(" c\n") || c.contains(" l\n"), "{spec:?} drew nothing");
        }
    }

    #[test]
    fn databar_rows_use_symbology_heights() {
        // Omnidirectional: one 33-module row at magnification 2.
        let c = content(SymbolSpec::DataBar { kind: 1, module_width: 2, separator_height: 1, height: 25, segments_per_row: 22 }, "0101234567890");
        assert!(c.lines().any(|l| l.ends_with(" 66 re")));
    }

    #[test]
    fn bad_data_draws_nothing() {
        let c = content(SymbolSpec::Postal { kind: PostalKind::Postnet, module_width: 2, height: 30, print_interpretation_line: false, print_above: false }, "12");
        assert!(c.is_empty());
    }
}
