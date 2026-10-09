//! Font embedding: Type0/CIDFontType0|2 fonts with Identity-H encoding and ToUnicode CMaps.
//!
//! Fonts are subset to the glyphs actually used (via `subsetter`), which keeps both the file
//! size and the deflate time small. The subsetter renumbers glyphs consecutively and gives
//! CFF fonts an identity GID -> CID charset, so text is shown with 2-byte codes that are the
//! *remapped* glyph IDs: for TrueType `/CIDToGIDMap /Identity` maps CID == GID, and for CFF
//! the CIDs address the subset's glyphs directly.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use pdf_writer::types::{CidFontType, FontFlags, SystemInfo, UnicodeCmap};
use pdf_writer::{Filter, Finish, Name, Pdf, Rect, Ref, Str};
use subsetter::GlyphRemapper;
use ttf_parser::{Face, GlyphId, RawFace, Tag};

/// Fonts available to the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontId {
    /// ATTriumvirate-BoldCd.otf (CFF) — ZPL scalable font 0 / text default.
    Triumvirate,
    /// DejaVuSansMono.ttf — barcode interpretation lines.
    DejaVuMono,
}

static TRIUMVIRATE_DATA: &[u8] = include_bytes!("../../png-renderer/fonts/ATTriumvirate-BoldCd.otf");
static DEJAVU_MONO_DATA: &[u8] = include_bytes!("../../png-renderer/fonts/DejaVuSansMono.ttf");

impl FontId {
    /// Raw font file bytes.
    pub(crate) fn data(self) -> &'static [u8] {
        match self {
            FontId::Triumvirate => TRIUMVIRATE_DATA,
            FontId::DejaVuMono => DEJAVU_MONO_DATA,
        }
    }

    /// Parsed font face (parsed once, cached for the process lifetime).
    pub(crate) fn face(self) -> &'static Face<'static> {
        static TRIUMVIRATE: OnceLock<Face<'static>> = OnceLock::new();
        static DEJAVU_MONO: OnceLock<Face<'static>> = OnceLock::new();
        let cell = match self {
            FontId::Triumvirate => &TRIUMVIRATE,
            FontId::DejaVuMono => &DEJAVU_MONO,
        };
        cell.get_or_init(|| Face::parse(self.data(), 0).expect("embedded font must parse"))
    }

    /// Page resource name of this font.
    pub(crate) fn resource_name(self) -> Name<'static> {
        match self {
            FontId::Triumvirate => Name(b"F0"),
            FontId::DejaVuMono => Name(b"F1"),
        }
    }

    /// PostScript name used as /BaseFont.
    fn base_font(self) -> &'static str {
        match self {
            FontId::Triumvirate => "ATTriumvirate-BoldCd",
            FontId::DejaVuMono => "DejaVuSansMono",
        }
    }

    /// Glyph ID for `ch` (0 / .notdef when the font has no glyph for it).
    pub(crate) fn glyph(self, ch: char) -> u16 {
        self.face().glyph_index(ch).map(|g| g.0).unwrap_or(0)
    }
}

#[derive(Default)]
pub(crate) struct FontRegistry {
    used: BTreeMap<FontId, UsedGlyphs>,
}

/// Glyphs used from one font.
struct UsedGlyphs {
    /// Original GID -> subset GID.
    remapper: GlyphRemapper,
    /// Subset GID -> (original GID, the text it represents for ToUnicode).
    glyphs: BTreeMap<u16, (u16, String)>,
}

impl Default for UsedGlyphs {
    fn default() -> Self {
        // .notdef always maps to 0.
        Self { remapper: GlyphRemapper::new(), glyphs: BTreeMap::from([(0, (0, String::new()))]) }
    }
}

impl FontRegistry {
    /// Records the glyphs needed to show `text` in `font`. Returns the font's resource
    /// name and the subset glyph IDs (2-byte CIDs) in display order.
    pub fn use_glyphs(&mut self, font: FontId, text: &str) -> (Name<'static>, Vec<u16>) {
        let used = self.used.entry(font).or_default();
        let gids = text
            .chars()
            .map(|ch| {
                let gid = font.glyph(ch);
                let cid = used.remapper.remap(gid);
                used.glyphs.entry(cid).or_insert_with(|| (gid, if gid == 0 { String::new() } else { ch.to_string() }));
                cid
            })
            .collect();
        (font.resource_name(), gids)
    }

