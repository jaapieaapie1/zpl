//! ^GB boxes, ^GC circles and ^GD diagonal lines as filled vector paths.
//!
//! All geometry is filled (never stroked) so the painted area matches ZPL dot semantics
//! exactly: borders grow inward from the outer bounds.

use crate::Ctx;
use pdf_writer::{Content, Name};
use zpl_parser::command::{DiagonalDirection, LineColor};
use zpl_state_manager::DrawInstruction;

/// Bezier control-point factor for approximating a quarter circle.
const KAPPA: f32 = 0.552_284_8;

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    match *instruction {
        DrawInstruction::DrawBox { x, y, width, height, thickness, color, rounding, reverse } => {
            draw_box(ctx, x, y, width, height, thickness, color, rounding, reverse)
        }
        DrawInstruction::DrawCircle { x, y, diameter, thickness, color } => {
            draw_circle(ctx, x, y, diameter, thickness, color)
        }
        DrawInstruction::DrawDiagonalLine { x, y, width, height, thickness, color, direction } => {
            draw_diagonal(ctx, x, y, width, height, thickness, color, direction)
        }
        _ => {}
    }
}

fn gray(color: LineColor) -> f32 {
    match color {
        LineColor::Black => 0.0,
        LineColor::White => 1.0,
    }
}

/// Begin a painted element: save state and pick the fill. Reverse (^FR) paints white
/// through the Difference blend mode, which inverts whatever is underneath.
fn begin(ctx: &mut Ctx, color: LineColor, reverse: bool) {
    ctx.content.save_state();
    if reverse {
        ctx.uses_difference = true;
        ctx.content.set_parameters(Name(b"GsDiff")).set_fill_gray(1.0);
    } else {
        ctx.content.set_fill_gray(gray(color));
    }
}

/// Append a (possibly rounded) rectangle subpath.
fn rounded_rect(c: &mut Content, x: f32, y: f32, w: f32, h: f32, r: f32) {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    if r <= 0.0 {
        c.rect(x, y, w, h);
        return;
    }
    let k = r * KAPPA;
    let (x1, y1) = (x + w, y + h);
    c.move_to(x + r, y);
    c.line_to(x1 - r, y);
    c.cubic_to(x1 - r + k, y, x1, y + r - k, x1, y + r);
    c.line_to(x1, y1 - r);
    c.cubic_to(x1, y1 - r + k, x1 - r + k, y1, x1 - r, y1);
    c.line_to(x + r, y1);
    c.cubic_to(x + r - k, y1, x, y1 - r + k, x, y1 - r);
    c.line_to(x, y + r);
    c.cubic_to(x, y + r - k, x + r - k, y, x + r, y);
    c.close_path();
}

/// Append a full-circle subpath (four Bezier arcs).
fn ellipse(c: &mut Content, cx: f32, cy: f32, r: f32) {
    let k = r * KAPPA;
    c.move_to(cx + r, cy);
    c.cubic_to(cx + r, cy + k, cx + k, cy + r, cx, cy + r);
    c.cubic_to(cx - k, cy + r, cx - r, cy + k, cx - r, cy);
    c.cubic_to(cx - r, cy - k, cx - k, cy - r, cx, cy - r);
    c.cubic_to(cx + k, cy - r, cx + r, cy - k, cx + r, cy);
    c.close_path();
}

#[allow(clippy::too_many_arguments)]
fn draw_box(
    ctx: &mut Ctx,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    thickness: u32,
    color: LineColor,
    rounding: u32,
    reverse: bool,
) {
    if thickness == 0 {
        return;
    }
    // ZPL: width and height are never smaller than the border thickness.
    let w = width.max(thickness) as f32;
    let h = height.max(thickness) as f32;
    let t = thickness as f32;
    let (x, y) = (x as f32, y as f32);
    let short = w.min(h);
    // ^GB rounding 0-8: radius = (rounding / 8) * (shorter side / 2).
    let radius = rounding.min(8) as f32 / 8.0 * short / 2.0;

    begin(ctx, color, reverse);
    rounded_rect(&mut ctx.content, x, y, w, h, radius);
    if 2.0 * t < short {
        let inner_r = (radius - t).max(0.0);
        rounded_rect(&mut ctx.content, x + t, y + t, w - 2.0 * t, h - 2.0 * t, inner_r);
    }
    ctx.content.fill_even_odd();
    ctx.content.restore_state();
}

fn draw_circle(ctx: &mut Ctx, x: u32, y: u32, diameter: u32, thickness: u32, color: LineColor) {
    if diameter == 0 || thickness == 0 {
        return;
    }
    let d = diameter as f32;
    let r = d / 2.0;
    let (cx, cy) = (x as f32 + r, y as f32 + r);
    let t = thickness as f32;

    begin(ctx, color, false);
    ellipse(&mut ctx.content, cx, cy, r);
    if t < r {
        ellipse(&mut ctx.content, cx, cy, r - t);
    }
    ctx.content.fill_even_odd();
    ctx.content.restore_state();
}

