//! Font embedding: Type0/CIDFontType0|2 fonts with Identity-H encoding and ToUnicode CMaps.
//!
//! Fonts are subset to the glyphs actually used (via `subsetter`), which keeps both the file
//! size and the deflate time small. The subsetter renumbers glyphs consecutively and gives
//! CFF fonts an identity GID -> CID charset, so text is shown with 2-byte codes that are the
//! *remapped* glyph IDs: for TrueType `/CIDToGIDMap /Identity` maps CID == GID, and for CFF
//! the CIDs address the subset's glyphs directly.
//!
//! Besides the built-in fonts ([`FontId`]) the registry holds TrueType/OpenType fonts
//! downloaded by the label (~DU / ~DT / ~DY), registered per render with
//! [`FontRegistry::register_custom`] and parsed once per registry.

use std::collections::BTreeMap;
use std::sync::{Arc, OnceLock};

use pdf_writer::types::{CidFontType, FontFlags, SystemInfo, UnicodeCmap};
use pdf_writer::{Filter, Finish, Name, Pdf, Rect, Ref, Str};
use subsetter::GlyphRemapper;
use ttf_parser::{Face, GlyphId, RawFace, Tag};

/// Fonts built into the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontId {
    /// ZplSansCn-Bold.otf (CFF, TeX Gyre Heros Cn Bold with font 0 metrics) — ZPL scalable font 0 / text default.
    Triumvirate,
    /// DejaVuSansMono.ttf — barcode interpretation lines and the bitmap fonts A, C, D, F, G.
    DejaVuMono,
    /// DejaVuSans-Bold.ttf — per-character fallback for glyphs the other fonts lack
    /// (Greek, Cyrillic, symbols such as ™ ® €, ...).
    DejaVuSansBold,
    /// OCRB.otf (CFF) — bitmap font E.
    OcrB,
    /// OCRA.ttf — bitmap font H.
    OcrA,
    /// DejaVuSansMono-Bold.ttf — bitmap font B.
    DejaVuMonoBold,
}

/// Number of [`FontId`]s, for the per-font caches.
pub(crate) const BUILTIN_FONTS: usize = 6;


impl FontId {
    /// Raw font file bytes. The files are included by png-renderer, which renders with the
    /// same fonts, and read from its statics at run time: a `static` initialised from them
    /// would be const-evaluated and give this crate a second copy of every font.
    pub(crate) fn data(self) -> &'static [u8] {
        match self {
            FontId::Triumvirate => png_renderer::TRIUMVIRATE,
            FontId::DejaVuMono => png_renderer::DEJAVU_SANS_MONO,
            // pdf-renderer/fonts/DejaVuSans-Bold.ttf
            FontId::DejaVuSansBold => png_renderer::DEJAVU_SANS_BOLD,
            FontId::OcrB => png_renderer::OCR_B,
            FontId::OcrA => png_renderer::OCR_A,
            FontId::DejaVuMonoBold => png_renderer::DEJAVU_SANS_MONO_BOLD,
        }
    }

    /// Parsed font face (parsed once, cached for the process lifetime).
    pub(crate) fn face(self) -> &'static Face<'static> {
        static FACES: [OnceLock<Face<'static>>; BUILTIN_FONTS] = [const { OnceLock::new() }; BUILTIN_FONTS];
        FACES[self as usize].get_or_init(|| Face::parse(self.data(), 0).expect("embedded font must parse"))
    }

    /// Page resource name of this font.
    pub(crate) fn resource_name(self) -> Name<'static> {
        match self {
            FontId::Triumvirate => Name(b"F0"),
            FontId::DejaVuMono => Name(b"F1"),
            FontId::DejaVuSansBold => Name(b"F2"),
            FontId::OcrB => Name(b"F3"),
            FontId::OcrA => Name(b"F4"),
            FontId::DejaVuMonoBold => Name(b"F5"),
        }
    }

    /// PostScript name used as /BaseFont.
    fn base_font(self) -> &'static str {
        match self {
            FontId::Triumvirate => "ZplSansCn-Bold",
            FontId::DejaVuMono => "DejaVuSansMono",
            FontId::DejaVuSansBold => "DejaVuSans-Bold",
            FontId::OcrB => "OCRB",
            FontId::OcrA => "OCRA",
            FontId::DejaVuMonoBold => "DejaVuSansMono-Bold",
        }
    }

    /// Shaping face over the same bytes as [`FontId::data`] (built once, cached).
    pub(crate) fn shaper(self) -> &'static rustybuzz::Face<'static> {
        static SHAPERS: [OnceLock<rustybuzz::Face<'static>>; BUILTIN_FONTS] = [const { OnceLock::new() }; BUILTIN_FONTS];
        SHAPERS[self as usize].get_or_init(|| rustybuzz::Face::from_face(self.face().clone()))
    }

    /// Glyph ID for `ch` (0 / .notdef when the font has no glyph for it).
    pub(crate) fn glyph(self, ch: char) -> u16 {
        self.face().glyph_index(ch).map(|g| g.0).unwrap_or(0)
    }
}

