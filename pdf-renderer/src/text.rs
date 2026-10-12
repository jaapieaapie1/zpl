use std::borrow::Cow;
use std::ops::Range;
use std::sync::OnceLock;

use crate::fonts::{FontKey, FontRegistry, ResName};
use crate::{Ctx, FontId};
use pdf_writer::types::{LineJoinStyle, TextRenderingMode};
use pdf_writer::Str;
use ttf_parser::{Face, GlyphId};
use zpl_parser::command::{BlockJustification, FieldDirection, FontName, Justification, Orientation, TextLayout};

mod shape;
use shape::{Mode, ShapedLine};
use zpl_state_manager::{DrawInstruction, TextAnchor, TextBlock};

/// Zebra places the top of capital letters at the field origin; like the PNG renderer the
/// cap height is taken as 0.75 em.
const CAP_HEIGHT_EM: f32 = 0.75;

/// Extra width the PNG renderer adds to its text boxes (`estimate_text_width`); kept so
/// justified/rotated fields land on the same dots.
const BOX_PAD: f32 = 10.0;

/// Horizontal squeeze for DejaVu Sans Bold fallback glyphs next to the condensed Triumvirate.
const FALLBACK_CONDENSE: f32 = 0.8;

/// Monospaced outline font that stands in for the bitmap fonts A–H. The regular face is
/// the one barcode interpretation lines embed anyway (a third embedded font costs ~60 µs a
/// label); it is emboldened by stroking the outlines, see [`BITMAP_STROKE_EM`].
const BITMAP_FONT: FontId = FontId::DejaVuMono;

/// Outline stroke width for bitmap-font glyphs, in em of the cap height: brings DejaVu Sans
/// Mono's stems close to the weight of Zebra's bitmap fonts.
const BITMAP_STROKE_EM: f32 = 0.055;

/// One of Zebra's bitmapped fonts A–H at magnification 1, in dots ("Intercharacter Gap and
/// Baseline Parameters" / "Font Matrices" in docs/commands/other/FONTS.md).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BitmapCell {
    pub height: u32,
    pub width: u32,
    /// Intercharacter gap after every character.
    pub gap: u32,
    /// Baseline, measured down from the top of the cell.
    pub baseline: u32,
    /// Uppercase-only character set (fonts B and H have no lowercase or descenders).
    pub uppercase: bool,
}

/// Matrix of bitmap font `font` on a `dpmm` dots/mm printhead; `None` for scalable fonts.
pub(crate) fn bitmap_cell(font: FontName, dpmm: u32) -> Option<BitmapCell> {
    let cell = |height, width, gap, baseline, uppercase| BitmapCell { height, width, gap, baseline, uppercase };
    // Only E (OCR-B) and H (OCR-A) differ between resolutions; gap and baseline are given
    // for 8 dots/mm and scaled with the matrix elsewhere.
    let scaled = |h: u32, w: u32, (h8, w8, gap8, base8): (u32, u32, u32, u32), uppercase| {
        let gap = ((gap8 * w) as f32 / w8 as f32).round() as u32;
        let base = ((base8 * h) as f32 / h8 as f32).round() as u32;
        cell(h, w, gap, base, uppercase)
    };
    Some(match font {
        FontName::FontA => cell(9, 5, 1, 7, false),
        FontName::FontB => cell(11, 7, 2, 11, true),
        FontName::FontC | FontName::FontD => cell(18, 10, 2, 14, false),
        FontName::FontE => {
            let (h, w) = match dpmm {
                0..=6 => (21, 10),
                7..=8 => (28, 15),
                _ => (42, 20),
            };
            scaled(h, w, (28, 15, 5, 23), false)
        }
        FontName::FontF => cell(26, 13, 3, 21, false),
        FontName::FontG => cell(60, 40, 8, 48, false),
        FontName::FontH => {
            let (h, w) = match dpmm {
                0..=6 => (17, 11),
                7..=8 => (21, 13),
                _ => (34, 22),
            };
            scaled(h, w, (21, 13, 6, 21), true)
        }
        _ => return None,
    })
}

/// Fonts P–V: fixed-size renditions of the scalable font 0 (H x W in dots at magnification 1).
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

/// Integer magnifications (vertical, horizontal) of a `base_h` x `base_w` matrix for a
/// requested `height` x `width`: each is the nearest multiple of the base, 1 to 10 (^A).
/// A dimension given alone sets both ("the standard matrix for that font automatically
/// determines the other value"). The state manager reports an omitted width as equal to
/// the height, so `width == height` on a non-square matrix counts as "width not given".
pub(crate) fn magnification(height: u32, width: u32, base_h: u32, base_w: u32) -> (u32, u32) {
    let mag = |v: u32, base: u32| (v > 0).then(|| ((v as f32 / base as f32).round() as u32).clamp(1, 10));
    let width = if width == height && base_h != base_w { 0 } else { width };
    match (mag(height, base_h), mag(width, base_w)) {
        (Some(h), Some(w)) => (h, w),
        (Some(m), None) | (None, Some(m)) => (m, m),
        (None, None) => (1, 1),
    }
}

/// Bitmap-font layout: every character occupies a fixed cell.
#[derive(Clone, Copy)]
struct Bitmap {
    /// Cell width plus intercharacter gap, in dots.
    pitch: f32,
    /// Offset from the cell's left edge to the outline glyph's pen position.
    offset: f32,
    /// Cell width without the intercharacter gap.
    cell_w: f32,
}

/// Layout parameters shared by the single-line, block and vertical paths.
struct Field {
    font: FontKey,
    x: f32,
    y: f32,
    /// Em size of `font` in dots, horizontally and vertically.
    size_x: f32,
    size_y: f32,
    /// ^FP inter-character gap in dots.
    gap: f32,
    /// Line advance in dots (em size, or the bitmap cell height).
    line_h: f32,
    /// Distance from the top of a line to its baseline.
    ascent: f32,
    bitmap: Option<Bitmap>,
    /// Line width (dots) for fill+stroke rendering that emboldens the glyphs; 0 = fill only.
    stroke: f32,
    orientation: Orientation,
    anchor: TextAnchor,
    /// ^PA default glyph: characters no font has print as the field font's .notdef glyph
    /// (usually a box) instead of a space.
    notdef: bool,
}

/// Cached per-face metrics of a built-in font, in em units.
struct BuiltinMetrics {
    cap: f32,
    advance_m: f32,
    ink_m: (f32, f32),
}

fn builtin_metrics(id: FontId) -> &'static BuiltinMetrics {
    static CACHE: [OnceLock<BuiltinMetrics>; 3] = [OnceLock::new(), OnceLock::new(), OnceLock::new()];
    CACHE[id as usize].get_or_init(|| {
        let face = id.face();
        let upem = face.units_per_em() as f32;
        let m = face.glyph_index('M').unwrap_or(GlyphId(0));
        let bbox = face.glyph_bounding_box(m);
        BuiltinMetrics {
            cap: cap_height(face),
            advance_m: face.glyph_hor_advance(m).unwrap_or(0) as f32 / upem,
            ink_m: bbox.map_or((0.0, 0.6), |b| (b.x_min as f32 / upem, (b.x_max - b.x_min) as f32 / upem)),
        }
    })
}

/// Cap height in em (OS/2, else the outline of 'H', else 0.7).
fn cap_height(face: &Face) -> f32 {
    let upem = face.units_per_em() as f32;
    face.capital_height()
        .filter(|&h| h > 0)
        .map(|h| h as f32)
        .or_else(|| face.glyph_index('H').and_then(|g| face.glyph_bounding_box(g)).map(|b| b.y_max as f32))
        .map_or(0.7, |h| h / upem)
}

fn font_cap(fonts: &FontRegistry, key: FontKey) -> f32 {
    match key {
        FontKey::Builtin(id) => builtin_metrics(id).cap,
        FontKey::Custom(_) => cap_height(fonts.face(key)),
    }
}

/// Fonts tried, in order, for a character the field's font lacks.
const FALLBACKS: [FontId; 2] = [FontId::Triumvirate, FontId::DejaVuSansBold];

/// Font and glyph that show `ch` in a field set in `primary`, and the character the glyph
/// stands for: the primary font when it has the glyph, else the first fallback that does.
/// A character no font has prints as the primary's space (^PA default glyph off, the
/// default) or its .notdef glyph (`notdef`).
#[inline]
fn resolve(fonts: &FontRegistry, primary: FontKey, ch: char, notdef: bool) -> (FontKey, u16, char) {
    if let Some(g) = fonts.face(primary).glyph_index(ch) {
        return (primary, g.0, ch);
    }
    resolve_fallback(fonts, primary, ch, notdef)
}

#[cold]
fn resolve_fallback(fonts: &FontRegistry, primary: FontKey, ch: char, notdef: bool) -> (FontKey, u16, char) {
    for id in FALLBACKS {
        let key = FontKey::Builtin(id);
        if key == primary {
            continue;
        }
        if let Some(g) = id.face().glyph_index(ch) {
            return (key, g.0, ch);
        }
    }
    match fonts.face(primary).glyph_index(' ') {
        Some(space) if !notdef => (primary, space.0, ' '),
        _ => (primary, 0, ch),
    }
}

