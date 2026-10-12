//! ^PA advanced text layout for the PNG renderer (best effort, mirrors pdf-renderer's
//! text/shape.rs in a simpler form).
//!
//! Lines are laid out as glyph runs: Unicode bidi reordering per line (paragraph level from
//! the line's first strong character), DejaVu Sans Bold as fallback for characters the
//! field's font lacks, and — with ^PA character shaping or OpenType support — rustybuzz
//! shaping over the same font bytes; glyphs are then drawn by glyph ID with ab_glyph, so
//! contextual forms and ligatures that have no cmap entry still render. Bitmap fonts keep
//! their fixed cells and only get bidi reordering. All-off ^PA never gets here.

use std::sync::OnceLock;

use ab_glyph::{Font, FontArc, GlyphId, PxScale, ScaleFont, point};
use image::{Rgb, RgbImage};
use rustybuzz::ttf_parser::Tag;
use rustybuzz::{Direction, Feature, UnicodeBuffer};
use unicode_bidi::{BidiClass, BidiInfo};
use zpl_parser::command::TextLayout;

/// DejaVu Sans Bold: per-character fallback (Hebrew, Arabic, Greek, Cyrillic, symbols).
/// Shared with pdf-renderer so the font is embedded in the binary once.
pub static DEJAVU_SANS_BOLD: &[u8] = include_bytes!("../../pdf-renderer/fonts/DejaVuSans-Bold.ttf");

/// Horizontal squeeze of fallback glyphs next to the condensed Triumvirate (as in the PDF):
/// Labelary's Greek and Cyrillic advance about 0.7 as far as DejaVu Sans Bold's.
const FALLBACK_CONDENSE: f32 = 0.7;

/// Whether `layout` changes how `text` is laid out (otherwise the classic path draws it).
pub(crate) fn active(layout: &TextLayout, text: &str) -> bool {
    layout.shaping || layout.opentype || (layout.bidi && has_rtl(text))
}

fn has_rtl(text: &str) -> bool {
    text.chars().any(|c| {
        c >= '\u{590}'
            && matches!(
                unicode_bidi::bidi_class(c),
                BidiClass::R | BidiClass::AL | BidiClass::RLE | BidiClass::RLO | BidiClass::RLI
            )
    })
}

fn is_attached(c: char) -> bool {
    matches!(unicode_bidi::bidi_class(c), BidiClass::NSM | BidiClass::BN)
}

/// `line` reordered for display (bitmap fonts): right-to-left runs reversed with their
/// marks kept after the base, mirrored characters swapped.
pub(crate) fn visual_string(layout: &TextLayout, line: &str) -> String {
    if !(layout.bidi && has_rtl(line)) {
        return line.to_string();
    }
    let info = BidiInfo::new(line, None);
    let mut out = String::with_capacity(line.len());
    for (run, rtl) in runs(&info, line) {
        let run = &line[run];
        if !rtl {
            out.push_str(run);
            continue;
        }
        let chars: Vec<char> = run.chars().collect();
        let mut end = chars.len();
        while end > 0 {
            let mut start = end - 1;
            while start > 0 && is_attached(chars[start]) {
                start -= 1;
            }
            for &c in &chars[start..end] {
                out.push(unicode_bidi_mirroring::get_mirrored(c).unwrap_or(c));
            }
            end = start;
        }
    }
    out
}

/// Visual runs (byte ranges, right-to-left?) of `text` as one line.
fn runs(info: &BidiInfo, text: &str) -> Vec<(std::ops::Range<usize>, bool)> {
    let mut out = Vec::new();
    for para in &info.paragraphs {
        let (levels, runs) = info.visual_runs(para, para.range.clone());
        out.extend(runs.into_iter().map(|r| {
            let rtl = levels[r.start].is_rtl();
            (r, rtl)
        }));
    }
    if out.is_empty() {
        out.push((0..text.len(), false));
    }
    out
}

/// A font a line can be shown in.
pub(crate) struct LayoutFont {
    pub font: FontArc,
    pub data: FontData,
    pub scale: PxScale,
}

