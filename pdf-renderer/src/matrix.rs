//! 2D symbols (DataMatrix, PDF417, Aztec) as runs of filled module rectangles.

use crate::barcodes::{Origin, bars_placement, fill_rects, runs};
use crate::Ctx;
use zpl_parser::command::Orientation;
use zpl_state_manager::DrawInstruction;
use zpl_symbols::matrix::{self, Matrix};

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    match instruction {
        DrawInstruction::DrawDataMatrix {
            x, y, data, orientation, module_height, symbol_height, columns, rows, escape, rectangular, bottom,
            right,
        } => {
            let Ok(m) = matrix::datamatrix(&png_renderer::symbol_bytes(data), *rows, *columns, *rectangular, *escape) else { return };
            // Without an explicit module size the ^BY height is the approximate symbol height.
            let module = module_height.unwrap_or(symbol_height / m.height.max(1)).max(1);
            draw(ctx, &m, (*x, *y, *bottom, *right), *orientation, module, module);
        }
        DrawInstruction::DrawPdf417 {
            x, y, data, orientation, row_height, module_width, symbol_height, security, columns, rows, truncated, bottom,
            right,
        } => {
            let Ok(m) = matrix::pdf417(&png_renderer::symbol_bytes(data), *security, *columns, *rows, *truncated) else { return };
            let module = (*module_width).max(1);
            // Row height in dots, or the ^BY height shared by all rows.
            let row = row_height.unwrap_or(symbol_height / m.height.max(1)).max(1);
            draw(ctx, &m, (*x, *y, *bottom, *right), *orientation, module, row);
        }
        DrawInstruction::DrawAztec { x, y, data, orientation, magnification, ecc, bottom, right } => {
            let Ok(m) = matrix::aztec(&png_renderer::symbol_bytes(data), *ecc) else { return };
            let module = (*magnification).max(1);
            draw(ctx, &m, (*x, *y, *bottom, *right), *orientation, module, module);
        }
        _ => {}
    }
}

/// Draws `m` with `module_w` x `module_h` dot modules, placed like the bars of a 1D symbol:
/// with `bottom` (^FT) the origin is the unrotated symbol's bottom-left corner.
fn draw(ctx: &mut Ctx, m: &Matrix, at: Origin, orientation: Orientation, module_w: u32, module_h: u32) {
    let (w, h) = (m.width * module_w, m.height * module_h);
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
    fill_rects(ctx, bars_placement(orientation, at, [0.0, 0.0, w as f32, h as f32]), &rects);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn content_of(instruction: DrawInstruction) -> String {
        let mut ctx = Ctx::new();
        render(&mut ctx, &instruction);
        String::from_utf8(ctx.content.finish().to_vec()).unwrap()
    }

    #[test]
    fn rotated_symbols_are_placed_like_bars() {
        // ^FO: the rotated symbol's top-left is the origin (84 x 84 dots here).
        let dm = DrawInstruction::DrawDataMatrix {
            x: 650, y: 50, data: "ROTATE DM".into(), orientation: Orientation::Rotated270, module_height: Some(6),
            symbol_height: 10, columns: 0, rows: 0, escape: None, rectangular: false, bottom: false, right: false,
        };
        assert!(content_of(dm).contains("0 -1 1 0 650 134 cm"));
        // ^FT: the unrotated bottom-left corner is the origin; a 206 x 24 dot PDF417 read
        // bottom up ends on it (Labelary: 376,694 to 399,899).
        let pdf = DrawInstruction::DrawPdf417 {
            x: 400, y: 900, data: "PDF FT".into(), orientation: Orientation::Rotated270, row_height: Some(4),
            module_width: 2, symbol_height: 10, security: Some(0), columns: 2, rows: 6, truncated: false, bottom: true,
            right: false,
        };
        assert!(content_of(pdf).contains("0 -1 1 0 376 900 cm"));
    }
}
