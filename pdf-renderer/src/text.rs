use std::borrow::Cow;
use std::ops::Range;

use crate::{Ctx, FontId};
use pdf_writer::Str;
use ttf_parser::GlyphId;
use zpl_parser::command::{BlockJustification, FieldDirection, Justification, Orientation};
use zpl_state_manager::{DrawInstruction, TextAnchor, TextBlock};

/// Zebra places the top of capital letters at the field origin; like the PNG renderer the
/// cap height is taken as 0.75 em.
const CAP_HEIGHT_EM: f32 = 0.75;

/// Extra width the PNG renderer adds to its text boxes (`estimate_text_width`); kept so
/// justified/rotated fields land on the same dots.
const BOX_PAD: f32 = 10.0;

/// Layout parameters shared by the single-line, block and vertical paths.
struct Field {
    font: FontId,
    x: f32,
    y: f32,
    size_x: f32,
    size_y: f32,
    /// ^FP inter-character gap in dots.
    gap: f32,
    orientation: Orientation,
    anchor: TextAnchor,
}

impl Field {
    /// Width in dots of `text` laid out on one line, including the ^FP gaps between characters.
    fn measure(&self, text: &str) -> f32 {
        let mut n = 0usize;
        let adv: f32 = text
            .chars()
            .map(|c| {
                n += 1;
                char_advance(self.font, c)
            })
            .sum();
        adv * self.size_x + self.gap * n.saturating_sub(1) as f32
    }

    /// Distance from the top of a line to its baseline.
    fn ascent(&self) -> f32 {
        CAP_HEIGHT_EM * self.size_y
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

    fn show(&self, ctx: &mut Ctx, text: &str, x: f32, baseline_y: f32) {
        show_text_spaced(ctx, self.font, text, x, baseline_y, self.size_x, self.size_y, self.gap);
    }
}

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    let DrawInstruction::DrawText {
        x,
        y,
        text,
        height,
        width,
        orientation,
        justification,
        anchor,
        block,
        direction,
        char_gap,
        ..
    } = instruction
    else {
        return;
    };
    if text.is_empty() {
        return;
    }

    // ZPL font sizes are em sizes; width 0 means "same as height".
    let size_y = *height as f32;
    let field = Field {
        font: FontId::Triumvirate,
        x: *x as f32,
        y: *y as f32,
        size_y,
        size_x: if *width == 0 { size_y } else { *width as f32 },
        gap: *char_gap as f32,
        orientation: *orientation,
        anchor: *anchor,
    };

    match (direction, block) {
        (FieldDirection::Vertical, _) => render_vertical(ctx, &field, text),
        (_, Some(block)) => render_block(ctx, &field, text, block, matches!(direction, FieldDirection::Reverse)),
        (FieldDirection::Reverse, None) => {
            let reversed: String = text.chars().rev().collect();
            render_line(ctx, &field, &reversed, *justification);
        }
        (FieldDirection::Horizontal, None) => render_line(ctx, &field, text, *justification),
    }
}

/// A single unblocked line.
fn render_line(ctx: &mut Ctx, field: &Field, text: &str, justification: Justification) {
    let (x, y, size_y) = (field.x, field.y, field.size_y);
    let advance = field.measure(text);
    let shift = |w: f32| match justification {
        Justification::Left => 0.0,
        Justification::Right => w,
        Justification::Auto => w / 2.0,
    };

    if field.anchor == TextAnchor::Baseline {
        // ^FT: the origin is the start of the baseline in the rotated frame.
        let matrix = field.frame(0.0, 0.0);
        let u = -shift(advance);
        field.in_frame(ctx, matrix, |ctx, ox, oy| field.show(ctx, text, ox + u, oy));
        return;
    }

    // Box width as the PNG renderer computes it.
    let text_width = advance.ceil() + BOX_PAD;
    // Baseline inside an unrotated box whose top is the field origin.
    let baseline = field.ascent();

    match field.orientation {
        Orientation::Normal => {
            let ax = x - shift(text_width);
            field.show(ctx, text, ax, y + baseline);
        }
        Orientation::Rotated90 => {
            // Text box is `text_width` x `2*height`, rotated 90° clockwise: reads top to
            // bottom with glyph tops facing right; rotated box width is 2*height.
            let box_h = 2.0 * size_y;
            let ax = x - shift(box_h);
            with_transform(ctx, [0.0, 1.0, -1.0, 0.0, ax + box_h, y], |ctx| {
                field.show(ctx, text, 0.0, baseline);
            });
        }
        Orientation::Rotated180 => {
            let box_h = 2.0 * size_y;
            let ax = x - shift(text_width);
            with_transform(ctx, [-1.0, 0.0, 0.0, -1.0, ax + text_width, y + box_h], |ctx| {
                field.show(ctx, text, 0.0, baseline);
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
                field.show(ctx, text, 0.0, baseline);
            });
        }
    }
}

/// ^FPV: one character per line, top to bottom, each starting at the origin's x.
fn render_vertical(ctx: &mut Ctx, field: &Field, text: &str) {
    let step = field.size_y + field.gap;
    let count = text.chars().count();
    let widest = text.chars().map(|c| char_advance(field.font, c)).fold(0.0, f32::max) * field.size_x;
    let first = match field.anchor {
        TextAnchor::Top => field.ascent(),
        TextAnchor::Baseline => 0.0,
    };
    let matrix = field.frame(widest, step * count as f32);
    field.in_frame(ctx, matrix, |ctx, ox, oy| {
        let mut buf = [0u8; 4];
        for (i, c) in text.chars().enumerate() {
            let s = c.encode_utf8(&mut buf);
            show_text(ctx, field.font, s, ox, oy + first + i as f32 * step, field.size_x, field.size_y);
        }
    });
}