/// A font the registry can show text in: built in, or downloaded by the label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum FontKey {
    Builtin(FontId),
    /// Index into [`FontRegistry::custom`].
    Custom(u16),
}

impl From<FontId> for FontKey {
    fn from(id: FontId) -> Self {
        FontKey::Builtin(id)
    }
}

/// Page resource name (`F0`.. for built-ins, `C0`.. for downloaded fonts), stored inline so
/// it can be held while the registry is borrowed mutably.
#[derive(Clone, Copy)]
pub(crate) struct ResName {
    buf: [u8; 8],
    len: u8,
}

impl ResName {
    pub(crate) fn of(key: FontKey) -> Self {
        let mut buf = [0u8; 8];
        let len = match key {
            FontKey::Builtin(id) => {
                let n = id.resource_name().0;
                buf[..n.len()].copy_from_slice(n);
                n.len()
            }
            FontKey::Custom(i) => {
                use std::io::Write;
                let mut cur = std::io::Cursor::new(&mut buf[..]);
                let _ = write!(cur, "C{i}");
                cur.position() as usize
            }
        };
        ResName { buf, len: len as u8 }
    }

    pub(crate) fn name(&self) -> Name<'_> {
        Name(&self.buf[..self.len as usize])
    }
}

/// A downloaded font. `face` borrows `data`'s heap buffer, so it is declared first and
/// therefore dropped first; the buffer is never mutated (it sits behind an `Arc`).
struct CustomFont {
    /// Shaping face, built on first use (^PA shaping); borrows `data` like `face`.
    shaper: OnceLock<Option<rustybuzz::Face<'static>>>,
    face: Option<Face<'static>>,
    data: Arc<Vec<u8>>,
    /// PostScript name for /BaseFont.
    base_font: String,
}

impl CustomFont {
    fn new(data: &Arc<Vec<u8>>, index: usize) -> Self {
        let data = Arc::clone(data);
        // SAFETY: the bytes live on the heap owned by `data`, which this struct keeps alive
        // and never mutates; `face` is dropped before `data` (field order), and the face is
        // only handed out with lifetimes bounded by a borrow of the registry.
        let bytes: &'static [u8] = unsafe { std::slice::from_raw_parts(data.as_ptr(), data.len()) };
        let face = Face::parse(bytes, 0).ok().filter(|face| usable(bytes, face));
        let base_font = face
            .as_ref()
            .and_then(postscript_name)
            .unwrap_or_else(|| format!("ZplCustom{index}"));
        CustomFont { shaper: OnceLock::new(), face, data, base_font }
    }
}

/// Whether `face` has outlines the subsetter can embed. TrueType subsetting failures are
/// survivable (the whole font is embedded instead, see [`FontProgram::build`]); CFF ones are
/// not, so CFF fonts get a trial subset up front. CFF2 / bitmap-only fonts are rejected.
fn usable(data: &[u8], face: &Face) -> bool {
    let raw = face.raw_face();
    if raw.table(Tag::from_bytes(b"glyf")).is_some() {
        return face.number_of_glyphs() > 0;
    }
    if raw.table(Tag::from_bytes(b"CFF ")).is_some() {
        let mut remapper = GlyphRemapper::new();
        for ch in ['A', '0', 'a'] {
            if let Some(g) = face.glyph_index(ch) {
                remapper.remap(g.0);
            }
        }
        return subsetter::subset(data, 0, &remapper).is_ok();
    }
    false
}

/// PostScript name from the `name` table, restricted to characters valid in a PDF name.
fn postscript_name(face: &Face) -> Option<String> {
    let name = face
        .names()
        .into_iter()
        .filter(|n| n.name_id == ttf_parser::name_id::POST_SCRIPT_NAME)
        // Non-Unicode (e.g. Mac Roman) records: keep the bytes; non-ASCII is filtered below.
        .map(|n| n.to_string().unwrap_or_else(|| n.name.iter().map(|&b| b as char).collect()))
        .next()?;
    let clean: String = name.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').take(60).collect();
    (!clean.is_empty()).then_some(clean)
}

