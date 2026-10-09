use crate::{Ctx, FontId};
use pdf_writer::Str;
use ttf_parser::GlyphId;
use zpl_parser::command::{Justification, Orientation};
use zpl_state_manager::DrawInstruction;

/// Zebra places the top of capital letters at the field origin; like the PNG renderer the
/// cap height is taken as 0.75 em.
const CAP_HEIGHT_EM: f32 = 0.75;

/// Extra width the PNG renderer adds to its text boxes (`estimate_text_width`); kept so
/// justified/rotated fields land on the same dots.
const BOX_PAD: f32 = 10.0;

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    let DrawInstruction::DrawText { x, y, text, height, width, orientation, justification, .. } = instruction else {
        return;
    };
    if text.is_empty() {
        return;
    }

    let font = FontId::Triumvirate;
    let (x, y) = (*x as f32, *y as f32);
    // ZPL font sizes are em sizes; width 0 means "same as height".
    let size_y = *height as f32;
    let size_x = if *width == 0 { size_y } else { *width as f32 };

    let advance: f32 = text.chars().map(|c| char_advance(font, c)).sum::<f32>() * size_x;
    // Box width as the PNG renderer computes it.
    let text_width = advance.ceil() + BOX_PAD;
    // Baseline inside an unrotated box whose top is the field origin.
    let baseline = CAP_HEIGHT_EM * size_y;

    let shift = |w: f32| match justification {
        Justification::Left => 0.0,
        Justification::Right => w,
        Justification::Auto => w / 2.0,
    };

    match orientation {
        Orientation::Normal => {
            let ax = x - shift(text_width);
            show_text(ctx, font, text, ax, y + baseline, size_x, size_y);
        }
        Orientation::Rotated90 => {
            // Text box is `text_width` x `2*height`, rotated 90° clockwise: reads top to
            // bottom with glyph tops facing right; rotated box width is 2*height.
            let box_h = 2.0 * size_y;
            let ax = x - shift(box_h);
            with_transform(ctx, [0.0, 1.0, -1.0, 0.0, ax + box_h, y], |ctx| {
                show_text(ctx, font, text, 0.0, baseline, size_x, size_y);
            });
        }
        Orientation::Rotated180 => {
            let box_h = 2.0 * size_y;
            let ax = x - shift(text_width);
            with_transform(ctx, [-1.0, 0.0, 0.0, -1.0, ax + text_width, y + box_h], |ctx| {
                show_text(ctx, font, text, 0.0, baseline, size_x, size_y);
            });
        }
        Orientation::Rotated270 => {
            // Text box is `text_width` x `height + pad`, rotated 90° counter-clockwise:
            // reads bottom to top ending above the origin; rotated box sits left of x
            // for left justification (PNG renderer semantics).
            let box_h = size_y + BOX_PAD;
            let ax = match justification {
                Justification::Left => x - box_h,
                Justification::Right => x,
                Justification::Auto => x - box_h / 2.0,
            };
            with_transform(ctx, [0.0, -1.0, 1.0, 0.0, ax, y], |ctx| {
                show_text(ctx, font, text, 0.0, baseline, size_x, size_y);
            });
        }
    }
}

fn with_transform(ctx: &mut Ctx, matrix: [f32; 6], f: impl FnOnce(&mut Ctx)) {
    ctx.content.save_state();
    ctx.content.transform(matrix);
    f(ctx);
    ctx.content.restore_state();
}

/// Shared text primitive (used by barcodes for interpretation lines).
///
/// Draws `text` in `font` as a real PDF text object. Coordinates are in dots in the
/// current (y-down) user space: `x` is the left edge of the first glyph and `baseline_y`
/// the baseline. `size_x`/`size_y` are the em size in dots horizontally/vertically.
/// Callers wanting rotation wrap the call in `save_state` / `transform` / `restore_state`.
pub(crate) fn show_text(ctx: &mut Ctx, font: FontId, text: &str, x: f32, baseline_y: f32, size_x: f32, size_y: f32) {
    if text.is_empty() {
        return;
    }
    let (name, gids) = ctx.fonts.use_glyphs(font, text);
    let bytes: Vec<u8> = gids.iter().flat_map(|g| g.to_be_bytes()).collect();
    ctx.content
        .begin_text()
        .set_font(name, 1.0)
        // Negative d flips glyphs upright in the y-down user space.
        .set_text_matrix([size_x, 0.0, 0.0, -size_y, x, baseline_y])
        .show(Str(&bytes))
        .end_text();
}

/// Font metrics in em units (1.0 = one em), for layout by callers.
#[allow(dead_code)]
pub(crate) struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
}