    /// Subsets and deflates the program of every used font (the bulk of the document's
    /// cost) while `main` runs. With `parallel`, each font gets its own scoped thread;
    /// otherwise everything runs on the calling thread. Programs are in `write` order.
    pub fn build_programs<R: Send>(&self, parallel: bool, main: impl FnOnce() -> R) -> (Vec<FontProgram>, R) {
        if !parallel {
            let result = main();
            let programs = self.used.iter().map(|(&font, used)| FontProgram::build(font, used)).collect();
            return (programs, result);
        }
        std::thread::scope(|scope| {
            let handles: Vec<_> = self
                .used
                .iter()
                .map(|(&font, used)| scope.spawn(move || FontProgram::build(font, used)))
                .collect();
            let result = main();
            let programs = handles.into_iter().map(|h| h.join().expect("font program thread panicked")).collect();
            (programs, result)
        })
    }

    /// Writes every used font into `pdf`; returns (resource name, font dict ref).
    /// `programs` comes from [`FontRegistry::build_programs`].
    pub fn write(&self, pdf: &mut Pdf, alloc: &mut Ref, programs: Vec<FontProgram>) -> Vec<(String, Ref)> {
        self.used
            .iter()
            .zip(programs)
            .map(|((&font, used), program)| {
                let id = write_font(pdf, alloc, font, used, program);
                (String::from_utf8_lossy(font.resource_name().0).into_owned(), id)
            })
            .collect()
    }
}

/// A subset font program, deflated and ready to embed.
pub(crate) struct FontProgram {
    data: Vec<u8>,
    /// Bare CFF table (FontFile3 /CIDFontType0C) rather than a TrueType file (FontFile2).
    cff: bool,
    /// Uncompressed length (TrueType /Length1).
    raw_len: usize,
}

impl FontProgram {
    fn build(font: FontId, used: &UsedGlyphs) -> Self {
        let subset = subsetter::subset(font.data(), 0, &used.remapper).expect("embedded font must subset");
        // CFF outlines: embed the bare (now CID-keyed) CFF table.
        let is_cff = font.face().raw_face().table(Tag::from_bytes(b"CFF ")).is_some();
        let program = if is_cff {
            let raw = RawFace::parse(&subset, 0).expect("subset font must parse");
            raw.table(Tag::from_bytes(b"CFF ")).expect("subset keeps CFF table")
        } else {
            &subset
        };
        FontProgram { data: crate::compress(program), cff: is_cff, raw_len: program.len() }
    }
}

/// Six-uppercase-letter subset tag (PDF 32000 9.6.4), derived from the glyph set.
fn subset_tag<'a>(gids: impl Iterator<Item = &'a u16>) -> String {
    // FNV-1a over the glyph IDs.
    let mut hash: u64 = 0xcbf29ce484222325;
    for gid in gids {
        for byte in gid.to_be_bytes() {
            hash = (hash ^ byte as u64).wrapping_mul(0x100000001b3);
        }
    }
    (0..6).map(|i| (b'A' + ((hash >> (i * 8)) % 26) as u8) as char).collect()
}

