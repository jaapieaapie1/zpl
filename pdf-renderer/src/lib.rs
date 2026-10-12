//! Vector PDF renderer for ZPL draw instructions.
//!
//! Every element is emitted as native PDF content: text as embedded-font text objects,
//! boxes/lines/circles and barcode bars/QR modules as filled paths, and ^GF graphics as
//! 1-bit image masks. No raster of the whole label is ever produced.
//!
//! Coordinate system: the page content stream starts with a CTM that maps printer dots
//! (origin top-left, y down) to PDF points. All modules therefore draw in dot units.
//! Anything that renders glyphs must flip y back locally (see `text.rs`).

use pdf_writer::writers::Resources;
use pdf_writer::{Buf, Content, Filter, Finish, Name, Pdf, Rect, Ref};
use zpl_state_manager::{DrawInstruction, PageSetup, DEFAULT_DPMM};

mod barcodes;
mod fonts;
mod graphic;
mod matrix;
mod shapes;
mod symbols;
mod text;

pub use fonts::FontId;

pub struct PdfRenderer {
    width: u32,
    height: u32,
    dpmm: u32,
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
    /// Printer resolution of the current page, for the bitmap font matrices and MaxiCode.
    pub dpmm: u32,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            content: Content::new(),
            fonts: Default::default(),
            images: Vec::new(),
            uses_difference: false,
            ink: 0.0,
            dpmm: DEFAULT_DPMM,
        }
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
        Self::with_dpmm(width, height, DEFAULT_DPMM)
    }

    /// Renderer for a `width` x `height` dot label on a `dpmm` dots/mm printer (6, 8, 12 or
    /// 24): each dot is 72 / 152, 203, 300 or 600 points.
    pub fn with_dpmm(width: u32, height: u32, dpmm: u32) -> Self {
        Self { width, height, dpmm, parallel: false, inverted: false, mirrored: false }
    }

    /// Renderer for a label's page settings: size and resolution plus ^PO inversion and ^PM
    /// mirroring.
    pub fn for_page(page: &PageSetup) -> Self {
        Self { inverted: page.inverted, mirrored: page.mirrored, ..Self::with_dpmm(page.width, page.height, page.dpmm) }
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
        let page = PageSetup {
            width: self.width,
            height: self.height,
            inverted: self.inverted,
            mirrored: self.mirrored,
            dpmm: self.dpmm,
            scale: (1, 1),
        };
        self.render_pages(&[(page, instructions)])
    }

    /// Renders one PDF page per label, each with its own size, resolution and ^PO / ^PM
    /// settings; the renderer's own page settings are not used. Fonts and images are shared
    /// by all pages, so each font is embedded once, subset to the glyphs of every page.
    pub fn render_pages(&self, pages: &[(PageSetup, &[DrawInstruction])]) -> Vec<u8> {
        let mut ctx = Ctx::new();
        let mut contents = Vec::with_capacity(pages.len());
        for (setup, instructions) in pages {
            // Points per format dot: 72 / dpi, times the ^JM / ^MU format scale.
            let scale = 72.0 / setup.format_dpi();
            ctx.dpmm = setup.dpmm;
            begin_page(&mut ctx, setup, scale);
            for instruction in *instructions {
                render_instruction(&mut ctx, instruction);
            }
            let content = std::mem::replace(&mut ctx.content, Content::new()).finish();
            contents.push((content, setup.width as f32 * scale, setup.height as f32 * scale));
        }
        self.write_document(ctx, contents)
    }

    /// `pages` holds each page's raw content stream and size in points.
    fn write_document(&self, ctx: Ctx, pages: Vec<(Buf, f32, f32)>) -> Vec<u8> {
        let mut alloc = Ref::new(1);
        let catalog_id = alloc.bump();
        let pages_id = alloc.bump();
        let resources_id = alloc.bump();
        let gs_diff_id = alloc.bump();
        let page_ids: Vec<(Ref, Ref)> = pages.iter().map(|_| (alloc.bump(), alloc.bump())).collect();

        let mut pdf = Pdf::new();
        pdf.catalog(catalog_id).pages(pages_id);
        pdf.pages(pages_id).kids(page_ids.iter().map(|&(page, _)| page)).count(page_ids.len() as i32);

        // With `parallel_fonts`, font subsetting runs on worker threads while this thread
        // deflates the rest.
        let (programs, (contents, images)) = ctx.fonts.build_programs(self.parallel, || {
            let images: Vec<Vec<u8>> = ctx.images.iter().map(|img| compress(&img.data)).collect();
            let contents: Vec<Vec<u8>> = pages.iter().map(|(raw, ..)| compress(raw)).collect();
            (contents, images)
        });

        let font_refs = ctx.fonts.write(&mut pdf, &mut alloc, programs);
        let image_refs: Vec<Ref> =
            ctx.images.iter().zip(&images).map(|(img, data)| img.write(&mut pdf, &mut alloc, data)).collect();

        if ctx.uses_difference {
            pdf.ext_graphics(gs_diff_id).blend_mode(pdf_writer::types::BlendMode::Difference);
        }

        // One resource dictionary, shared by every page.
        let mut resources = pdf.indirect(resources_id).start::<Resources>();
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

        for ((_, page_w, page_h), (&(page_id, content_id), content)) in pages.iter().zip(page_ids.iter().zip(&contents)) {
            let mut page = pdf.page(page_id);
            page.media_box(Rect::new(0.0, 0.0, *page_w, *page_h));
            page.parent(pages_id);
            page.contents(content_id);
            page.pair(Name(b"Resources"), resources_id);
            page.finish();
            pdf.stream(content_id, content).filter(Filter::FlateDecode);
        }

        pdf.finish()
    }
}

