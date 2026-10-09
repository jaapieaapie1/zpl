//! Vector PDF renderer for ZPL draw instructions.
//!
//! Every element is emitted as native PDF content: text as embedded-font text objects,
//! boxes/lines/circles and barcode bars/QR modules as filled paths, and ^GF graphics as
//! 1-bit image masks. No raster of the whole label is ever produced.
//!
//! Coordinate system: the page content stream starts with a CTM that maps printer dots
//! (origin top-left, y down) to PDF points. All modules therefore draw in dot units.
//! Anything that renders glyphs must flip y back locally (see `text.rs`).

use pdf_writer::{Content, Filter, Finish, Name, Pdf, Rect, Ref};
use zpl_state_manager::{DrawInstruction, PageSetup};

mod barcodes;
mod fonts;
mod graphic;
mod matrix;
mod shapes;
mod text;

pub use fonts::FontId;

/// Default Zebra resolution (8 dots/mm).
pub const DEFAULT_DPI: f32 = 203.0;

pub struct PdfRenderer {
    width: u32,
    height: u32,
    dpi: f32,
    parallel: bool,
    inverted: bool,
    mirrored: bool,
}

/// Rendering state shared by all element modules.
pub(crate) struct Ctx {
    /// Page content stream, in dot coordinates (y down).
    pub content: Content,
    /// Fonts referenced so far; written as resources when the document is finished.
    pub fonts: fonts::FontRegistry,
    /// ^GF bitmaps; each becomes an /ImageMask XObject named `Im{index}`.
    pub images: Vec<graphic::ImageMask>,
    /// Set when a reverse (^FR) element used the `/GsDiff` difference blend state.
    pub uses_difference: bool,
    /// Gray level that prints black: 0 normally, 1 inside a reverse (^FR) element, where
    /// white under the Difference blend mode inverts what is underneath.
    pub ink: f32,
}

impl Ctx {
    pub fn new() -> Self {
        Self { content: Content::new(), fonts: Default::default(), images: Vec::new(), uses_difference: false, ink: 0.0 }
    }

    /// Fill gray for a ZPL line color, honouring reverse printing.
    pub fn gray(&self, color: zpl_parser::command::LineColor) -> f32 {
        match color {
            zpl_parser::command::LineColor::Black => self.ink,
            zpl_parser::command::LineColor::White => 1.0 - self.ink,
        }
    }
}

impl PdfRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_dpi(width, height, DEFAULT_DPI)
    }

    pub fn with_dpi(width: u32, height: u32, dpi: f32) -> Self {
        Self { width, height, dpi, parallel: false, inverted: false, mirrored: false }
    }

    /// Renderer for a label's page settings: size plus ^PO inversion and ^PM mirroring.
    pub fn for_page(page: &PageSetup) -> Self {
        Self { inverted: page.inverted, mirrored: page.mirrored, ..Self::new(page.width, page.height) }
    }

    /// Subsets each font on its own short-lived thread while the page is compressed.
    ///
    /// Off by default: thread start-up (and a fresh per-thread compressor) costs more than
    /// it saves on Linux, measured ~25-80% slower per render, and far worse on small cloud
    /// VMs. It only shaved ~20 µs off font-heavy labels on an idle macOS machine.
    pub fn parallel_fonts(mut self) -> Self {
        self.parallel = true;
        self
    }

    pub fn render(&self, instructions: &[DrawInstruction]) -> Vec<u8> {
        let scale = 72.0 / self.dpi;
        let page_w = self.width as f32 * scale;
        let page_h = self.height as f32 * scale;

        let mut ctx = Ctx::new();

        // Map dots (top-left origin, y down) to points (bottom-left origin, y up).
        ctx.content.transform([scale, 0.0, 0.0, -scale, 0.0, page_h]);
        let (w, h) = (self.width as f32, self.height as f32);
        // ^PM mirrors left-right, ^POI turns the label upside down; both flips cancel x.
        match (self.inverted, self.mirrored) {
            (false, false) => {}
            (false, true) => {
                ctx.content.transform([-1.0, 0.0, 0.0, 1.0, w, 0.0]);
            }
            (true, false) => {
                ctx.content.transform([-1.0, 0.0, 0.0, -1.0, w, h]);
            }
            (true, true) => {
                ctx.content.transform([1.0, 0.0, 0.0, -1.0, 0.0, h]);
            }
        }

        // Opaque white label stock, so ^FR (Difference blend) inverts against white rather than
        // the transparent page backdrop, where it would just paint white.
        ctx.content
            .set_fill_gray(1.0)
            .rect(0.0, 0.0, self.width as f32, self.height as f32)
            .fill_nonzero()
            .set_fill_gray(0.0);

        for instruction in instructions {
            render_instruction(&mut ctx, instruction);
        }

        self.write_document(ctx, page_w, page_h)
    }

    fn write_document(&self, ctx: Ctx, page_w: f32, page_h: f32) -> Vec<u8> {
        let mut alloc = Ref::new(1);
        let catalog_id = alloc.bump();
        let pages_id = alloc.bump();
        let page_id = alloc.bump();
        let content_id = alloc.bump();
        let gs_diff_id = alloc.bump();

        let mut pdf = Pdf::new();
        pdf.catalog(catalog_id).pages(pages_id);
        pdf.pages(pages_id).kids([page_id]).count(1);

        // With `parallel_fonts`, font subsetting runs on worker threads while this thread
        // deflates the rest.
        let raw = ctx.content.finish();
        let (programs, (content, images)) = ctx.fonts.build_programs(self.parallel, || {
            let images: Vec<Vec<u8>> = ctx.images.iter().map(|img| compress(&img.data)).collect();
            (compress(&raw), images)
        });

        let font_refs = ctx.fonts.write(&mut pdf, &mut alloc, programs);
        let image_refs: Vec<Ref> =
            ctx.images.iter().zip(&images).map(|(img, data)| img.write(&mut pdf, &mut alloc, data)).collect();

        if ctx.uses_difference {
            pdf.ext_graphics(gs_diff_id).blend_mode(pdf_writer::types::BlendMode::Difference);
        }

        let mut page = pdf.page(page_id);
        page.media_box(Rect::new(0.0, 0.0, page_w, page_h));
        page.parent(pages_id);
        page.contents(content_id);
        let mut resources = page.resources();
        {
            let mut fonts = resources.fonts();
            for (name, id) in &font_refs {
                fonts.pair(Name(name.as_bytes()), *id);
            }
        }
        {
            let mut xobjects = resources.x_objects();
            for (i, id) in image_refs.iter().enumerate() {
                xobjects.pair(Name(format!("Im{i}").as_bytes()), *id);
            }
        }
        if ctx.uses_difference {
            resources.ext_g_states().pair(Name(b"GsDiff"), gs_diff_id);
        }
        resources.finish();
        page.finish();

        pdf.stream(content_id, &content).filter(Filter::FlateDecode);

        pdf.finish()
    }
}