/// One wrapped line of a block: a byte range into the unescaped text.
struct Line {
    range: Range<usize>,
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

/// Greedy word wrap of `text[para]` into lines no wider than `avail(line_index)` dots
/// (`None` = no limit). Breaks at spaces; a word wider than a line is broken between
/// characters. The spaces at a break are dropped.
fn wrap(field: &Field, text: &str, para: Range<usize>, avail: impl Fn(usize) -> Option<f32>, lines: &mut Vec<Line>) {
    let mut start = para.start;
    loop {
        let limit = avail(lines.len());
        let rest = &text[start..para.end];
        let mut w = 0.0;
        let mut last_space = None;
        let mut cut = None;
        if let Some(limit) = limit {
            for (i, c) in rest.char_indices() {
                let cw = char_advance(field.font, c) * field.size_x + if i > 0 { field.gap } else { 0.0 };
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
            lines.push(Line { range: start..para.end, para_end: true });
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
        lines.push(Line { range: start..start + end, para_end: false });
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

/// ^FB / ^TB: word-wrapped block of lines.
fn render_block(ctx: &mut Ctx, field: &Field, text: &str, block: &TextBlock, reverse: bool) {
    let (clean, paragraphs) = split_paragraphs(text);
    let text: &str = &clean;
    let indent = block.hanging_indent as f32;
    let width = block.width as f32;
    let indent_of = |i: usize| if i == 0 { 0.0 } else { indent };
    let avail = |i: usize| (block.width > 0).then(|| (width - indent_of(i)).max(0.0));

    let mut lines = Vec::new();
    for para in paragraphs {
        wrap(field, text, para, avail, &mut lines);
    }

    let line_height = (field.size_y + block.line_spacing as f32).max(1.0);
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

    let widths: Vec<f32> = lines[..keep].iter().map(|l| field.measure(&text[l.range.clone()])).collect();
    let box_w = if block.width > 0 { width } else { widths.iter().copied().fold(0.0, f32::max) };
    let ascent = field.ascent();
    let baseline_of = |row: usize| match field.anchor {
        TextAnchor::Top => row as f32 * line_height + ascent,
        // ^FT: the origin is the baseline of the last line.
        TextAnchor::Baseline => -((rows - 1 - row) as f32 * line_height),
    };
    let matrix = field.frame(box_w, rows as f32 * line_height);

    field.in_frame(ctx, matrix, |ctx, ox, oy| {
        let mut reversed = String::new();
        for (i, (line, &line_w)) in lines[..keep].iter().zip(&widths).enumerate() {
            let mut s = &text[line.range.clone()];
            if s.is_empty() {
                continue;
            }
            if reverse {
                reversed.clear();
                reversed.extend(s.chars().rev());
                s = &reversed;
            }
            let row = i.min(rows - 1);
            let baseline = oy + baseline_of(row);
            let room = if block.width > 0 { width - indent_of(i) } else { box_w - indent_of(i) };
            let left = ox + indent_of(i);
            match block.justification {
                BlockJustification::Left => field.show(ctx, s, left, baseline),
                BlockJustification::Center => field.show(ctx, s, left + (room - line_w) / 2.0, baseline),
                BlockJustification::Right => field.show(ctx, s, left + room - line_w, baseline),
                BlockJustification::Justified => {
                    let spaces = s.matches(' ').count();
                    if line.para_end || spaces == 0 || line_w >= room {
                        field.show(ctx, s, left, baseline);
                    } else {
                        // Words drawn one by one with the slack spread over the spaces.
                        let extra = (room - line_w) / spaces as f32;
                        let space = char_advance(field.font, ' ') * field.size_x + field.gap;
                        let mut pen = left;
                        for (k, word) in s.split(' ').enumerate() {
                            if k > 0 {
                                pen += space + extra + field.gap;
                            }
                            field.show(ctx, word, pen, baseline);
                            pen += field.measure(word);
                        }
                    }
                }
            }
        }
    });
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
    show_text_spaced(ctx, font, text, x, baseline_y, size_x, size_y, 0.0);
}

/// [`show_text`] with `gap` extra dots after every character (^FP), via the `Tc` operator.
#[allow(clippy::too_many_arguments)]
fn show_text_spaced(
    ctx: &mut Ctx,
    font: FontId,
    text: &str,
    x: f32,
    baseline_y: f32,
    size_x: f32,
    size_y: f32,
    gap: f32,
) {
    if text.is_empty() {
        return;
    }
    let (name, gids) = ctx.fonts.use_glyphs(font, text);
    let bytes: Vec<u8> = gids.iter().flat_map(|g| g.to_be_bytes()).collect();
    let content = &mut ctx.content;
    content.begin_text().set_font(name, 1.0);
    if gap != 0.0 {
        // Tc is in unscaled text space; the text matrix scales it by size_x.
        content.set_char_spacing(gap / size_x);
    }
    content
        // Negative d flips glyphs upright in the y-down user space.
        .set_text_matrix([size_x, 0.0, 0.0, -size_y, x, baseline_y])
        .show(Str(&bytes));
    if gap != 0.0 {
        // Text state outlives ET; reset it for later text.
        content.set_char_spacing(0.0);
    }
    content.end_text();
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
            anchor: TextAnchor::Top,
            block: None,
            direction: FieldDirection::Horizontal,
            char_gap: 0,
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
            })
            .collect();
            std::fs::write(&path, PdfRenderer::new(406, 406).render(&ins)).unwrap();
        }
    }
}
