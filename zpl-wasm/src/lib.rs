//! WebAssembly bindings for rendering ZPL in the browser (the `/playground` editor).
//!
//! - `renderRgba(zpl, clock, debug, page, dpmm)` renders page `page` (0-based) to RGBA
//!   pixels for a `<canvas>`, plus its elements: where each one is drawn and which source
//!   text produced it. Every label (^XA...^XZ, times its ^PQ copies) is a page.
//! - `renderPdf(zpl, clock, dpmm)` renders all pages to PDF file bytes.
//!
//! `dpmm` is the printer resolution in dots per millimetre: 6, 8, 12 or 24 (anything else
//! means 8). Pixels and element positions are printer dots.
//! `clock` is the printer clock for ^FC fields, in seconds since the Unix epoch in local
//! time; the browser has no system clock reachable from `wasm32-unknown-unknown`.
//! Source positions are in UTF-16 code units, so they index straight into a JavaScript
//! string. A parse error is thrown as a `ParseFailure`.

use pdf_renderer::PdfRenderer;
use png_renderer::PngRenderer;
use wasm_bindgen::prelude::*;
use zpl_parser::error::ParseError;
use zpl_parser::Parser;
use zpl_state_manager::{Page, StateManager, DEFAULT_DPMM, DPMM_VALUES};

/// Values per element in [`Frame::elements`].
const ELEMENT_STRIDE: usize = 8;
/// Marks a missing source position in [`Frame::elements`].
const NONE: u32 = u32::MAX;

/// A rendered page as RGBA pixels, row by row, and the elements drawn on it.
#[wasm_bindgen]
pub struct Frame {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    elements: Vec<u32>,
    page: u32,
    page_count: u32,
}

#[wasm_bindgen]
impl Frame {
    /// The page shown (0-based): the one asked for, or the last page if there are fewer.
    #[wasm_bindgen(getter)]
    pub fn page(&self) -> u32 {
        self.page
    }

    /// Pages in the input; always at least 1.
    #[wasm_bindgen(getter, js_name = pageCount)]
    pub fn page_count(&self) -> u32 {
        self.page_count
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Moves the pixels out (as a `Uint8Array`); later calls return an empty array.
    #[wasm_bindgen(js_name = takePixels)]
    pub fn take_pixels(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.pixels)
    }

    /// Moves the elements out (as a `Uint32Array`), in drawing order, 8 values each:
    /// `x, y, width, height` in dots, then `fieldStart, fieldEnd` (the field's source,
    /// ^FO through ^FS) and `dataStart, dataEnd` (the text of its ^FD, `0xFFFFFFFF` if none).
    #[wasm_bindgen(js_name = takeElements)]
    pub fn take_elements(&mut self) -> Vec<u32> {
        std::mem::take(&mut self.elements)
    }
}

#[wasm_bindgen]
pub struct ParseFailure {
    message: String,
    start: Option<u32>,
    end: Option<u32>,
}

#[wasm_bindgen]
impl ParseFailure {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// Start of the offending text in UTF-16 code units, if the error has a position.
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Option<u32> {
        self.start
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Option<u32> {
        self.end
    }
}

impl ParseFailure {
    fn new(zpl: &str, error: ParseError) -> Self {
        let span = match &error {
            ParseError::MissingRequiredParameter { span, .. }
            | ParseError::InvalidNumber { span, .. }
            | ParseError::InvalidJustification { span, .. }
            | ParseError::InvalidYesNo { span, .. }
            | ParseError::InvalidBlockJustification { span, .. }
            | ParseError::InvalidLineColor { span, .. }
            | ParseError::InvalidCompressionType { span, .. }
            | ParseError::InvalidCode128Mode { span, .. } => Some(span),
            ParseError::InvalidOrientation | ParseError::InvalidZ64Data { .. } => None,
        };
        let offsets = Utf16Offsets::new(zpl);
        Self {
            message: error.to_string(),
            start: span.map(|s| offsets.of(s.start)),
            end: span.map(|s| offsets.of(s.end)),
        }
    }
}

/// Converts byte offsets into a string to UTF-16 offsets. Keeps one entry per non-ASCII
/// character, so ASCII labels (nearly all of them, ^GF data included) cost nothing.
struct Utf16Offsets<'a> {
    text: &'a str,
    /// (byte offset just past a non-ASCII char, UTF-8 bytes minus UTF-16 units so far).
    shrink: Vec<(usize, usize)>,
}

impl<'a> Utf16Offsets<'a> {
    fn new(text: &'a str) -> Self {
        let mut shrink = Vec::new();
        let mut total = 0;
        for (i, c) in text.char_indices().filter(|(_, c)| !c.is_ascii()) {
            total += c.len_utf8() - c.len_utf16();
            shrink.push((i + c.len_utf8(), total));
        }
        Self { text, shrink }
    }