#[derive(Default)]
pub(crate) struct FontRegistry {
    used: BTreeMap<FontKey, UsedGlyphs>,
    /// Downloaded fonts seen in this render (parsed or not), in registration order.
    custom: Vec<CustomFont>,
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

impl UsedGlyphs {
    /// Records `gid` (showing `ch`) and returns its subset ID.
    fn add(&mut self, ch: char, gid: u16) -> u16 {
        let cid = self.remapper.remap(gid);
        self.glyphs.entry(cid).or_insert_with(|| (gid, if gid == 0 { String::new() } else { ch.to_string() }));
        cid
    }
}

impl FontRegistry {
    /// Registers a downloaded font. Returns `None` when it can't be parsed or embedded, in
    /// which case callers fall back to a built-in font. The same `Arc` (or identical bytes)
    /// yields the same key, so a font is parsed at most once per render.
    pub fn register_custom(&mut self, data: &Arc<Vec<u8>>) -> Option<FontKey> {
        let index = match self
            .custom
            .iter()
            .position(|c| Arc::ptr_eq(&c.data, data) || (c.data.len() == data.len() && c.data[..] == data[..]))
        {
            Some(i) => i,
            None => {
                let index = self.custom.len();
                if index > u16::MAX as usize {
                    return None;
                }
                self.custom.push(CustomFont::new(data, index));
                index
            }
        };
        self.custom[index].face.as_ref().map(|_| FontKey::Custom(index as u16))
    }

    /// Parsed face of `key`. Custom keys only come from [`FontRegistry::register_custom`],
    /// which hands them out for parsed fonts only.
    pub fn face(&self, key: FontKey) -> &Face<'_> {
        match key {
            FontKey::Builtin(id) => id.face(),
            FontKey::Custom(i) => self.custom[i as usize].face.as_ref().expect("custom font key without a face"),
        }
    }

