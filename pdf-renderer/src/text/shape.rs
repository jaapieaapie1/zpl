//! ^PA advanced text layout: Unicode bidi reordering and OpenType shaping (rustybuzz).
//!
//! Three layout modes, picked per field by [`mode`]:
//! - [`Mode::Classic`]: one glyph per character in logical order, exactly as without ^PA.
//!   Also used with bidi on when the field has no right-to-left characters.
//! - [`Mode::Bidi`]: bidi on but neither shaping nor OpenType (or a bitmap font, whose fixed
//!   cells are kept): each line is reordered into a visual string (RTL runs reversed,
//!   brackets mirrored) and drawn by the classic one-glyph-per-character code.
//! - [`Mode::Shaped`]: shaping and/or OpenType on (scalable fonts): each line is split into
//!   bidi runs, then into runs of one font and script, and every run is shaped by rustybuzz
//!   with the font that gets embedded. Glyphs are placed with the shaped advances/offsets.
//!   Without bidi, runs are still shaped in their script's own direction (so joining and
//!   required ligatures are right) but laid out in logical order, as nothing is reordered.
//!
//! Wrapping (^FB/^TB) happens in logical order on the paragraph's shaped advances; each line
//! is then reordered on its own with its paragraph's level (UBA rule L1/L2), and justified
//! (J) right-to-left paragraphs end on the right. ^FPR (reverse) mirrors the finished visual
//! line: clusters (a ligature, a base with its marks) are printed in reverse order, the same
//! as the classic character reversal for Latin text. ^FPV (vertical) prints one character
//! per line in logical order and always uses the classic path. A character no font has
//! prints as a space, or with ^PA default glyph on as the field font's .notdef glyph.

use std::ops::Range;

use rustybuzz::ttf_parser::Tag;
use rustybuzz::{Direction, Feature, UnicodeBuffer};
use unicode_bidi::{BidiClass, BidiInfo};
use unicode_script::{Script, UnicodeScript};
use zpl_parser::command::TextLayout;

use super::{resolve, Field};
use crate::fonts::{FontKey, FontRegistry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Mode {
    Classic,
    Bidi,
    Shaped,
}

/// Layout mode of a field with `layout` showing `text`. All-off ^PA is always classic, so
/// default labels never pay for the advanced path.
pub(super) fn mode(layout: &TextLayout, bitmap: bool, text: &str) -> Mode {
    if !(layout.bidi || layout.shaping || layout.opentype) {
        return Mode::Classic;
    }
    if (layout.shaping || layout.opentype) && !bitmap {
        return Mode::Shaped;
    }
    if layout.bidi && has_rtl(text) {
        Mode::Bidi
    } else {
        Mode::Classic
    }
}

/// Whether `text` has a strong right-to-left character (or RTL embedding/override/isolate).
pub(super) fn has_rtl(text: &str) -> bool {
    text.chars().any(|c| {
        c >= '\u{590}'
            && matches!(
                unicode_bidi::bidi_class(c),
                BidiClass::R | BidiClass::AL | BidiClass::RLE | BidiClass::RLO | BidiClass::RLI
            )
    })
}

/// Bidi analysis of one paragraph, or `None` when it is laid out left to right as is (bidi
/// off, or nothing right-to-left in it). The paragraph level comes from the first strong
/// character (UBA rules P2/P3), as Zebra does.
pub(super) fn bidi_info<'a>(layout: &TextLayout, text: &'a str) -> Option<BidiInfo<'a>> {
    (layout.bidi && has_rtl(text)).then(|| BidiInfo::new(text, None))
}

/// The byte ranges of `line` in visual order (left to right), each with its direction.
fn visual_runs(info: Option<&BidiInfo>, line: Range<usize>) -> Vec<(Range<usize>, bool)> {
    let Some(info) = info else {
        return vec![(line, false)];
    };
    visual_runs_of(info, line)
}

fn visual_runs_of(info: &BidiInfo, line: Range<usize>) -> Vec<(Range<usize>, bool)> {
    let mut out = Vec::new();
    for para in &info.paragraphs {
        let range = para.range.start.max(line.start)..para.range.end.min(line.end);
        if range.is_empty() {
            continue;
        }
        let (levels, runs) = info.visual_runs(para, range);
        out.extend(runs.into_iter().map(|run| {
            let rtl = levels[run.start].is_rtl();
            (run, rtl)
        }));
    }
    out
}

/// Characters that stay with the preceding character (nonspacing marks, joiners).
fn is_attached(c: char) -> bool {
    matches!(unicode_bidi::bidi_class(c), BidiClass::NSM | BidiClass::BN)
}