    /// Clamps out-of-range offsets and rounds ones inside a character down to its start.
    fn of(&self, byte: usize) -> u32 {
        let mut byte = byte.min(self.text.len());
        while !self.text.is_char_boundary(byte) {
            byte -= 1;
        }
        let before = self.shrink.partition_point(|&(end, _)| end <= byte);
        let shrink = if before == 0 { 0 } else { self.shrink[before - 1].1 };
        (byte - shrink) as u32
    }
}

/// Start of the text in a ^FD/^FV command: past its prefix (^, or whatever ^CC set) and
/// the two-letter name.
fn field_text_start(zpl: &str, data: zpl_parser::Span) -> usize {
    let prefix = zpl[data.start..].chars().next().map_or(0, char::len_utf8);
    (data.start + prefix + 2).min(data.end)
}

fn state(clock: f64, dpmm: u32) -> StateManager {
    let mut state = StateManager::new();
    state.set_clock_unix(clock as i64);
    state.set_dpmm(dpmm);
    state
}

/// `dpmm` if Zebra makes printheads of that resolution, else the default 8 dots/mm.
fn resolution(dpmm: u32) -> u32 {
    if DPMM_VALUES.contains(&dpmm) { dpmm } else { DEFAULT_DPMM }
}

/// Renders page `page` (0-based, clamped to the last) of `zpl` to RGBA pixels and its
/// element map. `debug` outlines every field.
#[wasm_bindgen(js_name = renderRgba)]
pub fn render_rgba(zpl: &str, clock: f64, debug: bool, page: u32, dpmm: u32) -> Result<Frame, ParseFailure> {
    let dpmm = resolution(dpmm);
    let (commands, spans) = Parser::with_dpmm(zpl, dpmm).parse_with_spans().map_err(|e| ParseFailure::new(zpl, e))?;
    let mut pages = state(clock, dpmm).process_pages_mapped(commands, &spans);
    let page_count = pages.len();
    let index = (page as usize).min(page_count - 1);
    let (Page { instructions, setup }, sources) = pages.swap_remove(index);
    let renderer = PngRenderer::for_page(&setup, debug);
    let (image, bounds) = renderer.render_with_bounds(&instructions);

    let offsets = Utf16Offsets::new(zpl);
    let mut elements = Vec::with_capacity(bounds.len() * ELEMENT_STRIDE);
    for (bounds, source) in bounds.iter().zip(&sources) {
        let Some(b) = bounds else { continue };
        let (data_start, data_end) =
            source.data.map_or((NONE, NONE), |d| (offsets.of(field_text_start(zpl, d)), offsets.of(d.end)));
        elements.extend_from_slice(&[
            b.x,
            b.y,
            b.width,
            b.height,
            offsets.of(source.field.start),
            offsets.of(source.field.end),
            data_start,
            data_end,
        ]);
    }

    let (width, height) = image.dimensions();
    let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
    for rgb in image.as_raw().chunks_exact(3) {
        pixels.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
    }
    Ok(Frame { width, height, pixels, elements, page: index as u32, page_count: page_count as u32 })
}