/// Starts a page's content: the dots-to-points transform, ^PO / ^PM and the label stock.
fn begin_page(ctx: &mut Ctx, setup: &PageSetup, scale: f32) {
    let (w, h) = (setup.width as f32, setup.height as f32);

    // Map dots (top-left origin, y down) to points (bottom-left origin, y up).
    ctx.content.transform([scale, 0.0, 0.0, -scale, 0.0, h * scale]);
    // ^PM mirrors left-right, ^POI turns the label upside down; both flips cancel x.
    match (setup.inverted, setup.mirrored) {
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
    ctx.content.set_fill_gray(1.0).rect(0.0, 0.0, w, h).fill_nonzero().set_fill_gray(0.0);
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
        DrawInstruction::DrawSymbol { .. } => symbols::render(ctx, instruction),
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

#[cfg(test)]
mod tests {
    use super::*;
    use zpl_parser::Parser;
    use zpl_state_manager::StateManager;

    fn render_zpl(zpl: &str) -> String {
        let pages = StateManager::new().process_pages(Parser::new(zpl).parse().unwrap());
        let pages: Vec<_> = pages.iter().map(|p| (p.setup, p.instructions.as_slice())).collect();
        let pdf = PdfRenderer::new(1, 1).render_pages(&pages);
        // Only the dictionaries are inspected; streams are binary.
        String::from_utf8_lossy(&pdf).into_owned()
    }

    fn count(pdf: &str, needle: &str) -> usize {
        pdf.matches(needle).count()
    }

    #[test]
    fn one_pdf_page_per_label() {
        let pdf = render_zpl("^XA^PW406^LL203^FO10,10^A0N,30^FDone^FS^XZ^XA^PW203^FO10,10^A0N,30^FDtwo^FS^XZ");
        assert_eq!(count(&pdf, "/Type /Page\n"), 2);
        assert!(pdf.contains("/Count 2"));
        // Each page has its own size (^LL carries over); one font embedding serves both.
        assert!(pdf.contains("/MediaBox [0 0 144 72]"));
        assert!(pdf.contains("/MediaBox [0 0 72 72]"));
        assert_eq!(count(&pdf, "/FontFile"), 1);
    }

    #[test]
    fn copies_share_images() {
        let pdf = render_zpl("^XA^PQ3^FO10,10^GFA,2,2,1,FF00^FS^XZ");
        assert_eq!(count(&pdf, "/Type /Page\n"), 3);
        assert_eq!(count(&pdf, "/ImageMask"), 1);
    }

    fn render_at(zpl: &str, dpmm: u32) -> String {
        let mut state = StateManager::new();
        state.set_dpmm(dpmm);
        let pages = state.process_pages(Parser::with_dpmm(zpl, dpmm).parse().unwrap());
        let pages: Vec<_> = pages.iter().map(|p| (p.setup, p.instructions.as_slice())).collect();
        String::from_utf8_lossy(&PdfRenderer::for_page(&pages[0].0).render_pages(&pages)).into_owned()
    }

    #[test]
    fn page_size_follows_resolution() {
        // The default label is 4 x 6 inches (288 x 432 pt) at every resolution.
        for dpmm in [6, 8, 12, 24] {
            assert!(render_at("^XA^FO10,10^GB50,50,5^FS^XZ", dpmm).contains("/MediaBox [0 0 288 432]"), "{dpmm}");
        }
        // 4 x 6 inches in dots.
        assert!(render_at("^XA^PW812^LL1218^XZ", 8).contains("/MediaBox [0 0 288 432]"));
        assert!(render_at("^XA^PW1200^LL1800^XZ", 12).contains("/MediaBox [0 0 288 432]"));
        // The same dots are smaller on a finer printhead.
        assert!(render_at("^XA^PW600^LL300^XZ", 12).contains("/MediaBox [0 0 144 72]"));
        assert!(PdfRenderer::with_dpmm(600, 300, 12).render(&[]).windows(22).any(|w| w == b"/MediaBox [0 0 144 72]"));
        // ^JMB: half as many, twice as large dots.
        assert!(render_at("^XA^JMB^PW406^LL609^XZ", 8).contains("/MediaBox [0 0 288 432]"));
        assert!(render_at("^XA^JMB^FO1,1^GB5,5,5^FS^XZ", 12).contains("/MediaBox [0 0 288 432]"));
    }

    #[test]
    fn control_characters_print_nothing() {
        let plain = render_zpl("^XA^FO10,10^A0N,30^FDVAN:^FS^FO10,60^ADN^FDab^FS^XZ");
        assert_eq!(render_zpl("^XA^FO10,10^A0N,30^FDVAN:\r\n^FS^FO10,60^ADN^FDa\nb^FS^XZ"), plain);
        assert_eq!(render_zpl("^XA^FO10,10^A0N,30^FH^FDVAN:_0D_0A^FS^FO10,60^ADN^FH^FDa_09_7Fb^FS^XZ"), plain);
    }

    #[test]
    fn render_is_a_single_page() {
        let pdf = PdfRenderer::new(203, 203).render(&[]);
        let pdf = String::from_utf8_lossy(&pdf);
        assert_eq!(count(&pdf, "/Type /Page\n"), 1);
        assert!(pdf.contains("/MediaBox [0 0 72 72]"));
    }
}