fn advance_em(fonts: &FontRegistry, key: FontKey, gid: u16) -> f32 {
    let face = fonts.face(key);
    face.glyph_hor_advance(GlyphId(gid)).unwrap_or(0) as f32 / face.units_per_em() as f32
}

impl Field {
    /// Em size (x, y) of fallback font `key` in this field: cap heights matched to the
    /// field's font, condensed next to Triumvirate's narrow glyphs.
    fn fallback_size(&self, fonts: &FontRegistry, key: FontKey) -> (f32, f32) {
        let k = font_cap(fonts, self.font) / font_cap(fonts, key);
        let condense = if key == FontKey::Builtin(FontId::DejaVuSansBold) { FALLBACK_CONDENSE } else { 1.0 };
        (self.size_x * k * condense, self.size_y * k)
    }

    /// Advance of one character in dots, without the ^FP gap.
    fn char_width(&self, fonts: &FontRegistry, ch: char) -> f32 {
        if let Some(b) = self.bitmap {
            return b.pitch;
        }
        let (key, gid, _) = resolve(fonts, self.font, ch, self.notdef);
        let size_x = if key == self.font { self.size_x } else { self.fallback_size(fonts, key).0 };
        advance_em(fonts, key, gid) * size_x
    }

    /// Width in dots of `text` laid out on one line, including the ^FP gaps between characters.
    fn measure(&self, fonts: &FontRegistry, text: &str) -> f32 {
        let mut n = 0usize;
        let adv: f32 = text
            .chars()
            .map(|c| {
                n += 1;
                self.char_width(fonts, c)
            })
            .sum();
        adv + self.gap * n.saturating_sub(1) as f32
    }

    /// Maps a local frame (u along the text, v down across lines, both in dots) into page
    /// space. For ^FO (top anchor) the local box `w` x `h` with its top-left at (0, 0) lands
    /// with its rotated bounding box's top-left on the field origin; for ^FT (baseline anchor)
    /// local (0, 0) is the origin itself and the frame simply rotates around it.
    fn frame(&self, w: f32, h: f32) -> [f32; 6] {
        let (x, y) = (self.x, self.y);
        match (self.anchor, self.orientation) {
            (_, Orientation::Normal) => [1.0, 0.0, 0.0, 1.0, x, y],
            (TextAnchor::Baseline, Orientation::Rotated90) => [0.0, 1.0, -1.0, 0.0, x, y],
            (TextAnchor::Baseline, Orientation::Rotated180) => [-1.0, 0.0, 0.0, -1.0, x, y],
            (TextAnchor::Baseline, Orientation::Rotated270) => [0.0, -1.0, 1.0, 0.0, x, y],
            (TextAnchor::Top, Orientation::Rotated90) => [0.0, 1.0, -1.0, 0.0, x + h, y],
            (TextAnchor::Top, Orientation::Rotated180) => [-1.0, 0.0, 0.0, -1.0, x + w, y + h],
            (TextAnchor::Top, Orientation::Rotated270) => [0.0, -1.0, 1.0, 0.0, x, y + w],
        }
    }

    /// Runs `f` in the local frame given by `matrix`. Unrotated frames are passed as an
    /// offset instead of a `cm` so plain text needs no extra graphics state.
    fn in_frame(&self, ctx: &mut Ctx, matrix: [f32; 6], f: impl FnOnce(&mut Ctx, f32, f32)) {
        if matches!(self.orientation, Orientation::Normal) {
            f(ctx, matrix[4], matrix[5]);
        } else {
            with_transform(ctx, matrix, |ctx| f(ctx, 0.0, 0.0));
        }
    }

    /// Draws `text` with its first character's cell (or pen position) at `x` and its
    /// baseline at `baseline_y`, switching fonts for characters the field's font lacks.
    fn show(&self, ctx: &mut Ctx, text: &str, x: f32, baseline_y: f32) {
        if text.is_empty() {
            return;
        }
        // Fast path: every glyph in the field's font, one text run.
        let mut bytes = Vec::with_capacity(text.len() * 2);
        if let Some(name) = ctx.fonts.use_text(self.font, text, &mut bytes) {
            let (offset, tc) = self.char_spacing(self.font, self.size_x);
            begin(ctx, self.stroke);
            put_run(ctx, name, &bytes, [self.size_x, self.size_y], tc, x + offset, baseline_y);
            end(ctx, tc != 0.0, self.stroke);
            return;
        }
        let fonts = &ctx.fonts;

        // Mixed fonts: one run per stretch of characters resolved to the same font.
        let resolved: Vec<(char, FontKey, u16)> = text
            .chars()
            .map(|c| {
                let (key, gid, shown) = resolve(fonts, self.font, c, self.notdef);
                (shown, key, gid)
            })
            .collect();
        begin(ctx, self.stroke);
        let mut any_tc = false;
        let mut pen = x;
        let mut i = 0;
        while i < resolved.len() {
            let key = resolved[i].1;
            let mut j = i + 1;
            while j < resolved.len() && resolved[j].1 == key {
                j += 1;
            }
            let run = &resolved[i..j];
            let fonts = &ctx.fonts;
            if let (Some(b), false) = (self.bitmap, key == self.font) {
                // Proportional fallback glyphs inside fixed cells: each one centred in its
                // cell, squeezed to fit it.
                let (_, sy) = self.fallback_size(fonts, key);
                for &(c, _, g) in run {
                    let adv = advance_em(&ctx.fonts, key, g);
                    let sx = if adv > 0.0 { (b.cell_w / adv).min(sy) } else { sy };
                    let left = pen + (b.cell_w - adv * sx) / 2.0;
                    emit_run(ctx, key, std::iter::once((c, g)), [sx, sy], 0.0, left, baseline_y);
                    pen += b.pitch + self.gap;
                }
                i = j;
                continue;
            }
            let size = if key == self.font {
                [self.size_x, self.size_y]
            } else {
                let (sx, sy) = self.fallback_size(fonts, key);
                [sx, sy]
            };
            let width: f32 = match self.bitmap {
                Some(b) => (b.pitch + self.gap) * run.len() as f32,
                None => run.iter().map(|&(_, _, g)| advance_em(fonts, key, g) * size[0] + self.gap).sum(),
            };
            let (offset, tc) = self.char_spacing(key, size[0]);
            any_tc |= tc != 0.0;
            emit_run(ctx, key, run.iter().map(|&(c, _, g)| (c, g)), size, tc, pen + offset, baseline_y);
            pen += width;
            i = j;
        }
        end(ctx, any_tc, self.stroke);
    }

    /// Pen offset from the cell's left edge and `Tc` (unscaled text space) for a run in
    /// `key` (the field's font, or a fallback in a scalable field) at horizontal em size `size_x`.
    fn char_spacing(&self, key: FontKey, size_x: f32) -> (f32, f32) {
        match (self.bitmap, key) {
            (Some(b), FontKey::Builtin(id)) => (b.offset, (b.pitch + self.gap) / size_x - builtin_metrics(id).advance_m),
            _ => (0.0, self.gap / size_x),
        }
    }
}