    /// Shaping face of `key`, over the same font bytes that get embedded.
    pub fn shaper(&self, key: FontKey) -> &rustybuzz::Face<'_> {
        match key {
            FontKey::Builtin(id) => id.shaper(),
            FontKey::Custom(i) => {
                let custom = &self.custom[i as usize];
                custom
                    .shaper
                    .get_or_init(|| custom.face.clone().map(rustybuzz::Face::from_face))
                    .as_ref()
                    .expect("custom font key without a face")
            }
        }
    }

    fn data(&self, key: FontKey) -> &[u8] {
        match key {
            FontKey::Builtin(id) => id.data(),
            FontKey::Custom(i) => &self.custom[i as usize].data,
        }
    }

    fn base_font(&self, key: FontKey) -> &str {
        match key {
            FontKey::Builtin(id) => id.base_font(),
            FontKey::Custom(i) => &self.custom[i as usize].base_font,
        }
    }

    /// Records the glyphs needed to show `text` in `font`. Returns the font's resource
    /// name and the subset glyph IDs (2-byte CIDs) in display order.
    #[allow(dead_code)]
    pub fn use_glyphs(&mut self, font: FontId, text: &str) -> (Name<'static>, Vec<u16>) {
        let used = self.used.entry(font.into()).or_default();
        let gids = text.chars().map(|ch| used.add(ch, font.glyph(ch))).collect();
        (font.resource_name(), gids)
    }

    /// Records the `(char, original GID)` pairs shown in `key` and appends their 2-byte
    /// subset codes to `out`. Returns the font's resource name.
    pub fn use_glyph_ids(&mut self, key: FontKey, glyphs: impl Iterator<Item = (char, u16)>, out: &mut Vec<u8>) -> ResName {
        let used = self.used.entry(key).or_default();
        for (ch, gid) in glyphs {
            out.extend_from_slice(&used.add(ch, gid).to_be_bytes());
        }
        ResName::of(key)
    }

    /// Records one (possibly shaped) glyph of `key` standing for `text` (the source
    /// characters of its cluster, for ToUnicode; empty for none) and returns its 2-byte code.
    /// Glyph IDs need not be reachable through the cmap (contextual forms, ligatures).
    pub fn use_glyph(&mut self, key: FontKey, gid: u16, text: &str) -> u16 {
        let used = self.used.entry(key).or_default();
        let cid = used.remapper.remap(gid);
        used.glyphs.entry(cid).or_insert_with(|| (gid, if gid == 0 { String::new() } else { text.to_owned() }));
        cid
    }

    /// Records `text` shown entirely in `key` and appends its 2-byte subset codes to
    /// `out`. Returns `None` (leaving `out` truncated to its old length) as soon as `key`
    /// lacks a glyph, so the caller can lay the text out with fallback fonts instead.
    pub fn use_text(&mut self, key: FontKey, text: &str, out: &mut Vec<u8>) -> Option<ResName> {
        let face = match key {
            FontKey::Builtin(id) => id.face(),
            FontKey::Custom(i) => self.custom[i as usize].face.as_ref()?,
        };
        let used = self.used.entry(key).or_default();
        let start = out.len();
        for ch in text.chars() {
            let Some(gid) = face.glyph_index(ch) else {
                out.truncate(start);
                return None;
            };
            out.extend_from_slice(&used.add(ch, gid.0).to_be_bytes());
        }
        Some(ResName::of(key))
    }

    /// Subsets and deflates the program of every used font (the bulk of the document's
    /// cost) while `main` runs. With `parallel`, each font gets its own scoped thread;
    /// otherwise everything runs on the calling thread. Programs are in `write` order.
    pub fn build_programs<R: Send>(&self, parallel: bool, main: impl FnOnce() -> R) -> (Vec<FontProgram>, R) {
        if !parallel {
            let result = main();
            let programs = self.used.iter().map(|(&key, used)| FontProgram::build(self, key, used)).collect();
            return (programs, result);
        }
        std::thread::scope(|scope| {
            let handles: Vec<_> = self
                .used
                .iter()
                .map(|(&key, used)| scope.spawn(move || FontProgram::build(self, key, used)))
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
            .map(|((&key, used), program)| {
                let id = write_font(pdf, alloc, self.face(key), self.base_font(key), used, program);
                (String::from_utf8_lossy(ResName::of(key).name().0).into_owned(), id)
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
    /// Deflated CIDToGIDMap stream when the whole (unsubset) TrueType font is embedded
    /// because subsetting failed; `None` means `/Identity`.
    cid_to_gid: Option<Vec<u8>>,
}

impl FontProgram {
    fn build(registry: &FontRegistry, key: FontKey, used: &UsedGlyphs) -> Self {
        let data = registry.data(key);
        // CFF outlines: embed the bare (now CID-keyed) CFF table.
        let is_cff = registry.face(key).raw_face().table(Tag::from_bytes(b"CFF ")).is_some();
        let subset = match subsetter::subset(data, 0, &used.remapper) {
            Ok(subset) => subset,
            Err(_) if !is_cff => {
                // Only reachable for downloaded TrueType fonts: embed them whole and map
                // the subset IDs used in the content stream back to the original glyphs.
                let max = used.glyphs.keys().next_back().copied().unwrap_or(0) as usize;
                let mut map = vec![0u8; (max + 1) * 2];
                for (&cid, &(gid, _)) in &used.glyphs {
                    map[cid as usize * 2..cid as usize * 2 + 2].copy_from_slice(&gid.to_be_bytes());
                }
                return FontProgram {
                    data: crate::compress(data),
                    cff: false,
                    raw_len: data.len(),
                    cid_to_gid: Some(crate::compress(&map)),
                };
            }
            Err(e) => panic!("font must subset (custom CFF fonts are trial-subset on registration): {e:?}"),
        };
        let program = if is_cff {
            let raw = RawFace::parse(&subset, 0).expect("subset font must parse");
            raw.table(Tag::from_bytes(b"CFF ")).expect("subset keeps CFF table")
        } else {
            &subset
        };
        FontProgram { data: crate::compress(program), cff: is_cff, raw_len: program.len(), cid_to_gid: None }
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

fn write_font(pdf: &mut Pdf, alloc: &mut Ref, face: &Face, name: &str, used: &UsedGlyphs, program: FontProgram) -> Ref {
    let glyphs = &used.glyphs;
    let type0_id = alloc.bump();
    let cid_id = alloc.bump();
    let descriptor_id = alloc.bump();
    let file_id = alloc.bump();
    let cmap_id = alloc.bump();
    let map_id = program.cid_to_gid.as_ref().map(|_| alloc.bump());

    let upem = face.units_per_em() as f32;
    let to_pdf = |v: f32| v * 1000.0 / upem;
    let base_font = format!("{}+{}", subset_tag(glyphs.keys().chain(glyphs.values().map(|(gid, _)| gid))), name);

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
        match map_id {
            Some(id) => {
                cid.cid_to_gid_map_stream(id);
            }
            None => {
                cid.cid_to_gid_map_predefined(Name(b"Identity"));
            }
        }
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

    if let (Some(id), Some(map)) = (map_id, &program.cid_to_gid) {
        pdf.stream(id, map).filter(Filter::FlateDecode);
    }

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
