//! 2D symbols (DataMatrix, PDF417, Aztec) as runs of filled module rectangles.

use crate::barcodes::{fill_rects, placement, runs};
use crate::Ctx;
use zpl_parser::command::Orientation;
use zpl_state_manager::DrawInstruction;
use zpl_symbols::matrix::{self, Matrix};

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    match instruction {
        DrawInstruction::DrawDataMatrix {
            x, y, data, orientation, module_height, symbol_height, columns, rows, escape, rectangular, bottom,
        } => {
            let Ok(m) = matrix::datamatrix(data.as_bytes(), *rows, *columns, *rectangular, *escape) else { return };
            // Without an explicit module size the ^BY height is the approximate symbol height.
            let module = module_height.unwrap_or(symbol_height / m.height.max(1)).max(1);
            draw(ctx, &m, *x, *y, *orientation, module, module, *bottom);
        }
        DrawInstruction::DrawPdf417 {
            x, y, data, orientation, row_height, module_width, symbol_height, security, columns, rows, truncated, bottom,
        } => {
            let Ok(m) = matrix::pdf417(data.as_bytes(), *security, *columns, *rows, *truncated) else { return };
            let module = (*module_width).max(1);
            // Row height is a multiple of the module width, or the ^BY height shared by all rows.
            let row = row_height.map(|h| h * module).unwrap_or(symbol_height / m.height.max(1)).max(1);
            draw(ctx, &m, *x, *y, *orientation, module, row, *bottom);
        }
        DrawInstruction::DrawAztec { x, y, data, orientation, magnification, ecc, bottom } => {
            let Ok(m) = matrix::aztec(data.as_bytes(), *ecc) else { return };
            let module = (*magnification).max(1);
            draw(ctx, &m, *x, *y, *orientation, module, module, *bottom);
        }
        _ => {}
    }
}

/// Draws `m` with `module_w` x `module_h` dot modules; with `bottom` (^FT) the origin is the
/// symbol's bottom-left corner.
#[allow(clippy::too_many_arguments)]
fn draw(ctx: &mut Ctx, m: &Matrix, x: u32, y: u32, orientation: Orientation, module_w: u32, module_h: u32, bottom: bool) {
    let (w, h) = (m.width * module_w, m.height * module_h);
    let y = if bottom { y.saturating_sub(h) } else { y };
    let (mw, mh) = (module_w as f32, module_h as f32);

    let mut rects = Vec::new();
    let mut row_bits = Vec::with_capacity(m.width as usize);
    for row in 0..m.height {
        row_bits.clear();
        row_bits.extend((0..m.width).map(|col| m.get(col, row) as u8));
        for (start, len) in runs(&row_bits) {
            rects.push((start as f32 * mw, row as f32 * mh, len as f32 * mw, mh));
        }
    }
    fill_rects(ctx, placement(orientation, x, y, w, h, 0), &rects);
}