/// Builds the layout of a DrawText field.
#[allow(clippy::too_many_arguments)]
fn field_for(
    ctx: &mut Ctx,
    font: FontName,
    custom_font: Option<&std::sync::Arc<Vec<u8>>>,
    height: u32,
    width: u32,
    x: f32,
    y: f32,
    gap: f32,
    orientation: Orientation,
    anchor: TextAnchor,
) -> Field {
    let scalable = |font: FontKey, cap: f32, size_y: f32, size_x: f32| Field {
        font,
        x,
        y,
        size_x,
        size_y,
        gap,
        line_h: size_y,
        ascent: cap * size_y,
        bitmap: None,
        stroke: 0.0,
        orientation,
        anchor,
        notdef: false,
    };

    // Downloaded TrueType/OpenType font: scalable like font 0.
    if let Some(key) = custom_font.and_then(|data| ctx.fonts.register_custom(data)) {
        let size_y = height.max(1) as f32;
        let size_x = if width == 0 { size_y } else { width as f32 };
        let cap = cap_height(ctx.fonts.face(key));
        return scalable(key, cap, size_y, size_x);
    }

    if let Some(cell) = bitmap_cell(font, ctx.dpmm) {
        let (mh, mw) = magnification(height, width, cell.height, cell.width);
        let m = builtin_metrics(BITMAP_FONT);
        // Cap height of the stroked outline = character height above the baseline; the
        // widest glyph ('M') fills the cell width; the intercharacter gap follows.
        let cap = (cell.baseline * mh) as f32;
        let stroke = BITMAP_STROKE_EM * cap;
        let size_y = (cap - stroke) / m.cap;
        let ink_w = (cell.width * mw) as f32;
        let size_x = (ink_w - stroke) / m.ink_m.1;
        return Field {
            font: BITMAP_FONT.into(),
            x,
            y,
            size_x,
            size_y,
            gap,
            line_h: (cell.height * mh) as f32,
            ascent: (cell.baseline * mh) as f32,
            bitmap: Some(Bitmap {
                pitch: ((cell.width + cell.gap) * mw) as f32,
                offset: stroke / 2.0 - m.ink_m.0 * size_x,
                cell_w: ink_w,
            }),
            stroke,
            orientation,
            anchor,
            notdef: false,
        };
    }

    let (size_y, size_x) = match fixed_scalable(font) {
        Some((h, w)) => {
            let (mh, mw) = magnification(height, width, h, w);
            ((h * mh) as f32, (w * mw) as f32)
        }
        // ZPL font sizes are em sizes; width 0 means "same as height".
        None => {
            let size_y = height as f32;
            (size_y, if width == 0 { size_y } else { width as f32 })
        }
    };
    scalable(FontId::Triumvirate.into(), CAP_HEIGHT_EM, size_y, size_x)
}

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    let DrawInstruction::DrawText {
        x,
        y,
        text,
        font,
        height,
        width,
        orientation,
        justification,
        anchor,
        block,
        direction,
        char_gap,
        custom_font,
        layout,
    } = instruction
    else {
        return;
    };
    if text.is_empty() {
        return;
    }

    let mut field = field_for(
        ctx,
        *font,
        custom_font.as_ref(),
        *height,
        *width,
        *x as f32,
        *y as f32,
        *char_gap as f32,
        *orientation,
        *anchor,
    );
    // Control characters (from ^FH escapes such as _0D) print nothing.
    let text: Cow<str> = if text.chars().any(|c| c < ' ' || c == '\x7f') {
        Cow::Owned(text.chars().filter(|&c| c >= ' ' && c != '\x7f').collect())
    } else {
        Cow::Borrowed(text)
    };
    // Uppercase-only bitmap fonts print lowercase letters as capitals.
    let text: Cow<str> = match bitmap_cell(*font, ctx.dpmm) {
        Some(cell) if cell.uppercase && custom_font.is_none() && text.chars().any(char::is_lowercase) => {
            Cow::Owned(text.to_uppercase())
        }
        _ => text,
    };
    let text: &str = &text;
    field.notdef = layout.default_glyph;
    let reverse = matches!(direction, FieldDirection::Reverse);

    // ^FPV prints one character per line in logical order: there is nothing to reorder or
    // join, so vertical fields always take the classic path.
    let mode = match direction {
        FieldDirection::Vertical => Mode::Classic,
        _ => shape::mode(layout, field.bitmap.is_some(), text),
    };
    if mode != Mode::Classic {
        match block {
            Some(block) => render_block(ctx, &field, text, block, reverse, mode, layout),
            None => {
                let info = shape::bidi_info(layout, text);
                let line = match mode {
                    Mode::Bidi => {
                        let visual = shape::visual_string(info.as_ref(), text, 0..text.len());
                        Shown::Text(if reverse { visual.chars().rev().collect() } else { visual })
                    }
                    _ => {
                        let features = shape::Features::new(layout);
                        let line = shape::shape(&ctx.fonts, &field, text, 0..text.len(), info.as_ref(), &features);
                        Shown::Shaped(line, reverse)
                    }
                };
                render_line(ctx, &field, &line, *justification);
            }
        }
        return;
    }

    match (direction, block) {
        (FieldDirection::Vertical, _) => render_vertical(ctx, &field, text),
        (_, Some(block)) => render_block(ctx, &field, text, block, reverse, Mode::Classic, layout),
        (FieldDirection::Reverse, None) => {
            let reversed: String = text.chars().rev().collect();
            render_line(ctx, &field, &Shown::Text(reversed), *justification);
        }
        (FieldDirection::Horizontal, None) => render_line(ctx, &field, &Shown::Plain(text), *justification),
    }
}

/// A line ready to draw.
enum Shown<'a> {
    Plain(&'a str),
    Text(String),
    /// Shaped glyphs; `true` prints the clusters in reverse order (^FPR).
    Shaped(ShapedLine, bool),
}

impl Field {
    fn width_of(&self, fonts: &FontRegistry, line: &Shown) -> f32 {
        match line {
            Shown::Plain(s) => self.measure(fonts, s),
            Shown::Text(s) => self.measure(fonts, s),
            Shown::Shaped(l, _) => l.width,
        }
    }

    fn show_line(&self, ctx: &mut Ctx, line: &Shown, x: f32, baseline_y: f32) {
        match line {
            Shown::Plain(s) => self.show(ctx, s, x, baseline_y),
            Shown::Text(s) => self.show(ctx, s, x, baseline_y),
            Shown::Shaped(l, reverse) => self.show_shaped(ctx, l, x, baseline_y, 0.0, *reverse),
        }
    }
}

/// A single unblocked line.
fn render_line(ctx: &mut Ctx, field: &Field, text: &Shown, justification: Justification) {
    let (x, y, size_y) = (field.x, field.y, field.size_y);
    let advance = field.width_of(&ctx.fonts, text);
    let shift = |w: f32| match justification {
        Justification::Left => 0.0,
        Justification::Right => w,
        Justification::Auto => w / 2.0,
    };

    if field.anchor == TextAnchor::Baseline {
        // ^FT: the origin is the start of the baseline in the rotated frame.
        let matrix = field.frame(0.0, 0.0);
        let u = -shift(advance);
        field.in_frame(ctx, matrix, |ctx, ox, oy| field.show_line(ctx, text, ox + u, oy));
        return;
    }

    if field.bitmap.is_some() {
        // Bitmap cells: the rotated cell box (text width x cell height) starts at the origin.
        let matrix = field.frame(advance, field.line_h);
        let u = -shift(advance);
        field.in_frame(ctx, matrix, |ctx, ox, oy| field.show_line(ctx, text, ox + u, oy + field.ascent));
        return;
    }

    // Box width as the PNG renderer computes it.
    let text_width = advance.ceil() + BOX_PAD;
    // Baseline inside an unrotated box whose top is the field origin.
    let baseline = field.ascent;

    match field.orientation {
        Orientation::Normal => {
            let ax = x - shift(text_width);
            field.show_line(ctx, text, ax, y + baseline);
        }
        Orientation::Rotated90 => {
            // Text box is `text_width` x `2*height`, rotated 90° clockwise: reads top to
            // bottom with glyph tops facing right; rotated box width is 2*height.
            let box_h = 2.0 * size_y;
            let ax = x - shift(box_h);
            with_transform(ctx, [0.0, 1.0, -1.0, 0.0, ax + box_h, y], |ctx| {
                field.show_line(ctx, text, 0.0, baseline);
            });
        }
        Orientation::Rotated180 => {
            let box_h = 2.0 * size_y;
            let ax = x - shift(text_width);
            with_transform(ctx, [-1.0, 0.0, 0.0, -1.0, ax + text_width, y + box_h], |ctx| {
                field.show_line(ctx, text, 0.0, baseline);
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
                field.show_line(ctx, text, 0.0, baseline);
            });
        }
    }
}

/// ^FPV: one character per line, top to bottom, each starting at the origin's x.
fn render_vertical(ctx: &mut Ctx, field: &Field, text: &str) {
    let step = field.line_h + field.gap;
    let count = text.chars().count();
    let widest = text.chars().map(|c| field.char_width(&ctx.fonts, c)).fold(0.0, f32::max);
    let first = match field.anchor {
        TextAnchor::Top => field.ascent,
        TextAnchor::Baseline => 0.0,
    };
    let matrix = field.frame(widest, step * count as f32);
    field.in_frame(ctx, matrix, |ctx, ox, oy| {
        let mut buf = [0u8; 4];
        for (i, c) in text.chars().enumerate() {
            let s = c.encode_utf8(&mut buf);
            field.show(ctx, s, ox, oy + first + i as f32 * step);
        }
    });
}


/// One wrapped line of a block: a byte range into the unescaped text.
struct Line {
    range: Range<usize>,
    /// Index of the paragraph (between `\&` breaks) the line belongs to.
    para: usize,
    /// Last line of a paragraph (before `\&` or the end): never stretched when justified.
    para_end: bool,
}

/// Resolves the ^FB escapes: `\\` is a backslash and `\&` a forced line break. Returns the
/// text with breaks removed and the byte ranges of the paragraphs between them.
#[allow(clippy::single_range_in_vec_init)] // one paragraph, not a range of indices
fn split_paragraphs(text: &str) -> (Cow<'_, str>, Vec<Range<usize>>) {
    if !text.contains('\\') {
        return (Cow::Borrowed(text), vec![0..text.len()]);
    }
    let mut clean = String::with_capacity(text.len());
    let mut paragraphs = Vec::new();
    let mut start = 0;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, chars.peek()) {
            ('\\', Some('&')) => {
                chars.next();
                paragraphs.push(start..clean.len());
                start = clean.len();
            }
            ('\\', Some('\\')) => {
                chars.next();
                clean.push('\\');
            }
            _ => clean.push(c),
        }
    }
    paragraphs.push(start..clean.len());
    (Cow::Owned(clean), paragraphs)
}