/// `ascent` is positive (above the baseline); `descent` is negative (below), as in the font.
pub(crate) fn metrics(font: FontId) -> FontMetrics {
    let face = font.face();
    let upem = face.units_per_em() as f32;
    FontMetrics { ascent: face.ascender() as f32 / upem, descent: face.descender() as f32 / upem }
}

/// Horizontal advance of `ch` in em units.
pub(crate) fn char_advance(font: FontId, ch: char) -> f32 {
    let face = font.face();
    let adv = face.glyph_hor_advance(GlyphId(font.glyph(ch))).unwrap_or(0);
    adv as f32 / face.units_per_em() as f32
}

/// Height of the outline of `ch` in em units (e.g. digit height for '0').
pub(crate) fn glyph_height(font: FontId, ch: char) -> f32 {
    let face = font.face();
    let upem = face.units_per_em() as f32;
    match face.glyph_bounding_box(GlyphId(font.glyph(ch))) {
        Some(b) => (b.y_max as f32 - b.y_min as f32).abs() / upem,
        None => face.ascender() as f32 / upem,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PdfRenderer;
    use zpl_parser::command::FontName;

    fn draw(text: &str, orientation: Orientation, justification: Justification) -> DrawInstruction {
        DrawInstruction::DrawText {
            x: 50,
            y: 60,
            text: text.into(),
            font: FontName::Font0,
            height: 40,
            width: 0,
            orientation,
            justification,
        }
    }

    fn contains(hay: &[u8], needle: &[u8]) -> bool {
        hay.windows(needle.len()).any(|w| w == needle)
    }

    #[test]
    fn draw_text_embeds_type0_font_with_tounicode() {
        let pdf = PdfRenderer::new(400, 300).render(&[draw("Hello 123", Orientation::Normal, Justification::Left)]);
        assert!(pdf.starts_with(b"%PDF"));
        assert!(contains(&pdf, b"/Type0"));
        assert!(contains(&pdf, b"/ToUnicode"));
        assert!(contains(&pdf, b"/Identity-H"));
        assert!(contains(&pdf, b"/F0"));
    }

    #[test]
    fn truetype_font_uses_fontfile2_and_identity_gid_map() {
        let mut registry = crate::fonts::FontRegistry::default();
        let (name, gids) = registry.use_glyphs(FontId::DejaVuMono, "0123");
        assert_eq!(name.0, b"F1");
        assert!(gids.iter().all(|&g| g != 0));
        let mut pdf = pdf_writer::Pdf::new();
        let mut alloc = pdf_writer::Ref::new(1);
        let (programs, ()) = registry.build_programs(true, || ());
        let refs = registry.write(&mut pdf, &mut alloc, programs);
        assert_eq!(refs.len(), 1);
        let bytes = pdf.finish();
        assert!(contains(&bytes, b"/CIDFontType2"));
        assert!(contains(&bytes, b"/FontFile2"));
        assert!(contains(&bytes, b"/CIDToGIDMap /Identity"));
    }

    #[test]
    fn all_orientations_render() {
        for o in [Orientation::Normal, Orientation::Rotated90, Orientation::Rotated180, Orientation::Rotated270] {
            for j in [Justification::Left, Justification::Right, Justification::Auto] {
                let pdf = PdfRenderer::new(400, 300).render(&[draw("AB", o, j)]);
                assert!(contains(&pdf, b"/Type0"));
            }
        }
    }

    #[test]
    fn shared_metrics_are_sane() {
        for f in [FontId::Triumvirate, FontId::DejaVuMono] {
            let m = metrics(f);
            assert!(m.ascent > 0.5 && m.descent < 0.0);
            assert!(char_advance(f, '0') > 0.2);
            assert!(glyph_height(f, '0') > 0.4);
        }
    }

    #[test]
    fn writes_sample_pdf_when_requested() {
        if let Ok(path) = std::env::var("PDF_TEXT_SAMPLE") {
            let ins: Vec<_> = [
                (Orientation::Normal, 20, 20),
                (Orientation::Rotated90, 300, 20),
                (Orientation::Rotated180, 20, 200),
                (Orientation::Rotated270, 300, 380),
            ]
            .into_iter()
            .map(|(o, x, y)| DrawInstruction::DrawText {
                x,
                y,
                text: "Hello World".into(),
                font: FontName::Font0,
                height: 40,
                width: 0,
                orientation: o,
                justification: Justification::Left,
            })
            .collect();
            std::fs::write(&path, PdfRenderer::new(406, 406).render(&ins)).unwrap();
        }
    }
}