/// Renders `zpl` to a PDF file with a page per label.
#[wasm_bindgen(js_name = renderPdf)]
pub fn render_pdf(zpl: &str, clock: f64, dpmm: u32) -> Result<Vec<u8>, ParseFailure> {
    let dpmm = resolution(dpmm);
    let commands = Parser::with_dpmm(zpl, dpmm).parse().map_err(|e| ParseFailure::new(zpl, e))?;
    let pages = state(clock, dpmm).process_pages(commands);
    let pages: Vec<_> = pages.iter().map(|p| (p.setup, p.instructions.as_slice())).collect();
    Ok(PdfRenderer::for_page(&pages[0].0).render_pages(&pages))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_offsets() {
        let offsets = Utf16Offsets::new("^FDé^FS");
        assert_eq!(offsets.of(5), 4);
        assert_eq!(offsets.of(4), 3);
        let offsets = Utf16Offsets::new("^FD😀^FS");
        assert_eq!(offsets.of(7), 5);
        assert_eq!(offsets.of(9), 7);
        assert_eq!(Utf16Offsets::new("é").of(1), 0);
        assert_eq!(Utf16Offsets::new("abc").of(99), 3);
    }

    #[test]
    fn renders_rgba() {
        let frame = render_rgba("^XA^PW100^LL50^FO10,10^A0N,20,20^FDhi^FS^XZ", 0.0, false, 0, 8).ok().unwrap();
        assert_eq!((frame.width, frame.height), (100, 50));
        assert_eq!(frame.pixels.len(), 100 * 50 * 4);
        assert!(frame.pixels.chunks_exact(4).any(|p| p[0] == 0));
    }

    #[test]
    fn maps_elements_to_source() {
        let zpl = "^XA^PW300^LL200\n^FO10,10^GB50,40,2^FS\n^FO100,20^A0N,30,30^FDé ok^FS\n^XZ";
        let frame = render_rgba(zpl, 0.0, false, 0, 8).ok().unwrap();
        let e = &frame.elements;
        assert_eq!(e.len(), 2 * ELEMENT_STRIDE);
        assert_eq!(&e[..ELEMENT_STRIDE], &[10, 10, 50, 40, 16, 37, NONE, NONE]);

        let text: Vec<u16> = zpl.encode_utf16().collect();
        let slice = |a: u32, b: u32| String::from_utf16(&text[a as usize..b as usize]).unwrap();
        let text_element = &e[ELEMENT_STRIDE..];
        assert_eq!(slice(text_element[4], text_element[5]), "^FO100,20^A0N,30,30^FDé ok^FS");
        assert_eq!(slice(text_element[6], text_element[7]), "é ok");
    }

    #[test]
    fn data_range_is_the_field_text() {
        let text = |zpl: &str| {
            let e = render_rgba(zpl, 0.0, false, 0, 8).ok().unwrap().elements;
            zpl.get(e[6] as usize..e[7] as usize).map(str::to_string)
        };
        assert_eq!(text("^XA^FO10,10^A0N,30,30^FDabc^FS^XZ").as_deref(), Some("abc"));
        assert_eq!(text("^XA^CC~~FO10,10~A0N,30,30~FDabc~FS~XZ").as_deref(), Some("abc"));
        // An empty ^FD draws nothing, so there is no element to click.
        assert!(render_rgba("^XA^FO10,10^FD^FS^XZ", 0.0, false, 0, 8).ok().unwrap().elements.is_empty());
    }

    #[test]
    fn renders_pages() {
        let zpl = "^XA^PW100^LL50^FO1,1^GB5,5,5^FS^XZ\n^XA^PQ2^PW200^FO10,10^SN1^FS^FO1,1^GB5,5,5^FS^XZ";
        let frame = render_rgba(zpl, 0.0, false, 0, 8).ok().unwrap();
        assert_eq!((frame.page, frame.page_count, frame.width), (0, 3, 100));
        assert_eq!(frame.elements.len(), ELEMENT_STRIDE);

        let frame = render_rgba(zpl, 0.0, false, 2, 8).ok().unwrap();
        assert_eq!((frame.page, frame.width, frame.elements.len()), (2, 200, 2 * ELEMENT_STRIDE));
        // Copies map to the source of the label they repeat.
        assert_eq!(&zpl[frame.elements[4] as usize..frame.elements[5] as usize], "^FO10,10^SN1^FS");

        // Past the end shows the last page.
        assert_eq!(render_rgba(zpl, 0.0, false, 9, 8).ok().unwrap().page, 2);

        let pdf = render_pdf(zpl, 0.0, 8).ok().unwrap();
        assert!(String::from_utf8_lossy(&pdf).contains("/Count 3"));
    }

    #[test]
    fn dpmm_sizes_the_default_label() {
        let size = |dpmm| {
            let frame = render_rgba("^XA^FO10,10^GB20,20,20^FS^XZ", 0.0, false, 0, dpmm).ok().unwrap();
            (frame.width, frame.height)
        };
        assert_eq!(size(6), (608, 912));
        assert_eq!(size(12), (1200, 1800));
        assert_eq!(size(400), (812, 1218));
        let pdf = render_pdf("^XA^FO10,10^GB20,20,20^FS^XZ", 0.0, 24).ok().unwrap();
        assert!(String::from_utf8_lossy(&pdf).contains("/MediaBox [0 0 288 432]"));
    }

    #[test]
    fn half_density_scales_elements() {
        let frame = render_rgba("^XA^JMB^PW100^LL50^FO10,10^GB20,10,10^FS^XZ", 0.0, false, 0, 8).ok().unwrap();
        assert_eq!((frame.width, frame.height), (200, 100));
        assert_eq!(&frame.elements[..4], &[20, 20, 40, 20]);
    }

    #[test]
    fn renders_pdf() {
        assert!(render_pdf("^XA^FO10,10^A0N,20,20^FDhi^FS^XZ", 0.0, 8).ok().unwrap().starts_with(b"%PDF"));
    }
}