/// Greedy word wrap of `text[para]` (paragraph number `index`) into lines no wider than
/// `avail(line_index)` dots (`None` = no limit). `width(byte, i, c)` is the width that
/// character `c` at byte offset `byte` adds as the `i`-th byte of a line, gaps included.
/// Breaks at spaces; a word wider than a line is broken between characters. The spaces at
/// a break are dropped.
fn wrap(
    text: &str,
    para: Range<usize>,
    index: usize,
    avail: impl Fn(usize) -> Option<f32>,
    width: impl Fn(usize, usize, char) -> f32,
    lines: &mut Vec<Line>,
) {
    let mut start = para.start;
    loop {
        let limit = avail(lines.len());
        let rest = &text[start..para.end];
        let mut w = 0.0;
        let mut last_space = None;
        let mut cut = None;
        if let Some(limit) = limit {
            for (i, c) in rest.char_indices() {
                let cw = width(start + i, i, c);
                if c == ' ' {
                    // Only a space after some content is a usable break.
                    if i > 0 {
                        last_space = Some(i);
                    }
                } else if i > 0 && w + cw > limit + 0.01 {
                    cut = Some(i);
                    break;
                }
                w += cw;
            }
        }
        let Some(cut) = cut else {
            lines.push(Line { range: start..para.end, para: index, para_end: true });
            return;
        };
        let end = match last_space {
            Some(sp) => {
                // Break at the start of the run of spaces before the overflowing word.
                let line = &rest[..sp];
                line.trim_end_matches(' ').len()
            }
            None => cut,
        };
        let end = if end == 0 { cut } else { end };
        lines.push(Line { range: start..start + end, para: index, para_end: false });
        let next = start + end;
        let skipped = text[next..para.end].len() - text[next..para.end].trim_start_matches(' ').len();
        start = next + skipped;
        if start >= para.end {
            if let Some(last) = lines.last_mut() {
                last.para_end = true;
            }
            return;
        }
    }
}

/// ^FB / ^TB: word-wrapped block of lines. Lines are wrapped in logical order; with ^PA
/// bidi each line is then reordered on its own (UBA rule L), with its paragraph's level.
#[allow(clippy::too_many_arguments)]
fn render_block(ctx: &mut Ctx, field: &Field, text: &str, block: &TextBlock, reverse: bool, mode: Mode, layout: &TextLayout) {
    let (clean, paragraphs) = split_paragraphs(text);
    let text: &str = &clean;
    let indent = block.hanging_indent as f32;
    let width = block.width as f32;
    let indent_of = |i: usize| if i == 0 { 0.0 } else { indent };
    let avail = |i: usize| (block.width > 0).then(|| (width - indent_of(i)).max(0.0));

    let bidi: Vec<_> = match mode {
        Mode::Classic => Vec::new(),
        _ => paragraphs.iter().map(|p| shape::bidi_info(layout, &text[p.clone()])).collect(),
    };
    // Right-to-left paragraphs end justified (^FB J) blocks on the right.
    let rtl = |para: usize| bidi.get(para).and_then(Option::as_ref).is_some_and(|i| i.paragraphs[0].level.is_rtl());
    let features = match mode {
        Mode::Shaped => shape::Features::new(layout),
        _ => shape::Features::default(),
    };

    let mut lines = Vec::new();
    for (index, para) in paragraphs.iter().enumerate() {
        let fonts = &ctx.fonts;
        if mode == Mode::Shaped {
            // Measure with the paragraph shaped as a whole: per-cluster advances.
            let p = &text[para.clone()];
            let shaped = shape::shape(fonts, field, p, 0..p.len(), bidi[index].as_ref(), &features);
            let (adv, starts) = shaped.byte_advances(p.len());
            let gap = field.gap;
            let at = |b: usize, i: usize, _| {
                let k = b - para.start;
                adv[k] + if i > 0 && starts[k] { gap } else { 0.0 }
            };
            wrap(text, para.clone(), index, avail, at, &mut lines);
        } else {
            let at = |_, i: usize, c| field.char_width(fonts, c) + if i > 0 { field.gap } else { 0.0 };
            wrap(text, para.clone(), index, avail, at, &mut lines);
        }
    }

    let line_height = (field.line_h + block.line_spacing as f32).max(1.0);
    let max_lines = block.max_lines.max(1) as usize;
    // Rows actually occupied; lines past them are dropped (^TB) or overprinted on the last row (^FB).
    let (rows, keep) = match block.clip_height {
        Some(h) => {
            let fit = (h as f32 / line_height).floor() as usize;
            (lines.len().min(fit), lines.len().min(fit))
        }
        None => (lines.len().min(max_lines), lines.len()),
    };
    if rows == 0 {
        return;
    }

    let shown: Vec<Shown> = lines[..keep]
        .iter()
        .map(|l| match mode {
            Mode::Classic => Shown::Plain(&text[l.range.clone()]),
            _ => {
                let para = &paragraphs[l.para];
                let p = &text[para.clone()];
                let range = l.range.start - para.start..l.range.end - para.start;
                let info = bidi[l.para].as_ref();
                if mode == Mode::Bidi {
                    Shown::Text(shape::visual_string(info, p, range))
                } else {
                    Shown::Shaped(shape::shape(&ctx.fonts, field, p, range, info, &features), reverse)
                }
            }
        })
        .collect();
    let widths: Vec<f32> = shown.iter().map(|l| field.width_of(&ctx.fonts, l)).collect();
    let box_w = if block.width > 0 { width } else { widths.iter().copied().fold(0.0, f32::max) };
    let ascent = field.ascent;
    let baseline_of = |row: usize| match field.anchor {
        TextAnchor::Top => row as f32 * line_height + ascent,
        // ^FT: the origin is the baseline of the last line.
        TextAnchor::Baseline => -((rows - 1 - row) as f32 * line_height),
    };
    let matrix = field.frame(box_w, rows as f32 * line_height);

    field.in_frame(ctx, matrix, |ctx, ox, oy| {
        let mut reversed = String::new();
        for (i, ((line, shown), &line_w)) in lines[..keep].iter().zip(&shown).zip(&widths).enumerate() {
            let row = i.min(rows - 1);
            let baseline = oy + baseline_of(row);
            let room = if block.width > 0 { width - indent_of(i) } else { box_w - indent_of(i) };
            let left = ox + indent_of(i);
            let mut s = match shown {
                Shown::Plain(s) => *s,
                Shown::Text(s) => s.as_str(),
                Shown::Shaped(l, reverse) => {
                    if l.glyphs.is_empty() {
                        continue;
                    }
                    let spaces = l.spaces();
                    let (x, extra) = match block.justification {
                        BlockJustification::Left => (left, 0.0),
                        BlockJustification::Center => (left + (room - line_w) / 2.0, 0.0),
                        BlockJustification::Right => (left + room - line_w, 0.0),
                        BlockJustification::Justified if line.para_end || spaces == 0 || line_w >= room => {
                            (if rtl(line.para) { left + room - line_w } else { left }, 0.0)
                        }
                        BlockJustification::Justified => (left, (room - line_w) / spaces as f32),
                    };
                    field.show_shaped(ctx, l, x, baseline, extra, *reverse);
                    continue;
                }
            };
            if s.is_empty() {
                continue;
            }
            if reverse {
                reversed.clear();
                reversed.extend(s.chars().rev());
                s = &reversed;
            }
            match block.justification {
                BlockJustification::Left => field.show(ctx, s, left, baseline),
                BlockJustification::Center => field.show(ctx, s, left + (room - line_w) / 2.0, baseline),
                BlockJustification::Right => field.show(ctx, s, left + room - line_w, baseline),
                BlockJustification::Justified => {
                    let spaces = s.matches(' ').count();
                    if line.para_end || spaces == 0 || line_w >= room {
                        let x = if rtl(line.para) { left + room - line_w } else { left };
                        field.show(ctx, s, x, baseline);
                    } else {
                        // Words drawn one by one with the slack spread over the spaces.
                        let extra = (room - line_w) / spaces as f32;
                        let space = field.char_width(&ctx.fonts, ' ') + field.gap;
                        let mut pen = left;
                        for (k, word) in s.split(' ').enumerate() {
                            if k > 0 {
                                pen += space + extra + field.gap;
                            }
                            field.show(ctx, word, pen, baseline);
                            pen += field.measure(&ctx.fonts, word);
                        }
                    }
                }
            }
        }
    });
}