/// `text[line]` reordered for display: right-to-left runs reversed (each base character
/// keeping its combining marks after it) with mirrored characters (brackets) swapped.
pub(super) fn visual_string(info: Option<&BidiInfo>, text: &str, line: Range<usize>) -> String {
    let mut out = String::with_capacity(line.len());
    let mut cluster = Vec::new();
    for (run, rtl) in visual_runs(info, line) {
        let run = &text[run];
        if !rtl {
            out.push_str(run);
            continue;
        }
        let mut chars = run.chars().rev().peekable();
        while let Some(c) = chars.next() {
            // Marks come before their base when walking backwards: gather them.
            cluster.clear();
            cluster.push(c);
            let mut c = c;
            while is_attached(c) {
                match chars.next() {
                    Some(next) => {
                        cluster.push(next);
                        c = next;
                    }
                    None => break,
                }
            }
            for &ch in cluster.iter().rev() {
                out.push(unicode_bidi_mirroring::get_mirrored(ch).unwrap_or(ch));
            }
        }
    }
    out
}

/// OpenType features for the shaping runs of a field. With OpenType support on, the
/// shaper's defaults (ligatures, kerning, contextual alternates, mark positioning, ...).
/// With only character shaping on, just what correct shaping needs: the joining forms
/// (init/medi/fina/isol), rlig, ccmp, locl and mark/mkmk stay on; kerning and
/// discretionary/contextual substitutions are switched off so Latin text stays one glyph
/// per character at its nominal advances. Joining scripts keep `liga` too, since fonts such
/// as DejaVu put the mandatory lam-alef ligature there.
#[derive(Default)]
pub(super) struct Features {
    other: Vec<Feature>,
    joining: Vec<Feature>,
}

impl Features {
    pub fn new(layout: &TextLayout) -> Self {
        if layout.opentype {
            return Self::default();
        }
        // A bounded range rather than `..`: rustybuzz 0.20.1 leaves right-to-left output in
        // logical order when `kern` is switched off globally (the whole-buffer range); a
        // range covering every cluster (byte offset) masks the lookups the same way.
        let all = 0..u32::MAX as usize - 1;
        let off = |tags: &[&[u8; 4]]| tags.iter().map(|tag| Feature::new(Tag::from_bytes(tag), 0, all.clone())).collect();
        Features {
            other: off(&[b"liga", b"clig", b"calt", b"kern", b"dlig", b"hlig"]),
            joining: off(&[b"kern", b"dlig", b"hlig"]),
        }
    }

    fn for_script(&self, script: rustybuzz::Script) -> &[Feature] {
        use rustybuzz::script;
        let joining = [
            script::ARABIC,
            script::SYRIAC,
            script::NKO,
            script::MONGOLIAN,
            script::MANDAIC,
            script::ADLAM,
            script::HANIFI_ROHINGYA,
            script::PHAGS_PA,
            script::MANICHAEAN,
            script::PSALTER_PAHLAVI,
            script::SOGDIAN,
        ];
        if joining.contains(&script) { &self.joining } else { &self.other }
    }
}

/// One shaped glyph.
#[derive(Debug, Clone)]
pub(super) struct Glyph {
    pub key: FontKey,
    pub gid: u16,
    /// Em size [x, y] in dots.
    pub size: [f32; 2],
    /// Pen advance in dots.
    pub advance: f32,
    /// Offset from the pen position, in dots (y up).
    pub dx: f32,
    pub dy: f32,
    /// Byte offset of the glyph's cluster in the shaped text: glyphs of one cluster are
    /// adjacent and the ^FP gap goes between clusters.
    pub cluster: u32,
    /// Source text the glyph stands for (ToUnicode), as a byte range of the shaped text.
    pub text: Range<u32>,
    /// Character the glyph stands for instead of `text`: ' ' for a missing character
    /// printed as a space, the mirrored character for a mirrored bracket. A glyph has one
    /// ToUnicode entry, and a mirrored bracket glyph is also the plain glyph of its mirror
    /// image, so it maps to the character it shows, as in the classic and bidi-only paths.
    pub alt: Option<char>,
    /// The cluster is a space (justification stretches it).
    pub space: bool,
}

/// A shaped line: glyphs in visual order, left to right.
#[derive(Debug, Clone, Default)]
pub(super) struct ShapedLine {
    pub glyphs: Vec<Glyph>,
    /// Total advance in dots, ^FP gaps between clusters included.
    pub width: f32,
    pub gap: f32,
    /// The shaped line's text; glyph text ranges are offsets into the full text, `base`
    /// bytes before it.
    source: String,
    base: u32,
}