/// Bytes of a font, for the shaper.
#[derive(Clone)]
pub(crate) enum FontData {
    Static(&'static [u8]),
    Shared(std::sync::Arc<Vec<u8>>),
}

impl FontData {
    fn bytes(&self) -> &[u8] {
        match self {
            FontData::Static(b) => b,
            FontData::Shared(b) => b,
        }
    }
}

fn fallback_font() -> &'static FontArc {
    static FONT: OnceLock<FontArc> = OnceLock::new();
    FONT.get_or_init(|| FontArc::try_from_slice(DEJAVU_SANS_BOLD).expect("embedded font must parse"))
}

fn cap_em(font: &FontArc) -> f32 {
    let upem = font.units_per_em().unwrap_or(1000.0);
    font.outline(font.glyph_id('H')).map(|o| (o.bounds.max.y - o.bounds.min.y).abs() / upem).filter(|&c| c > 0.0).unwrap_or(0.7)
}

/// One positioned glyph: pen-relative x and baseline offset (y down), in dots.
pub(crate) struct Placed {
    /// 0 = the field's font, 1 = the fallback.
    font: u8,
    id: GlyphId,
    x: f32,
    dy: f32,
}

/// A laid-out line: glyphs left to right.
pub(crate) struct Line {
    glyphs: Vec<Placed>,
    pub width: f32,
    fallback_scale: PxScale,
}

/// Lays out `text` (one line, logical order) in `primary` per `layout`.
pub(crate) fn layout_line(primary: &LayoutFont, primary_cap_em: f32, layout: &TextLayout, text: &str) -> Line {
    let fallback = fallback_font();
    // Fallback em size: cap heights matched to the field's font, condensed.
    let p_upem = primary.font.units_per_em().unwrap_or(1000.0);
    let p_px_per_em = primary.font.height_unscaled() / p_upem;
    let (em_x, em_y) = (primary.scale.x / p_px_per_em, primary.scale.y / p_px_per_em);
    let k = primary_cap_em / cap_em(fallback);
    let f_upem = fallback.units_per_em().unwrap_or(2048.0);
    let f_px_per_em = fallback.height_unscaled() / f_upem;
    let fallback_scale = PxScale { x: em_x * k * FALLBACK_CONDENSE * f_px_per_em, y: em_y * k * f_px_per_em };
    let fonts: [(&FontArc, PxScale, f32, f32); 2] = [
        (&primary.font, primary.scale, em_x / p_upem, em_y / p_upem),
        (fallback, fallback_scale, em_x * k * FALLBACK_CONDENSE / f_upem, em_y * k / f_upem),
    ];

    let info = (layout.bidi && has_rtl(text)).then(|| BidiInfo::new(text, None));
    let visual: Vec<(std::ops::Range<usize>, Option<bool>)> = match &info {
        Some(info) => runs(info, text).into_iter().map(|(r, rtl)| (r, Some(rtl))).collect(),
        None => vec![(0..text.len(), None)],
    };

    let shaping = layout.shaping || layout.opentype;
    let mut glyphs = Vec::new();
    let mut pen = 0.0;
    for (run, direction) in visual {
        // Segments of one font, logical order.
        let mut segs: Vec<(std::ops::Range<usize>, u8)> = Vec::new();
        for (i, c) in text[run.clone()].char_indices() {
            let at = run.start + i..run.start + i + c.len_utf8();
            let has = |f: &FontArc| f.glyph_id(c).0 != 0;
            let font = match segs.last_mut() {
                Some(last) if is_attached(c) && has(fonts[last.1 as usize].0) => last.1,
                _ if has(&primary.font) => 0,
                _ if has(fallback) => 1,
                _ => 0,
            };
            match segs.last_mut() {
                Some(last) if last.1 == font => last.0.end = at.end,
                _ => segs.push((at, font)),
            }
        }
        let rtl_run = direction == Some(true);
        if rtl_run {
            segs.reverse();
        }
        for (seg, f) in segs {
            let (font, scale, kx, ky) = fonts[f as usize];
            let mut seg_glyphs: Vec<(GlyphId, f32, f32, f32)> = Vec::new(); // id, advance, dx, dy
            if shaping {
                let data = if f == 0 { primary.data.bytes() } else { DEJAVU_SANS_BOLD };
                let Some(face) = rustybuzz::Face::from_slice(data, 0) else { continue };
                let mut buffer = UnicodeBuffer::new();
                buffer.push_str(&text[seg.clone()]);
                if let Some(rtl) = direction {
                    buffer.set_direction(if rtl { Direction::RightToLeft } else { Direction::LeftToRight });
                }
                buffer.guess_segment_properties();
                let rtl = buffer.direction() == Direction::RightToLeft;
                let features = features(layout, buffer.script());
                let out = rustybuzz::shape(&face, &features, buffer);
                for (g, p) in out.glyph_infos().iter().zip(out.glyph_positions()) {
                    seg_glyphs.push((
                        GlyphId(g.glyph_id as u16),
                        p.x_advance as f32 * kx,
                        p.x_offset as f32 * kx,
                        -(p.y_offset as f32) * ky,
                    ));
                }
                if rtl && direction.is_none() {
                    seg_glyphs.reverse();
                }
            } else {
                let scaled = font.as_scaled(scale);
                let mut push = |c: char| {
                    let c = if rtl_run { unicode_bidi_mirroring::get_mirrored(c).unwrap_or(c) } else { c };
                    let id = font.glyph_id(c);
                    seg_glyphs.push((id, scaled.h_advance(id), 0.0, 0.0));
                };
                if rtl_run {
                    let s = &text[seg.clone()];
                    let chars: Vec<char> = s.chars().collect();
                    let mut end = chars.len();
                    while end > 0 {
                        let mut start = end - 1;
                        while start > 0 && is_attached(chars[start]) {
                            start -= 1;
                        }
                        chars[start..end].iter().for_each(|&c| push(c));
                        end = start;
                    }
                } else {
                    text[seg.clone()].chars().for_each(&mut push);
                }
            }
            for (id, advance, dx, dy) in seg_glyphs {
                let (id, advance) = if id.0 == 0 && !layout.default_glyph {
                    // ^PA default glyph off: missing characters print as spaces.
                    let space = font.glyph_id(' ');
                    (space, font.as_scaled(scale).h_advance(space))
                } else {
                    (id, advance)
                };
                glyphs.push(Placed { font: f, id, x: pen + dx, dy });
                pen += advance;
            }
        }
    }
    Line { glyphs, width: pen, fallback_scale }
}