impl Field {
    /// Draws a shaped line with its left edge at `x` and baseline at `baseline_y`: one `Tm`
    /// per stretch of glyphs in one font on one baseline offset, the shaped positions
    /// expressed as `TJ` adjustments against the embedded advance widths. `extra` widens
    /// every space cluster (justification); `reverse` prints the clusters in reverse order.
    fn show_shaped(&self, ctx: &mut Ctx, line: &ShapedLine, x: f32, baseline_y: f32, extra: f32, reverse: bool) {
        struct Placed {
            key: FontKey,
            size: [f32; 2],
            code: u16,
            x: f32,
            y: f32,
            /// Advance the PDF viewer applies after the glyph (its /W width), in dots.
            width: f32,
        }
        if line.glyphs.is_empty() {
            return;
        }
        let clusters: Vec<Range<usize>> = if reverse { line.clusters().rev().collect() } else { line.clusters().collect() };
        let mut placed = Vec::with_capacity(line.glyphs.len());
        let mut pen = x;
        for (k, cluster) in clusters.iter().enumerate() {
            if k > 0 {
                pen += line.gap;
            }
            for g in &line.glyphs[cluster.clone()] {
                let mut buf = [0u8; 4];
                let code = ctx.fonts.use_glyph(g.key, g.gid, line.text_of(g, &mut buf));
                let face = ctx.fonts.face(g.key);
                let w = face.glyph_hor_advance(GlyphId(g.gid)).unwrap_or(0) as f32 / face.units_per_em() as f32;
                placed.push(Placed { key: g.key, size: g.size, code, x: pen + g.dx, y: baseline_y - g.dy, width: w * g.size[0] });
                pen += g.advance;
            }
            if line.glyphs[cluster.start].space {
                pen += extra;
            }
        }

        begin(ctx, self.stroke);
        let mut font = None;
        let mut bytes = Vec::with_capacity(placed.len() * 2);
        let mut i = 0;
        while i < placed.len() {
            let first = &placed[i];
            let mut j = i + 1;
            while j < placed.len() && placed[j].key == first.key && placed[j].size == first.size && placed[j].y == first.y {
                j += 1;
            }
            let content = &mut ctx.content;
            if font != Some(first.key) {
                content.set_font(ResName::of(first.key).name(), 1.0);
                font = Some(first.key);
            }
            let [sx, sy] = first.size;
            content.set_text_matrix([sx, 0.0, 0.0, -sy, first.x, first.y]);
            let mut tj = content.show_positioned();
            let mut items = tj.items();
            bytes.clear();
            for k in i..j {
                if k > i {
                    let prev = &placed[k - 1];
                    // TJ numbers are thousandths of an em, positive = move left.
                    let adjust = ((prev.x + prev.width - placed[k].x) / sx * 1000.0 * 100.0).round() / 100.0;
                    if adjust.abs() >= 0.05 {
                        items.show(Str(&bytes));
                        bytes.clear();
                        items.adjust(adjust);
                    }
                }
                bytes.extend_from_slice(&placed[k].code.to_be_bytes());
            }
            items.show(Str(&bytes));
            drop(items);
            drop(tj);
            i = j;
        }
        end(ctx, false, self.stroke);
    }
}

fn with_transform(ctx: &mut Ctx, matrix: [f32; 6], f: impl FnOnce(&mut Ctx)) {
    ctx.content.save_state();
    ctx.content.transform(matrix);
    f(ctx);
    ctx.content.restore_state();
}


/// Opens a text object; a non-zero `stroke` (line width in dots) fills and strokes the
/// glyphs inside a saved graphics state.
fn begin(ctx: &mut Ctx, stroke: f32) {
    if stroke > 0.0 {
        ctx.content
            .save_state()
            .set_line_width(stroke)
            .set_line_join(LineJoinStyle::RoundJoin)
            .set_stroke_gray(ctx.ink);
    }
    ctx.content.begin_text();
    if stroke > 0.0 {
        ctx.content.set_text_rendering_mode(TextRenderingMode::FillStroke);
    }
}

/// Ends a text object; `reset_tc` restores zero character spacing, since text state
/// outlives ET and later text expects none. A stroked object's state is restored instead.
fn end(ctx: &mut Ctx, reset_tc: bool, stroke: f32) {
    if reset_tc && stroke == 0.0 {
        ctx.content.set_char_spacing(0.0);
    }
    ctx.content.end_text();
    if stroke > 0.0 {
        ctx.content.restore_state();
    }
}

/// Shows `glyphs` (char, original GID) of `key` inside an open text object at em size
/// `size` = [x, y] dots, pen at `x`, baseline at `baseline_y` (y-down user space), with
/// `tc` character spacing in unscaled text space units (0 leaves the current Tc alone,
/// which callers keep at 0 outside their runs).
fn emit_run(
    ctx: &mut Ctx,
    key: FontKey,
    glyphs: impl Iterator<Item = (char, u16)>,
    size: [f32; 2],
    tc: f32,
    x: f32,
    baseline_y: f32,
) {
    let mut bytes = Vec::with_capacity(32);
    let name = ctx.fonts.use_glyph_ids(key, glyphs, &mut bytes);
    put_run(ctx, name, &bytes, size, tc, x, baseline_y);
}