impl ShapedLine {
    /// The source characters `glyph` stands for (ToUnicode).
    pub fn text_of<'a>(&'a self, glyph: &Glyph, buf: &'a mut [u8; 4]) -> &'a str {
        if let Some(c) = glyph.alt {
            return c.encode_utf8(buf);
        }
        let range = glyph.text.start.saturating_sub(self.base) as usize..glyph.text.end.saturating_sub(self.base) as usize;
        self.source.get(range).unwrap_or("")
    }

    /// Ranges of glyphs belonging to one cluster, left to right.
    pub fn clusters(&self) -> impl DoubleEndedIterator<Item = Range<usize>> + '_ {
        let mut starts = Vec::new();
        for (i, g) in self.glyphs.iter().enumerate() {
            if i == 0 || self.glyphs[i - 1].cluster != g.cluster {
                starts.push(i);
            }
        }
        let n = self.glyphs.len();
        (0..starts.len()).map(move |k| starts[k]..starts.get(k + 1).copied().unwrap_or(n))
    }

    pub fn spaces(&self) -> usize {
        self.clusters().filter(|c| self.glyphs[c.start].space).count()
    }

    /// Advance in dots that each byte of the shaped text contributes, credited to the first
    /// byte of its cluster, and whether that byte starts a cluster. For line wrapping.
    pub fn byte_advances(&self, len: usize) -> (Vec<f32>, Vec<bool>) {
        let mut adv = vec![0.0; len];
        let mut start = vec![false; len];
        for g in &self.glyphs {
            let i = (g.cluster - self.base) as usize;
            if i < len {
                adv[i] += g.advance;
                start[i] = true;
            }
        }
        (adv, start)
    }
}

/// Shapes `text[line]` for `field`. `info` is the bidi analysis of `text` (`None` = no
/// reordering). Joining looks at the line's text only.
pub(super) fn shape(
    fonts: &FontRegistry,
    field: &Field,
    text: &str,
    line: Range<usize>,
    info: Option<&BidiInfo>,
    features: &Features,
) -> ShapedLine {
    let mut out = ShapedLine {
        glyphs: Vec::with_capacity(line.len()),
        width: 0.0,
        gap: field.gap,
        source: text[line.clone()].to_owned(),
        base: line.start as u32,
    };
    let mut segs = Vec::new();
    match info {
        Some(info) => {
            for (run, rtl) in visual_runs_of(info, line.clone()) {
                segs.clear();
                segments(fonts, field, text, run, &mut segs);
                if rtl {
                    for (seg, key) in segs.iter().rev() {
                        shape_segment(fonts, field, text, seg.clone(), *key, Some(true), line.clone(), features, &mut out.glyphs);
                    }
                } else {
                    for (seg, key) in &segs {
                        shape_segment(fonts, field, text, seg.clone(), *key, Some(false), line.clone(), features, &mut out.glyphs);
                    }
                }
            }
        }
        // No reordering (bidi off): every run is shaped in its script's own direction, so
        // joining and required ligatures come out right, and laid out in logical order.
        None => {
            segments(fonts, field, text, line.clone(), &mut segs);
            for (seg, key) in &segs {
                shape_segment(fonts, field, text, seg.clone(), *key, None, line.clone(), features, &mut out.glyphs);
            }
        }
    }
    let clusters = out.clusters().count();
    out.width = out.glyphs.iter().map(|g| g.advance).sum::<f32>() + field.gap * clusters.saturating_sub(1) as f32;
    out
}

/// Splits the run `text[run]` (one direction) into maximal stretches shown in one font and
/// one script, in logical order. Characters resolve through the field's fallback chain;
/// marks and joiners stay with the preceding character's font when it has them, and
/// script-neutral characters (spaces, digits, punctuation) never start a new script.
fn segments(fonts: &FontRegistry, field: &Field, text: &str, run: Range<usize>, out: &mut Vec<(Range<usize>, FontKey)>) {
    let mut script = Script::Common;
    for (i, c) in text[run.clone()].char_indices() {
        let at = run.start + i..run.start + i + c.len_utf8();
        if let Some(last) = out.last_mut()
            && is_attached(c)
            && fonts.face(last.1).glyph_index(c).is_some()
        {
            last.0.end = at.end;
            continue;
        }
        let key = resolve(fonts, field.font, c, field.notdef).0;
        let sc = c.script();
        let neutral = matches!(sc, Script::Common | Script::Inherited | Script::Unknown);
        if let Some(last) = out.last_mut()
            && last.1 == key
            && (neutral || script == Script::Common || script == sc)
        {
            last.0.end = at.end;
            if !neutral {
                script = sc;
            }
            continue;
        }
        script = if neutral { Script::Common } else { sc };
        out.push((at, key));
    }
}