fn write_font(pdf: &mut Pdf, alloc: &mut Ref, font: FontId, used: &UsedGlyphs, program: FontProgram) -> Ref {
    let glyphs = &used.glyphs;
    let type0_id = alloc.bump();
    let cid_id = alloc.bump();
    let descriptor_id = alloc.bump();
    let file_id = alloc.bump();
    let cmap_id = alloc.bump();

    let face = font.face();
    let upem = face.units_per_em() as f32;
    let to_pdf = |v: f32| v * 1000.0 / upem;
    let base_font = format!("{}+{}", subset_tag(glyphs.keys().chain(glyphs.values().map(|(gid, _)| gid))), font.base_font());

    let cff = program.cff;

    // Type0 (composite) font.
    pdf.type0_font(type0_id)
        .base_font(Name(base_font.as_bytes()))
        .encoding_predefined(Name(b"Identity-H"))
        .descendant_font(cid_id)
        .to_unicode(cmap_id);

    // Descendant CIDFont.
    let advance = |gid: u16| to_pdf(face.glyph_hor_advance(GlyphId(gid)).unwrap_or(0) as f32);
    let cid_advance = |cid: u16| advance(glyphs.get(&cid).map_or(0, |&(gid, _)| gid));
    let mut cid = pdf.cid_font(cid_id);
    cid.subtype(if cff { CidFontType::Type0 } else { CidFontType::Type2 });
    cid.base_font(Name(base_font.as_bytes()));
    cid.system_info(SystemInfo { registry: Str(b"Adobe"), ordering: Str(b"Identity"), supplement: 0 });
    cid.font_descriptor(descriptor_id);
    cid.default_width(advance(0));
    {
        let mut widths = cid.widths();
        // Group consecutive glyph IDs into runs.
        let gids: Vec<u16> = glyphs.keys().copied().collect();
        let mut i = 0;
        while i < gids.len() {
            let start = gids[i];
            let mut end = i + 1;
            while end < gids.len() && gids[end] == gids[end - 1] + 1 {
                end += 1;
            }
            widths.consecutive(start, gids[i..end].iter().map(|&c| cid_advance(c)));
            i = end;
        }
    }
    if !cff {
        cid.cid_to_gid_map_predefined(Name(b"Identity"));
    }
    cid.finish();

    // Font descriptor.
    let bbox = face.global_bounding_box();
    let ascent = to_pdf(face.ascender() as f32);
    let descent = to_pdf(face.descender() as f32);
    let cap_height = face
        .capital_height()
        .filter(|&h| h > 0)
        .map(|h| to_pdf(h as f32))
        .unwrap_or(ascent);
    let italic_angle = face.italic_angle();
    let weight = face.weight().to_number();
    // Common heuristic for StemV from the OS/2 weight class.
    let stem_v = 10.0 + 220.0 * (weight as f32 - 50.0) / 900.0;

    let mut flags = FontFlags::NON_SYMBOLIC;
    if face.is_monospaced() {
        flags |= FontFlags::FIXED_PITCH;
    }
    if face.is_italic() || italic_angle != 0.0 {
        flags |= FontFlags::ITALIC;
    }
    if weight >= 600 {
        flags |= FontFlags::FORCE_BOLD;
    }

    let mut descriptor = pdf.font_descriptor(descriptor_id);
    descriptor
        .name(Name(base_font.as_bytes()))
        .flags(flags)
        .bbox(Rect::new(
            to_pdf(bbox.x_min as f32),
            to_pdf(bbox.y_min as f32),
            to_pdf(bbox.x_max as f32),
            to_pdf(bbox.y_max as f32),
        ))
        .italic_angle(italic_angle)
        .ascent(ascent)
        .descent(descent)
        .cap_height(cap_height)
        .stem_v(stem_v);
    if cff {
        descriptor.font_file3(file_id);
    } else {
        descriptor.font_file2(file_id);
    }
    descriptor.finish();

    // Font program.
    let mut stream = pdf.stream(file_id, &program.data);
    stream.filter(Filter::FlateDecode);
    if cff {
        stream.pair(Name(b"Subtype"), Name(b"CIDFontType0C"));
    } else {
        stream.pair(Name(b"Length1"), program.raw_len as i32);
    }
    stream.finish();

    // ToUnicode CMap.
    let mut cmap = UnicodeCmap::new(
        Name(b"Custom"),
        SystemInfo { registry: Str(b"Adobe"), ordering: Str(b"UCS"), supplement: 0 },
    );
    for (&cid, (_, text)) in glyphs {
        if !text.is_empty() {
            cmap.pair_with_multiple(cid, text.chars());
        }
    }
    let cmap_data = crate::compress(&cmap.finish());
    pdf.cmap(cmap_id, &cmap_data).filter(Filter::FlateDecode);

    type0_id
}