/// [`emit_run`] for already recorded glyph codes.
fn put_run(ctx: &mut Ctx, name: ResName, bytes: &[u8], size: [f32; 2], tc: f32, x: f32, baseline_y: f32) {
    let content = &mut ctx.content;
    content.set_font(name.name(), 1.0);
    if tc != 0.0 {
        content.set_char_spacing(tc);
    }
    content
        // Negative d flips glyphs upright in the y-down user space.
        .set_text_matrix([size[0], 0.0, 0.0, -size[1], x, baseline_y])
        .show(Str(bytes));
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
    begin(ctx, 0.0);
    emit_run(ctx, font.into(), text.chars().map(|c| (c, font.glyph(c))), [size_x, size_y], 0.0, x, baseline_y);
    end(ctx, false, 0.0);
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
    use zpl_parser::command::{FontName, TextLayout};

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
            anchor: TextAnchor::Top,
            block: None,
            direction: FieldDirection::Horizontal,
            char_gap: 0,
            custom_font: None,
            layout: Default::default(),
        }
    }

    /// Like `draw` with every field settable through the closure's struct update.
    fn field(text: &str, f: impl FnOnce(&mut Opts)) -> DrawInstruction {
        let mut o = Opts {
            orientation: Orientation::Normal,
            anchor: TextAnchor::Top,
            block: None,
            direction: FieldDirection::Horizontal,
            char_gap: 0,
        };
        f(&mut o);
        DrawInstruction::DrawText {
            x: 50,
            y: 60,
            text: text.into(),
            font: FontName::Font0,
            height: 40,
            width: 0,
            orientation: o.orientation,
            justification: Justification::Left,
            anchor: o.anchor,
            block: o.block,
            direction: o.direction,
            char_gap: o.char_gap,
            custom_font: None,
            layout: Default::default(),
        }
    }

    struct Opts {
        orientation: Orientation,
        anchor: TextAnchor,
        block: Option<TextBlock>,
        direction: FieldDirection,
        char_gap: u32,
    }

    fn block(width: u32, max_lines: u32, justification: BlockJustification) -> TextBlock {
        TextBlock { width, max_lines, line_spacing: 0, justification, hanging_indent: 0, clip_height: None }
    }

    /// Renders one instruction and returns the raw (uncompressed) content stream.
    fn content(ins: &DrawInstruction) -> String {
        let mut ctx = Ctx::new();
        render(&mut ctx, ins);
        String::from_utf8_lossy(&ctx.content.finish()).into_owned()
    }

    /// Every `Tm` operand list in the content stream, as [a b c d e f].
    fn text_matrices(content: &str) -> Vec<[f32; 6]> {
        let tokens: Vec<&str> = content.split_whitespace().collect();
        tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == "Tm")
            .map(|(i, _)| {
                let mut m = [0.0; 6];
                for (k, v) in tokens[i - 6..i].iter().enumerate() {
                    m[k] = v.parse().unwrap();
                }
                m
            })
            .collect()
    }

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.01
    }

    fn em(s: &str) -> f32 {
        s.chars().map(|c| char_advance(FontId::Triumvirate, c)).sum::<f32>() * 40.0
    }

    #[test]
    fn top_anchor_baseline_is_cap_height_below_origin() {
        let tm = text_matrices(&content(&field("Hello", |_| {})));
        assert_eq!(tm.len(), 1);
        assert!(approx(tm[0][4], 50.0) && approx(tm[0][5], 60.0 + 30.0), "{tm:?}");
    }

    #[test]
    fn baseline_anchor_puts_baseline_on_origin() {
        let tm = text_matrices(&content(&field("Hello", |o| o.anchor = TextAnchor::Baseline)));
        assert!(approx(tm[0][4], 50.0) && approx(tm[0][5], 60.0), "{tm:?}");
    }

    #[test]
    fn baseline_anchor_rotated_rotates_around_origin() {
        let c = content(&field("Hello", |o| {
            o.anchor = TextAnchor::Baseline;
            o.orientation = Orientation::Rotated90;
        }));
        assert!(c.contains("0 1 -1 0 50 60 cm"), "{c}");
        let tm = text_matrices(&c);
        assert!(approx(tm[0][4], 0.0) && approx(tm[0][5], 0.0), "{tm:?}");
    }

    #[test]
    fn block_wraps_words_into_lines() {
        // "AAAA BBBB CCCC" in a block that fits about one word per line.
        let w = em("AAAA ") as u32 + 2;
        let c = content(&field("AAAA BBBB CCCC", |o| o.block = Some(block(w, 10, BlockJustification::Left))));
        let tm = text_matrices(&c);
        assert_eq!(tm.len(), 3, "{c}");
        for (i, m) in tm.iter().enumerate() {
            assert!(approx(m[4], 50.0));
            assert!(approx(m[5], 60.0 + 30.0 + 40.0 * i as f32), "{tm:?}");
        }
    }

    #[test]
    fn block_line_spacing_and_hanging_indent() {
        let w = em("AAAA ") as u32 + 2;
        let mut b = block(w + 7, 10, BlockJustification::Left);
        b.line_spacing = 5;
        b.hanging_indent = 7;
        let tm = text_matrices(&content(&field("AAAA BBBB CCCC", |o| o.block = Some(b))));
        assert_eq!(tm.len(), 3);
        assert!(approx(tm[0][4], 50.0) && approx(tm[1][4], 57.0) && approx(tm[2][4], 57.0), "{tm:?}");
        assert!(approx(tm[1][5] - tm[0][5], 45.0) && approx(tm[2][5] - tm[1][5], 45.0), "{tm:?}");
    }

    #[test]
    fn long_word_is_broken_by_characters() {
        let w = em("AAA") as u32 + 1;
        let tm = text_matrices(&content(&field("AAAAAAA", |o| o.block = Some(block(w, 10, BlockJustification::Left)))));
        assert_eq!(tm.len(), 3, "{tm:?}");
    }

    #[test]
    fn forced_breaks_and_escaped_backslash() {
        let c = content(&field(r"A\\B\&C\&\&D", |o| o.block = Some(block(0, 10, BlockJustification::Left))));
        let tm = text_matrices(&c);
        // Lines "A\B", "C", "" (not drawn), "D".
        assert_eq!(tm.len(), 3, "{c}");
        assert!(approx(tm[1][5] - tm[0][5], 40.0) && approx(tm[2][5] - tm[1][5], 80.0), "{tm:?}");
        let (clean, paras) = split_paragraphs(r"A\\B\&C");
        assert_eq!(&clean[paras[0].clone()], r"A\B");
        assert_eq!(&clean[paras[1].clone()], "C");
    }

    #[test]
    fn fb_overflow_overprints_last_line() {
        let w = em("AAAA ") as u32 + 2;
        let tm = text_matrices(&content(&field("AAAA BBBB CCCC DDDD", |o| {
            o.block = Some(block(w, 2, BlockJustification::Left))
        })));
        assert_eq!(tm.len(), 4);
        assert!(approx(tm[0][5], 90.0) && approx(tm[1][5], 130.0));
        assert!(approx(tm[2][5], 130.0) && approx(tm[3][5], 130.0), "{tm:?}");
    }

    #[test]
    fn tb_drops_lines_that_do_not_fit() {
        let w = em("AAAA ") as u32 + 2;
        let mut b = block(w, u32::MAX, BlockJustification::Left);
        b.clip_height = Some(99); // two 40-dot lines
        let tm = text_matrices(&content(&field("AAAA BBBB CCCC DDDD", |o| o.block = Some(b))));
        assert_eq!(tm.len(), 2, "{tm:?}");
    }

    #[test]
    fn block_baseline_anchor_is_last_line() {
        let w = em("AAAA ") as u32 + 2;
        let tm = text_matrices(&content(&field("AAAA BBBB CCCC", |o| {
            o.block = Some(block(w, 10, BlockJustification::Left));
            o.anchor = TextAnchor::Baseline;
        })));
        assert!(approx(tm[2][5], 60.0) && approx(tm[0][5], -20.0), "{tm:?}");
    }

    #[test]
    fn block_center_and_right_justification() {
        let line = em("AB");
        let c = text_matrices(&content(&field("AB", |o| o.block = Some(block(200, 1, BlockJustification::Center)))));
        assert!(approx(c[0][4], 50.0 + (200.0 - line) / 2.0), "{c:?}");
        let r = text_matrices(&content(&field("AB", |o| o.block = Some(block(200, 1, BlockJustification::Right)))));
        assert!(approx(r[0][4], 50.0 + 200.0 - line), "{r:?}");
    }

    #[test]
    fn justified_lines_fill_width_except_last() {
        let w = em("AA BB CC") as u32 + 30;
        let tm = text_matrices(&content(&field("AA BB CC DDD", |o| {
            o.block = Some(block(w, 10, BlockJustification::Justified))
        })));
        // First line drawn word by word, last word ending at the right edge; last line in one run.
        assert_eq!(tm.len(), 4, "{tm:?}");
        assert!(approx(tm[2][4] + em("CC"), 50.0 + w as f32), "{tm:?}");
        assert!(approx(tm[3][4], 50.0));
    }

    #[test]
    fn rotated_block_rotates_whole_block() {
        let w = em("AAAA ") as u32 + 2;
        let c = content(&field("AAAA BBBB", |o| {
            o.block = Some(block(w, 10, BlockJustification::Left));
            o.orientation = Orientation::Rotated90;
        }));
        // Two lines of 40 dots: the rotated box spans x 50..130.
        assert!(c.contains("0 1 -1 0 130 60 cm"), "{c}");
        let tm = text_matrices(&c);
        assert!(approx(tm[0][5], 30.0) && approx(tm[1][5], 70.0), "{tm:?}");
    }

    #[test]
    fn vertical_direction_stacks_characters() {
        let tm = text_matrices(&content(&field("ABC", |o| {
            o.direction = FieldDirection::Vertical;
            o.char_gap = 5;
        })));
        assert_eq!(tm.len(), 3);
        for (i, m) in tm.iter().enumerate() {
            assert!(approx(m[4], 50.0) && approx(m[5], 90.0 + 45.0 * i as f32), "{tm:?}");
        }
    }

    #[test]
    fn char_gap_sets_character_spacing() {
        let c = content(&field("ABC", |o| o.char_gap = 8));
        // 8 dots at a 40-dot em is 0.2 text-space units; reset afterwards.
        assert!(c.contains("0.2 Tc"), "{c}");
        assert!(c.contains("0 Tc"), "{c}");
        let none = content(&field("ABC", |_| {}));
        assert!(!none.contains("Tc"));
    }

    #[test]
    fn reverse_direction_draws_characters_backwards() {
        // Glyph ids are assigned in first-use order, so render after a plain "AB" each time.
        let pair = |second: DrawInstruction| {
            let mut ctx = Ctx::new();
            render(&mut ctx, &field("AB", |_| {}));
            render(&mut ctx, &second);
            ctx.content.finish().to_vec()
        };
        let reversed = pair(field("AB", |o| o.direction = FieldDirection::Reverse));
        assert_eq!(reversed, pair(field("BA", |_| {})));
        assert_ne!(reversed, pair(field("AB", |_| {})));
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

    fn with_font(text: &str, font: FontName, height: u32, width: u32) -> DrawInstruction {
        let mut ins = draw(text, Orientation::Normal, Justification::Left);
        if let DrawInstruction::DrawText { font: f, height: h, width: w, .. } = &mut ins {
            *f = font;
            *h = height;
            *w = width;
        }
        ins
    }

    fn with_custom(mut ins: DrawInstruction, data: Vec<u8>) -> DrawInstruction {
        if let DrawInstruction::DrawText { custom_font, .. } = &mut ins {
            *custom_font = Some(std::sync::Arc::new(data));
        }
        ins
    }

    /// Operand of every `Tc` operator.
    fn char_spacings(content: &str) -> Vec<f32> {
        let tokens: Vec<&str> = content.split_whitespace().collect();
        tokens.windows(2).filter(|w| w[1] == "Tc").map(|w| w[0].parse().unwrap()).collect()
    }

    #[test]
    fn bitmap_matrices_follow_the_docs() {
        let a = bitmap_cell(FontName::FontA, 8).unwrap();
        assert_eq!((a.height, a.width, a.gap, a.baseline, a.uppercase), (9, 5, 1, 7, false));
        let e = bitmap_cell(FontName::FontE, 8).unwrap();
        assert_eq!((e.height, e.width, e.gap, e.baseline), (28, 15, 5, 23));
        assert_eq!(bitmap_cell(FontName::FontE, 12).unwrap().height, 42);
        let h = bitmap_cell(FontName::FontH, 8).unwrap();
        assert_eq!((h.height, h.width, h.gap, h.baseline, h.uppercase), (21, 13, 6, 21, true));
        assert!(bitmap_cell(FontName::Font0, 8).is_none());
        assert!(bitmap_cell(FontName::FontP, 8).is_none());
    }

    #[test]
    fn magnification_rounds_to_nearest_multiple() {
        assert_eq!(magnification(9, 5, 9, 5), (1, 1));
        assert_eq!(magnification(36, 20, 18, 10), (2, 2));
        assert_eq!(magnification(36, 40, 18, 10), (2, 4));
        // Only a height (the state manager reports width == height): same magnification.
        assert_eq!(magnification(30, 30, 9, 5), (3, 3));
        assert_eq!(magnification(15, 15, 9, 5), (2, 2));
        assert_eq!(magnification(0, 20, 18, 10), (2, 2));
        assert_eq!(magnification(0, 0, 18, 10), (1, 1));
        assert_eq!(magnification(4, 1, 18, 10), (1, 1));
        assert_eq!(magnification(1000, 1000, 9, 5), (10, 10));
    }

    #[test]
    fn bitmap_font_uses_fixed_cells() {
        // ^CFA,30: magnification 3 -> 27 x 15 cells, 18-dot pitch, baseline 21 dots down.
        let ins = with_font("Ab1", FontName::FontA, 30, 30);
        let c = content(&ins);
        assert!(c.contains("/F1 1 Tf") && c.contains("2 Tr"), "{c}");
        let tm = text_matrices(&c);
        assert_eq!(tm.len(), 1);
        let m = builtin_metrics(BITMAP_FONT);
        // Cap height of the stroked outline == 21 dots; 'M' ink == 15 dots wide.
        let stroke = BITMAP_STROKE_EM * 21.0;
        assert!(approx(-tm[0][3] * m.cap + stroke, 21.0), "{tm:?}");
        assert!(approx(tm[0][0] * m.ink_m.1 + stroke, 15.0), "{tm:?}");
        assert!(approx(tm[0][5], 60.0 + 21.0), "{tm:?}");
        // Every character advances exactly one pitch.
        let tc = char_spacings(&c)[0];
        assert!(approx((m.advance_m + tc) * tm[0][0], 18.0), "{tc}");
        let mut ctx = Ctx::new();
        let field = field_for(&mut ctx, FontName::FontA, None, 30, 30, 0.0, 0.0, 0.0, Orientation::Normal, TextAnchor::Top);
        assert!(approx(field.measure(&ctx.fonts, "Ab1"), 54.0));
        assert!(approx(field.line_h, 27.0) && approx(field.ascent, 21.0));
    }

    #[test]
    fn bitmap_font_default_size_and_char_gap() {
        // Font D with no size: one 18 x 10 cell per character, pitch 12, plus the ^FP gap.
        let mut ins = with_font("XY", FontName::FontD, 0, 0);
        if let DrawInstruction::DrawText { char_gap, .. } = &mut ins {
            *char_gap = 3;
        }
        let c = content(&ins);
        let tm = text_matrices(&c);
        let m = builtin_metrics(BITMAP_FONT);
        let tc = char_spacings(&c)[0];
        assert!(approx((m.advance_m + tc) * tm[0][0], 15.0), "{c}");
        assert!(approx(tm[0][5], 60.0 + 14.0));
        // Text state is restored with the graphics state around the stroked text.
        assert!(c.trim_end().ends_with("ET\nQ"), "{c}");
    }

    #[test]
    fn uppercase_only_bitmap_fonts_capitalise() {
        // Glyph codes are assigned in first-use order: render both into one context.
        let shown = |font: FontName| {
            let mut ctx = Ctx::new();
            render(&mut ctx, &with_font("ABC", font, 0, 0));
            render(&mut ctx, &with_font("abc", font, 0, 0));
            let c = String::from_utf8_lossy(&ctx.content.finish()).into_owned();
            let tj: Vec<String> = c.lines().filter(|l| l.ends_with("Tj")).map(str::to_owned).collect();
            tj[0] == tj[1]
        };
        assert!(shown(FontName::FontB) && shown(FontName::FontH));
        assert!(!shown(FontName::FontD));
    }

    #[test]
    fn fonts_p_to_v_are_fixed_size_font_zero() {
        let tm = text_matrices(&content(&with_font("Hi", FontName::FontP, 0, 0)));
        assert!(approx(tm[0][0], 18.0) && approx(tm[0][3], -20.0), "{tm:?}");
        let tm = text_matrices(&content(&with_font("Hi", FontName::FontQ, 56, 56)));
        assert!(approx(tm[0][0], 48.0) && approx(tm[0][3], -56.0), "{tm:?}");
    }

    #[test]
    fn missing_glyphs_fall_back_per_character() {
        assert!(FontId::Triumvirate.face().glyph_index('Ж').is_none());
        let c = content(&draw("AЖB", Orientation::Normal, Justification::Left));
        assert!(c.contains("/F0 1 Tf") && c.contains("/F2 1 Tf"), "{c}");
        let tm = text_matrices(&c);
        assert_eq!(tm.len(), 3, "{c}");
        // Each run starts where the previous one ended, measured with its own font.
        let fonts = FontRegistry::default();
        let a = char_advance(FontId::Triumvirate, 'A') * 40.0;
        assert!(approx(tm[1][4], 50.0 + a), "{tm:?}");
        let field = Field {
            font: FontId::Triumvirate.into(),
            x: 0.0,
            y: 0.0,
            size_x: 40.0,
            size_y: 40.0,
            gap: 0.0,
            line_h: 40.0,
            ascent: 30.0,
            bitmap: None,
            stroke: 0.0,
            orientation: Orientation::Normal,
            anchor: TextAnchor::Top,
            notdef: false,
        };
        let zhe = field.char_width(&fonts, 'Ж');
        assert!(approx(tm[2][4], 50.0 + a + zhe), "{tm:?}");
        assert!(approx(field.measure(&fonts, "AЖB"), a + zhe + char_advance(FontId::Triumvirate, 'B') * 40.0));
        let pdf = PdfRenderer::new(400, 300).render(&[draw("AЖB", Orientation::Normal, Justification::Left)]);
        assert!(contains(&pdf, b"DejaVuSans-Bold"));
    }

    #[test]
    fn bitmap_fallback_glyph_keeps_the_pitch() {
        // A glyph only DejaVu Sans Bold has.
        let ch = ['\u{2133}', '\u{2766}', '\u{1F0A1}', '\u{2622}']
            .into_iter()
            .find(|&c| BITMAP_FONT.face().glyph_index(c).is_none() && FontId::DejaVuSansBold.face().glyph_index(c).is_some());
        let ch = ch.expect("a glyph only DejaVu Sans Bold has");
        let text: String = ['A', ch, 'B'].iter().collect();
        let tm = text_matrices(&content(&with_font(&text, FontName::FontD, 18, 10)));
        assert_eq!(tm.len(), 3, "{tm:?}");
        // 'B' sits two pitches after 'A'.
        assert!(approx(tm[2][4] - tm[0][4], 24.0), "{tm:?}");
    }

    #[test]
    fn custom_truetype_font_is_embedded_and_subset() {
        let ins = with_custom(draw("Hello", Orientation::Normal, Justification::Left), FontId::DejaVuMono.data().to_vec());
        let c = content(&ins);
        assert!(c.contains("/C0 1 Tf"), "{c}");
        let tm = text_matrices(&c);
        // Em size = height; top of the caps on the origin.
        let cap = cap_height(FontId::DejaVuMono.face());
        assert!(approx(tm[0][0], 40.0) && approx(tm[0][5], 60.0 + cap * 40.0), "{tm:?}");
        let pdf = PdfRenderer::new(400, 300).render(&[ins.clone(), ins]);
        assert!(contains(&pdf, b"/FontFile2") && contains(&pdf, b"/CIDFontType2"));
        assert!(contains(&pdf, b"+DejaVuSansMono"));
        // Subset: far smaller than the 340 KB font.
        assert!(pdf.len() < 30_000, "{}", pdf.len());
    }

    #[test]
    fn custom_cff_font_uses_fontfile3() {
        let ins = with_custom(draw("Hello", Orientation::Normal, Justification::Left), FontId::Triumvirate.data().to_vec());
        assert!(content(&ins).contains("/C0 1 Tf"));
        let pdf = PdfRenderer::new(400, 300).render(&[ins]);
        assert!(contains(&pdf, b"/FontFile3") && contains(&pdf, b"/CIDFontType0C"));
    }

    #[test]
    fn same_custom_font_is_registered_once() {
        let data = std::sync::Arc::new(FontId::DejaVuMono.data().to_vec());
        let mut fonts = FontRegistry::default();
        let a = fonts.register_custom(&data);
        let b = fonts.register_custom(&std::sync::Arc::new(data.to_vec()));
        assert!(a.is_some() && a == b);
    }

    #[test]
    fn unparseable_custom_font_falls_back_to_font_zero() {
        let ins = with_custom(draw("Hello", Orientation::Normal, Justification::Left), b"not a font".to_vec());
        let c = content(&ins);
        assert!(c.contains("/F0 1 Tf") && !c.contains("/C0"), "{c}");
        let pdf = PdfRenderer::new(400, 300).render(&[ins]);
        assert!(contains(&pdf, b"ATTriumvirate"));
    }

    // ^PA advanced text layout.

    fn layout(default_glyph: bool, bidi: bool, shaping: bool, opentype: bool) -> TextLayout {
        TextLayout { default_glyph, bidi, shaping, opentype }
    }

    fn with_layout(mut ins: DrawInstruction, l: TextLayout) -> DrawInstruction {
        if let DrawInstruction::DrawText { layout, .. } = &mut ins {
            *layout = l;
        }
        ins
    }

    /// A Font0 field (40 dots) as `render` builds it, with ^PA default glyph `notdef`.
    fn font0_field(ctx: &mut Ctx, notdef: bool) -> Field {
        let mut f = field_for(ctx, FontName::Font0, None, 40, 0, 0.0, 0.0, 0.0, Orientation::Normal, TextAnchor::Top);
        f.notdef = notdef;
        f
    }

    fn shaped(text: &str, l: TextLayout) -> ShapedLine {
        let mut ctx = Ctx::new();
        let field = font0_field(&mut ctx, l.default_glyph);
        let info = shape::bidi_info(&l, text);
        shape::shape(&ctx.fonts, &field, text, 0..text.len(), info.as_ref(), &shape::Features::new(&l))
    }

    /// Glyph IDs a font maps `text` to through its cmap, one per character.
    fn cmap_gids(font: FontId, text: &str) -> Vec<u16> {
        text.chars().map(|c| font.glyph(c)).collect()
    }

    /// Every FlateDecode stream of `pdf`, inflated.
    fn streams(pdf: &[u8]) -> Vec<String> {
        let mut out = Vec::new();
        let mut rest = pdf;
        while let Some(i) = rest.windows(7).position(|w| w == b"stream\n") {
            let body = &rest[i + 7..];
            let end = body.windows(10).position(|w| w == b"\nendstream").unwrap_or(body.len());
            if let Ok(data) = miniz_oxide::inflate::decompress_to_vec_zlib(&body[..end]) {
                out.push(String::from_utf8_lossy(&data).into_owned());
            }
            rest = &body[(end + 10).min(body.len())..];
        }
        out
    }

    #[test]
    fn hebrew_bidi_reverses_visual_order_and_keeps_numbers_ltr() {
        let l = layout(false, true, false, false);
        let info = shape::bidi_info(&l, "שלום 123");
        // RTL paragraph (first strong character is Hebrew): the number stays left to right.
        assert_eq!(shape::visual_string(info.as_ref(), "שלום 123", 0.."שלום 123".len()), "123 םולש");
        let info = shape::bidi_info(&l, "abc שלום (x)");
        assert_eq!(shape::visual_string(info.as_ref(), "abc שלום (x)", 0.."abc שלום".len()), "abc םולש");
        // Mirrored brackets inside a right-to-left run.
        let t = "שלום (עולם)";
        let info = shape::bidi_info(&l, t);
        assert_eq!(shape::visual_string(info.as_ref(), t, 0..t.len()), "(םלוע) םולש");

        // Shaped path: glyphs in visual order, the number's digits left to right.
        let line = shaped("שלום 123", layout(false, true, true, false));
        let clusters: Vec<u32> = line.glyphs.iter().map(|g| g.cluster).collect();
        assert_eq!(clusters, [9, 10, 11, 8, 6, 4, 2, 0]);
        let hebrew: Vec<u16> = line.glyphs[4..].iter().map(|g| g.gid).collect();
        assert_eq!(hebrew, cmap_gids(FontId::DejaVuSansBold, "םולש"));

        // Through the renderer: one bidi-only field draws the reordered text.
        let pair = |a: DrawInstruction, b: DrawInstruction| {
            let render_one = |ins: &DrawInstruction| {
                let mut ctx = Ctx::new();
                render(&mut ctx, ins);
                ctx.content.finish().to_vec()
            };
            render_one(&a) == render_one(&b)
        };
        let plain = draw("123 םולש", Orientation::Normal, Justification::Left);
        assert!(pair(with_layout(draw("שלום 123", Orientation::Normal, Justification::Left), l), plain));
    }

    #[test]
    fn arabic_shaping_produces_contextual_forms() {
        let text = "بيت";
        let isolated = cmap_gids(FontId::DejaVuSansBold, text);
        let line = shaped(text, layout(false, true, true, false));
        assert_eq!(line.glyphs.len(), 3);
        assert!(line.glyphs.iter().all(|g| g.key == FontKey::Builtin(FontId::DejaVuSansBold)));
        // Visual order is right to left: last letter first, every one in a joining form.
        let gids: Vec<u16> = line.glyphs.iter().map(|g| g.gid).collect();
        for (shaped, iso) in gids.iter().zip(isolated.iter().rev()) {
            assert_ne!(shaped, iso, "{gids:?} vs cmap {isolated:?}");
        }
        // Without shaping (bidi only) the letters keep their isolated cmap glyphs.
        let mut ctx = Ctx::new();
        let field = font0_field(&mut ctx, false);
        assert!(text.chars().all(|c| resolve(&ctx.fonts, field.font, c, false).1 == FontId::DejaVuSansBold.glyph(c)));
        // Lam-alef is a required ligature even without OpenType features.
        assert_eq!(shaped("لا", layout(false, true, true, false)).glyphs.len(), 1);
        // Without bidi the run is still shaped right to left (same forms), but kept in
        // logical order.
        let logical = shaped(text, layout(false, false, true, false));
        let rev: Vec<u16> = logical.glyphs.iter().rev().map(|g| g.gid).collect();
        assert_eq!(rev, gids);
        assert_eq!(shaped("لا", layout(false, false, true, false)).glyphs.len(), 1);
    }

    #[test]
    fn ligatures_only_with_opentype() {
        let on = shaped("office", layout(false, false, false, true));
        // o, f, fi, c, e
        assert_eq!(on.glyphs.len(), 5, "{:?}", on.glyphs);
        let mut buf = [0u8; 4];
        assert_eq!(on.text_of(&on.glyphs[2], &mut buf), "fi");
        assert!(!cmap_gids(FontId::Triumvirate, "office").contains(&on.glyphs[2].gid));
        let off = shaped("office", layout(false, false, true, false));
        let gids: Vec<u16> = off.glyphs.iter().map(|g| g.gid).collect();
        assert_eq!(gids, cmap_gids(FontId::Triumvirate, "office"));
        // Shaping without kerning keeps the classic advances.
        let mut ctx = Ctx::new();
        let field = font0_field(&mut ctx, false);
        assert!(approx(off.width, field.measure(&ctx.fonts, "office")));
    }

    #[test]
    fn tounicode_maps_shaped_glyphs_to_source_text() {
        let pdf = PdfRenderer::new(600, 300).render(&[
            with_layout(draw("office", Orientation::Normal, Justification::Left), layout(false, true, true, true)),
            with_layout(draw("بيت", Orientation::Normal, Justification::Left), layout(false, true, true, true)),
        ]);
        let cmaps: String = streams(&pdf).into_iter().filter(|s| s.contains("beginbfchar") || s.contains("beginbfrange")).collect();
        let cmaps = cmaps.to_uppercase();
        // The "ffi" or "fi" ligature maps back to its characters.
        assert!(cmaps.contains("<00660069>") || cmaps.contains("<006600660069>"), "{cmaps}");
        // Arabic contextual forms map to the base letters.
        for c in ["<0628>", "<064A>", "<062A>"] {
            assert!(cmaps.contains(c), "{c} missing in {cmaps}");
        }
        // Shaped glyphs are drawn with TJ.
        let c = content(&with_layout(draw("بيت office", Orientation::Normal, Justification::Left), layout(false, true, true, true)));
        assert!(c.contains("TJ"), "{c}");
    }

    #[test]
    fn layout_off_is_unchanged() {
        let render_one = |ins: &DrawInstruction| {
            let mut ctx = Ctx::new();
            render(&mut ctx, ins);
            ctx.content.finish().to_vec()
        };
        for text in ["Hello 123", "AЖB", "שלום مرحبا"] {
            let plain = draw(text, Orientation::Normal, Justification::Left);
            // Bidi on without right-to-left text is the classic path too.
            let bidi = with_layout(plain.clone(), layout(false, true, false, false));
            assert_eq!(render_one(&plain), render_one(&with_layout(plain.clone(), TextLayout::default())));
            if !shape::has_rtl(text) {
                assert_eq!(render_one(&plain), render_one(&bidi));
            }
        }
        assert_eq!(shape::mode(&TextLayout::default(), false, "שלום"), Mode::Classic);
        assert_eq!(shape::mode(&layout(true, false, false, false), false, "שלום"), Mode::Classic);
        assert_eq!(shape::mode(&layout(false, true, false, false), false, "abc"), Mode::Classic);
        assert_eq!(shape::mode(&layout(false, true, false, false), false, "שלום"), Mode::Bidi);
        assert_eq!(shape::mode(&layout(false, true, true, false), true, "שלום"), Mode::Bidi);
        assert_eq!(shape::mode(&layout(false, false, true, false), false, "abc"), Mode::Shaped);
    }

    #[test]
    fn default_glyph_prints_notdef_else_space() {
        let missing = '\u{E000}';
        assert!(FALLBACKS.iter().all(|f| f.face().glyph_index(missing).is_none()));
        let mut ctx = Ctx::new();
        let space = FontId::Triumvirate.glyph(' ');
        let primary = FontKey::Builtin(FontId::Triumvirate);
        assert_eq!(resolve(&ctx.fonts, primary, missing, false), (primary, space, ' '));
        assert_eq!(resolve(&ctx.fonts, primary, missing, true), (primary, 0, missing));
        let field = font0_field(&mut ctx, false);
        assert!(approx(field.char_width(&ctx.fonts, missing), field.char_width(&ctx.fonts, ' ')));
        // Shaped path: same rule.
        let text = "A\u{E000}B";
        assert_eq!(shaped(text, layout(false, false, true, false)).glyphs[1].gid, space);
        assert_eq!(shaped(text, layout(true, false, true, false)).glyphs[1].gid, 0);
    }

    #[test]
    fn shaped_block_wraps_and_right_justifies_rtl_lines() {
        let text = "שלום עולם שלום עולם";
        let l = layout(false, true, true, false);
        let one = shaped("שלום עולם", l).width;
        let ins = with_layout(field(text, |o| o.block = Some(block(one as u32 + 5, 4, BlockJustification::Right))), l);
        let tm = text_matrices(&content(&ins));
        // Two lines, each ending at the right edge of the block.
        let rows: Vec<f32> = tm.iter().map(|m| m[5]).collect();
        assert!(rows.iter().any(|&y| approx(y, 90.0)) && rows.iter().any(|&y| approx(y, 130.0)), "{tm:?}");
        let first_x = tm.iter().filter(|m| approx(m[5], 90.0)).map(|m| m[4]).fold(f32::MAX, f32::min);
        assert!(approx(first_x, 50.0 + (one as u32 + 5) as f32 - one), "{tm:?} {one}");
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
                anchor: TextAnchor::Top,
                block: None,
                direction: FieldDirection::Horizontal,
                char_gap: 0,
                custom_font: None,
                layout: Default::default(),
            })
            .collect();
            std::fs::write(&path, PdfRenderer::new(406, 406).render(&ins)).unwrap();
        }
    }
}