/// Shapes `text[seg]` in `key` and appends its glyphs to `out`: in visual order for a given
/// direction (`Some(rtl)`), in logical order when shaped in the script's own direction (`None`).
#[allow(clippy::too_many_arguments)]
fn shape_segment(
    fonts: &FontRegistry,
    field: &Field,
    text: &str,
    seg: Range<usize>,
    key: FontKey,
    direction: Option<bool>,
    context: Range<usize>,
    features: &Features,
    out: &mut Vec<Glyph>,
) {
    let face = fonts.face(key);
    let mut buffer = UnicodeBuffer::new();
    for (i, c) in text[seg.clone()].char_indices() {
        buffer.add(c, (seg.start + i) as u32);
    }
    buffer.set_pre_context(&text[context.start..seg.start]);
    buffer.set_post_context(&text[seg.end..context.end]);
    if let Some(rtl) = direction {
        buffer.set_direction(if rtl { Direction::RightToLeft } else { Direction::LeftToRight });
    }
    buffer.guess_segment_properties();
    let rtl = buffer.direction() == Direction::RightToLeft;
    let features = features.for_script(buffer.script());
    let shaped = rustybuzz::shape(fonts.shaper(key), features, buffer);
    let infos = shaped.glyph_infos();
    let positions = shaped.glyph_positions();

    let size = if key == field.font {
        [field.size_x, field.size_y]
    } else {
        let (sx, sy) = field.fallback_size(fonts, key);
        [sx, sy]
    };
    let upem = face.units_per_em() as f32;
    let (kx, ky) = (size[0] / upem, size[1] / upem);
    let space = face.glyph_index(' ').map_or(0, |g| g.0);

    // Cluster boundaries: a cluster ends where the next (in logical order) begins.
    let mut starts: Vec<u32> = infos.iter().map(|g| g.cluster).collect();
    starts.sort_unstable();
    starts.dedup();
    let end_of = |c: u32| starts.get(starts.partition_point(|&s| s <= c)).copied().unwrap_or(seg.end as u32);

    let first = out.len();
    for (info, pos) in infos.iter().zip(positions) {
        let cluster = info.cluster;
        let range = cluster..end_of(cluster);
        let mut glyph = Glyph {
            key,
            gid: info.glyph_id as u16,
            size,
            advance: pos.x_advance as f32 * kx,
            dx: pos.x_offset as f32 * kx,
            dy: pos.y_offset as f32 * ky,
            cluster,
            text: range.clone(),
            alt: None,
            space: &text[range.start as usize..range.end as usize] == " ",
        };
        if glyph.gid == 0 && !field.notdef && space != 0 {
            // ^PA default glyph off: a missing character prints as a space.
            glyph.gid = space;
            glyph.advance = face.glyph_hor_advance(ttf_parser::GlyphId(space)).unwrap_or(0) as f32 * kx;
            glyph.alt = Some(' ');
        } else if rtl {
            let mut chars = text[range.start as usize..range.end as usize].chars();
            if let (Some(c), None) = (chars.next(), chars.next())
                && let Some(m) = unicode_bidi_mirroring::get_mirrored(c)
                && face.glyph_index(m).map(|g| g.0) == Some(glyph.gid)
            {
                glyph.alt = Some(m);
            }
        }
        out.push(glyph);
    }

    if rtl && direction.is_none() {
        out[first..].reverse();
    }
    let rtl = rtl && direction.is_some();

    // Clusters of several glyphs (decompositions, marks): each glyph that is the cmap glyph
    // of one of the cluster's characters stands for that character; if none is, the
    // logically first glyph stands for the whole cluster and the others for nothing.
    let glyphs = &mut out[first..];
    let mut i = 0;
    while i < glyphs.len() {
        let mut j = i + 1;
        while j < glyphs.len() && glyphs[j].cluster == glyphs[i].cluster {
            j += 1;
        }
        if j - i > 1 {
            let whole = glyphs[i].text.clone();
            let mut any = false;
            for g in &mut glyphs[i..j] {
                let hit = text[whole.start as usize..whole.end as usize]
                    .char_indices()
                    .find(|&(_, c)| face.glyph_index(c).map(|x| x.0) == Some(g.gid));
                g.text = match hit {
                    Some((k, c)) => {
                        any = true;
                        let s = whole.start + k as u32;
                        s..s + c.len_utf8() as u32
                    }
                    None => whole.start..whole.start,
                };
            }
            if !any {
                let lead = if rtl { j - 1 } else { i };
                glyphs[lead].text = whole;
            }
        }
        i = j;
    }
}