/// Corner points of a ^GD parallelogram: the line spans `width` x `height` dots and
/// has a horizontal thickness of `thickness` dots.
pub(crate) fn diagonal_points(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    thickness: u32,
    direction: DiagonalDirection,
) -> [(f32, f32); 4] {
    let t = thickness as f32;
    let w = (width.max(thickness)) as f32;
    let h = height.max(1) as f32;
    let (x0, y0) = (x as f32, y as f32);
    let (x1, y1) = (x0 + w, y0 + h);
    match direction {
        // '\' : top-left to bottom-right
        DiagonalDirection::LeftToRight => [(x0, y0), (x0 + t, y0), (x1, y1), (x1 - t, y1)],
        // '/' : top-right to bottom-left
        DiagonalDirection::RightToLeft => [(x1 - t, y0), (x1, y0), (x0 + t, y1), (x0, y1)],
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_diagonal(
    ctx: &mut Ctx,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    thickness: u32,
    color: LineColor,
    direction: DiagonalDirection,
) {
    if thickness == 0 {
        return;
    }
    let pts = diagonal_points(x, y, width, height, thickness, direction);
    begin(ctx, color, false);
    let c = &mut ctx.content;
    c.move_to(pts[0].0, pts[0].1);
    for &(px, py) in &pts[1..] {
        c.line_to(px, py);
    }
    c.close_path();
    c.fill_nonzero();
    ctx.content.restore_state();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::FontRegistry;

    fn ctx() -> Ctx {
        Ctx {
            content: Content::new(),
            fonts: FontRegistry::default(),
            images: Vec::new(),
            uses_difference: false,
        }
    }

    fn ops(instruction: DrawInstruction) -> (String, bool) {
        let mut c = ctx();
        render(&mut c, &instruction);
        let used = c.uses_difference;
        (String::from_utf8(c.content.finish().to_vec()).unwrap(), used)
    }

    fn gb(w: u32, h: u32, t: u32, rounding: u32, reverse: bool, color: LineColor) -> DrawInstruction {
        DrawInstruction::DrawBox { x: 10, y: 20, width: w, height: h, thickness: t, color, rounding, reverse }
    }

    #[test]
    fn box_border_is_outer_minus_inner() {
        let (s, diff) = ops(gb(100, 50, 5, 0, false, LineColor::Black));
        assert!(s.contains("10 20 100 50 re"));
        assert!(s.contains("15 25 90 40 re"));
        assert!(s.contains("f*"));
        assert!(s.contains("0 g"));
        assert!(!diff);
    }

    #[test]
    fn thick_box_is_solid() {
        let (s, _) = ops(gb(100, 50, 50, 0, false, LineColor::White));
        assert_eq!(s.matches(" re").count(), 1);
        assert!(s.contains("1 g"));
    }

    #[test]
    fn box_grows_to_thickness() {
        // ^GB0,0,3 draws a 3x3 dot.
        let (s, _) = ops(gb(0, 0, 3, 0, false, LineColor::Black));
        assert!(s.contains("10 20 3 3 re"));
    }

    #[test]
    fn rounded_box_uses_curves() {
        let (s, _) = ops(gb(100, 50, 5, 8, false, LineColor::Black));
        assert!(s.contains(" c\n") || s.contains(" c"));
        assert!(!s.contains(" re"));
    }

    #[test]
    fn reverse_box_uses_difference() {
        let (s, diff) = ops(gb(100, 50, 50, 0, true, LineColor::Black));
        assert!(diff);
        assert!(s.contains("/GsDiff gs"));
        assert!(s.contains("1 g"));
        assert!(s.starts_with("q"));
        assert!(s.trim_end().ends_with("Q"));
    }

    #[test]
    fn circle_ring_has_two_subpaths() {
        let (s, _) = ops(DrawInstruction::DrawCircle { x: 0, y: 0, diameter: 100, thickness: 10, color: LineColor::Black });
        assert_eq!(s.matches(" m").count(), 2);
        assert!(s.contains("f*"));
        let (s, _) = ops(DrawInstruction::DrawCircle { x: 0, y: 0, diameter: 100, thickness: 50, color: LineColor::Black });
        assert_eq!(s.matches(" m").count(), 1);
    }

    #[test]
    fn diagonal_geometry() {
        let p = diagonal_points(0, 0, 100, 50, 4, DiagonalDirection::LeftToRight);
        assert_eq!(p, [(0.0, 0.0), (4.0, 0.0), (100.0, 50.0), (96.0, 50.0)]);
        let p = diagonal_points(0, 0, 100, 50, 4, DiagonalDirection::RightToLeft);
        assert_eq!(p, [(96.0, 0.0), (100.0, 0.0), (4.0, 50.0), (0.0, 50.0)]);
    }
}
