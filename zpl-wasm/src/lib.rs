//! WebAssembly bindings for rendering ZPL in the browser (the `/playground` editor).
//!
//! - `render_rgba(zpl, clock)` renders a label to RGBA pixels for a `<canvas>`.
//! - `render_pdf(zpl, clock)` renders a label to PDF file bytes.
//!
//! `clock` is the printer clock for ^FC fields, in seconds since the Unix epoch in local
//! time; the browser has no system clock reachable from `wasm32-unknown-unknown`.
//! A parse error is thrown as a `ParseFailure`, with its position in UTF-16 code units so
//! it indexes straight into a JavaScript string.

use pdf_renderer::PdfRenderer;
use png_renderer::PngRenderer;
use wasm_bindgen::prelude::*;
use zpl_parser::error::ParseError;
use zpl_parser::Parser;
use zpl_state_manager::{DrawInstruction, PageSetup, StateManager};

/// A rendered label as RGBA pixels, row by row.
#[wasm_bindgen]
pub struct Frame {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Frame {
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
        Self {
            message: error.to_string(),
            start: span.map(|s| utf16_offset(zpl, s.start)),
            end: span.map(|s| utf16_offset(zpl, s.end)),
        }
    }
}

/// Converts a byte offset into `text` to a UTF-16 offset, clamping out-of-range offsets
/// and rounding ones inside a character down to its start.
fn utf16_offset(text: &str, byte: usize) -> u32 {
    let mut byte = byte.min(text.len());
    while !text.is_char_boundary(byte) {
        byte -= 1;
    }
    text[..byte].encode_utf16().count() as u32
}

fn layout(zpl: &str, clock: f64) -> Result<(Vec<DrawInstruction>, PageSetup), ParseFailure> {
    let commands = Parser::new(zpl).parse().map_err(|e| ParseFailure::new(zpl, e))?;
    let mut state = StateManager::new();
    state.set_clock_unix(clock as i64);
    let instructions = state.process(commands);
    Ok((instructions, state.page_setup()))
}

/// Renders `zpl` to RGBA pixels. `debug` outlines every field.
#[wasm_bindgen(js_name = renderRgba)]
pub fn render_rgba(zpl: &str, clock: f64, debug: bool) -> Result<Frame, ParseFailure> {
    let (instructions, page) = layout(zpl, clock)?;
    let image = PngRenderer::with_debug(page.width, page.height, debug).render(&instructions);
    let (width, height) = image.dimensions();
    let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
    for rgb in image.as_raw().chunks_exact(3) {
        pixels.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
    }
    Ok(Frame { width, height, pixels })
}

/// Renders `zpl` to a PDF file.
#[wasm_bindgen(js_name = renderPdf)]
pub fn render_pdf(zpl: &str, clock: f64) -> Result<Vec<u8>, ParseFailure> {
    let (instructions, page) = layout(zpl, clock)?;
    Ok(PdfRenderer::for_page(&page).render(&instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_offsets() {
        assert_eq!(utf16_offset("^FDé^FS", 5), 4);
        assert_eq!(utf16_offset("^FD😀^FS", 7), 5);
        assert_eq!(utf16_offset("é", 1), 0);
        assert_eq!(utf16_offset("abc", 99), 3);
    }

    #[test]
    fn renders_rgba() {
        let frame = render_rgba("^XA^PW100^LL50^FO10,10^A0N,20,20^FDhi^FS^XZ", 0.0, false).ok().unwrap();
        assert_eq!((frame.width, frame.height), (100, 50));
        assert_eq!(frame.pixels.len(), 100 * 50 * 4);
        assert!(frame.pixels.chunks_exact(4).any(|p| p[0] == 0));
    }

    #[test]
    fn renders_pdf() {
        assert!(render_pdf("^XA^FO10,10^A0N,20,20^FDhi^FS^XZ", 0.0).ok().unwrap().starts_with(b"%PDF"));
    }
}