/// zlib-compresses `data` at level 1: a few percent larger than level 6 but roughly twice
/// as fast. The compressor (~300 KB of hash tables) is reused across calls, since
/// allocating and zeroing a fresh one dominated the cost for the small streams of a label.
pub(crate) fn compress(data: &[u8]) -> Vec<u8> {
    use miniz_oxide::deflate::core::{compress, CompressorOxide, TDEFLFlush, TDEFLStatus};
    use miniz_oxide::DataFormat;

    thread_local! {
        static COMPRESSOR: std::cell::RefCell<Box<CompressorOxide>> =
            std::cell::RefCell::new(Box::new(CompressorOxide::new(0)));
    }

    COMPRESSOR.with_borrow_mut(|compressor| {
        compressor.reset();
        compressor.set_format_and_level(DataFormat::Zlib, 1);

        let mut input = data;
        let mut output = vec![0; (data.len() / 2).max(64)];
        let mut out_pos = 0;
        loop {
            let (status, bytes_in, bytes_out) = compress(compressor, input, &mut output[out_pos..], TDEFLFlush::Finish);
            out_pos += bytes_out;
            input = &input[bytes_in..];
            match status {
                TDEFLStatus::Done => {
                    output.truncate(out_pos);
                    return output;
                }
                TDEFLStatus::Okay => {
                    if output.len() - out_pos < 64 {
                        output.resize(output.len() * 2, 0);
                    }
                }
                status => panic!("deflate failed: {status:?}"),
            }
        }
    })
}

fn render_instruction(ctx: &mut Ctx, instruction: &DrawInstruction) {
    match instruction {
        DrawInstruction::DrawText { .. } => text::render(ctx, instruction),
        DrawInstruction::DrawBarcode { .. }
        | DrawInstruction::DrawLinear { .. }
        | DrawInstruction::DrawQrCode { .. }
        | DrawInstruction::DrawDataMatrix { .. }
        | DrawInstruction::DrawPdf417 { .. }
        | DrawInstruction::DrawAztec { .. } => barcodes::render(ctx, instruction),
        DrawInstruction::DrawBox { .. }
        | DrawInstruction::DrawCircle { .. }
        | DrawInstruction::DrawEllipse { .. }
        | DrawInstruction::DrawDiagonalLine { .. } => shapes::render(ctx, instruction),
        DrawInstruction::DrawGraphicField { .. } => graphic::render(ctx, instruction),
        DrawInstruction::Reverse(inner) => {
            // Painting white with the Difference blend mode inverts the backdrop.
            ctx.uses_difference = true;
            ctx.content.save_state();
            ctx.content.set_parameters(Name(b"GsDiff")).set_fill_gray(1.0);
            let ink = std::mem::replace(&mut ctx.ink, 1.0);
            render_instruction(ctx, inner);
            ctx.ink = ink;
            ctx.content.restore_state();
        }
    }
}

pub fn save_pdf(bytes: &[u8], path: &str) -> std::io::Result<()> {
    std::fs::write(path, bytes)
}