fn features(layout: &TextLayout, script: rustybuzz::Script) -> Vec<Feature> {
    use rustybuzz::script;
    if layout.opentype {
        return Vec::new();
    }
    let joining = [script::ARABIC, script::SYRIAC, script::NKO, script::MONGOLIAN, script::MANDAIC, script::ADLAM];
    let tags: &[&[u8; 4]] = if joining.contains(&script) {
        &[b"kern", b"dlig", b"hlig"]
    } else {
        &[b"liga", b"clig", b"calt", b"kern", b"dlig", b"hlig"]
    };
    // Bounded range: rustybuzz 0.20.1 mis-orders RTL output with a global `kern` off.
    tags.iter().map(|t| Feature::new(Tag::from_bytes(t), 0, 0..u32::MAX as usize - 1)).collect()
}

/// Draws `line` with its left edge at `x` and baseline at `baseline` (y down).
pub(crate) fn draw_line(image: &mut RgbImage, primary: &LayoutFont, line: &Line, x: f32, baseline: f32) {
    let fallback = fallback_font();
    let fallback_scale = line.fallback_scale;
    let (w, h) = (image.width() as i32, image.height() as i32);
    for g in &line.glyphs {
        let (font, scale) = if g.font == 0 { (&primary.font, primary.scale) } else { (fallback, fallback_scale) };
        let glyph = g.id.with_scale_and_position(scale, point(x + g.x, baseline + g.dy));
        let Some(outline) = font.outline_glyph(glyph) else { continue };
        let bounds = outline.px_bounds();
        outline.draw(|px, py, coverage| {
            let (px, py) = (bounds.min.x as i32 + px as i32, bounds.min.y as i32 + py as i32);
            if px < 0 || py < 0 || px >= w || py >= h || coverage <= 0.0 {
                return;
            }
            let pixel = image.get_pixel_mut(px as u32, py as u32);
            let ink = (coverage.min(1.0) * 255.0) as u8;
            *pixel = Rgb(pixel.0.map(|v| v.saturating_sub(ink)));
        });
    }
}
