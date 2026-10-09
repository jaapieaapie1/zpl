use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use zpl_parser::command::{
    BlockJustification, Code128Mode, DiagonalDirection, Justification, LineColor, LinearKind, Orientation,
};
use zpl_state_manager::{DrawInstruction, TextAnchor, TextBlock};
use zpl_symbols::linear::{self, LinearSymbol, Symbology};
use zpl_symbols::matrix::{self, Matrix};

pub mod code128;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

pub struct PngRenderer {
    width: u32,
    height: u32,
    debug: bool,
}

/// Scalable text font (^A0 and friends) at a given field size.
struct TextFont {
    font: FontRef<'static>,
    scale: PxScale,
    /// Distance from ab_glyph's draw origin (ascent line) down to the cap line.
    y_offset: i32,
    /// Cap height in dots: distance from the top of capitals to the baseline.
    cap: i32,
}

impl TextFont {
    fn new(height: u32, width: u32) -> Self {
        use ab_glyph::Font;

        let font_data: &'static [u8] = include_bytes!("../fonts/ATTriumvirate-BoldCd.otf");
        let font = FontRef::try_from_slice(font_data).expect("Error loading font");

        // Zebra sizes scalable fonts in em units, ab_glyph's PxScale is ascent - descent
        let units_per_em = font.units_per_em().unwrap_or(1000.0);
        let px_per_em = font.height_unscaled() / units_per_em;
        let width = if width == 0 { height } else { width };
        let scale = PxScale {
            x: width as f32 * px_per_em,
            y: height as f32 * px_per_em,
        };

        // Zebra places the top of capital letters at the field origin, ab_glyph places the ascent there
        let cap_height = units_per_em * 0.75;
        let y_offset = ((font.ascent_unscaled() - cap_height) / units_per_em * height as f32).round() as i32;
        let cap = (height as f32 * 0.75).round() as i32;
        Self { font, scale, y_offset, cap }
    }

    /// Advance width of `text` in dots.
    fn measure(&self, text: &str) -> f32 {
        use ab_glyph::{Font, ScaleFont};
        let scaled = self.font.as_scaled(self.scale);
        text.chars().map(|c| scaled.h_advance(self.font.glyph_id(c))).sum()
    }
}

/// Interpretation line font for 1D barcodes: monospaced; pitch, digit height and gap scale
/// with the module width (matches Labelary).
struct Interpretation {
    font: FontRef<'static>,
    scale: PxScale,
    char_advance: f32,
    digit_height: f32,
    ascent: f32,
    /// Negative, as ab_glyph reports it.
    descent: f32,
}

impl Interpretation {
    fn new(bar_width: u32) -> Self {
        use ab_glyph::{Font, ScaleFont};

        let font_data: &'static [u8] = include_bytes!("../fonts/DejaVuSansMono.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Error loading font");
        let char_advance = bar_width as f32 * 143.0 / 24.0;
        let digit_height = (bar_width * 7) as f32;
        let zero = font.glyph_id('0');
        let zero_height_unscaled = font
            .outline(zero)
            .map(|o| (o.bounds.max.y - o.bounds.min.y).abs())
            .unwrap_or(font.ascent_unscaled());
        let scale = PxScale {
            x: char_advance * font.height_unscaled() / font.h_advance_unscaled(zero),
            y: digit_height * font.height_unscaled() / zero_height_unscaled,
        };
        let scaled = font.as_scaled(scale);
        let (ascent, descent) = (scaled.ascent(), scaled.descent());
        Self { font, scale, char_advance, digit_height, ascent, descent }
    }

    fn text_width(&self, text: &str) -> f32 {
        self.char_advance * text.chars().count() as f32
    }

    /// Draws `text` one character per pitch, starting at `x` with its baseline at `baseline`.
    fn draw(&self, image: &mut RgbImage, text: &str, x: f32, baseline: f32) {
        let text_y = baseline - self.ascent;
        for (i, ch) in text.chars().enumerate() {
            let ch_x = x + i as f32 * self.char_advance;
            drawing::draw_text_mut(
                image,
                BLACK,
                ch_x.round() as i32,
                text_y.round() as i32,
                self.scale,
                &self.font,
                &ch.to_string(),
            );
        }
    }
}

fn symbology(kind: LinearKind) -> Symbology {
    match kind {
        LinearKind::Code11 { check_digits } => Symbology::Code11 { check_digits },
        LinearKind::Interleaved2of5 { check_digit } => Symbology::Interleaved2of5 { check_digit },
        LinearKind::Code39 { check_digit } => Symbology::Code39 { check_digit },
        LinearKind::Ean8 => Symbology::Ean8,
        LinearKind::UpcE { .. } => Symbology::UpcE,
        LinearKind::Code93 { print_check_digit } => Symbology::Code93 { check_digit: print_check_digit },
        LinearKind::Ean13 => Symbology::Ean13,
        LinearKind::Industrial2of5 => Symbology::Industrial2of5,
        LinearKind::Standard2of5 => Symbology::Standard2of5,
        LinearKind::Codabar { check_digit, .. } => Symbology::Codabar { check_digit },
        LinearKind::Msi { .. } => Symbology::Msi { check: linear::MsiCheck::None },
        LinearKind::Plessey { print_check_digit } => Symbology::Plessey { check_digit: print_check_digit },
        LinearKind::UpcEanExtension => Symbology::UpcEanExtension,
        LinearKind::UpcA { .. } => Symbology::UpcA,
    }
}

fn encode_linear(kind: LinearKind, data: &str) -> Option<LinearSymbol> {
    use zpl_parser::command::MsiCheck as Zpl;
    let mut symbol = match kind {
        LinearKind::Codabar { check_digit, start, stop } => linear::encode_codabar(data, start, stop, check_digit),
        LinearKind::Msi { check, print_check_digit } => {
            let check = match check {
                Zpl::None => linear::MsiCheck::None,
                Zpl::Mod10 => linear::MsiCheck::Mod10,
                Zpl::Mod10Mod10 => linear::MsiCheck::Mod10Mod10,
                Zpl::Mod11Mod10 => linear::MsiCheck::Mod11Mod10,
            };
            linear::encode_msi(data, check, print_check_digit)
        }
        kind => linear::encode(symbology(kind), data),
    }
    .ok()?;
    // UPC-A/UPC-E print the check digit as the last (outside) text segment.
    if matches!(
        kind,
        LinearKind::UpcA { print_check_digit: false } | LinearKind::UpcE { print_check_digit: false }
    ) {
        symbol.text.pop();
    }
    Some(symbol)
}

/// Greedy word wrap of `text` into lines no wider than `width` (the first line) or
/// `width - indent` (the others). `\&` forces a line break; overlong words are split.
fn wrap_lines(font: &TextFont, text: &str, width: u32, indent: u32) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let avail = |lines: &Vec<String>| {
        if lines.is_empty() { width as f32 } else { width.saturating_sub(indent).max(1) as f32 }
    };
    for paragraph in text.split("\\&") {
        let mut line = String::new();
        for word in paragraph.split(' ').filter(|w| !w.is_empty()) {
            let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
            if font.measure(&candidate) <= avail(&lines) {
                line = candidate;
                continue;
            }
            if !line.is_empty() {
                lines.push(std::mem::take(&mut line));
            }
            // Split a word that does not fit on a line of its own.
            for c in word.chars() {
                line.push(c);
                if font.measure(&line) > avail(&lines) && line.chars().count() > 1 {
                    line.pop();
                    lines.push(std::mem::take(&mut line));
                    line.push(c);
                }
            }
        }
        lines.push(line);
    }
    lines
}

impl PngRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, debug: false }
    }

    pub fn with_debug(width: u32, height: u32, debug: bool) -> Self {
        Self { width, height, debug }
    }

    pub fn render(&self, instructions: &[DrawInstruction]) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::from_pixel(self.width, self.height, WHITE);

        for instruction in instructions {
            self.render_instruction(&mut image, instruction);
        }

        image
    }

    fn render_instruction(&self, image: &mut RgbImage, instruction: &DrawInstruction) {
        match instruction {
            DrawInstruction::DrawText {
                x, y, text, height, width, orientation, justification, anchor, block, ..
            } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([255, 0, 0]));
                }
                self.render_text(
                    image, *x, *y, text, *height, *width, *orientation, *justification, *anchor, block.as_ref(),
                );
            }
            DrawInstruction::DrawBarcode {
                x, y, data, orientation, height, module_width,
                print_interpretation_line, print_interpretation_line_above, mode, ..
            } => {
                self.render_code128(
                    image, *x, *y, data, orientation, *height, *module_width,
                    *print_interpretation_line, *print_interpretation_line_above, *mode,
                );
            }
            DrawInstruction::DrawLinear {
                x, y, kind, data, orientation, height, module_width, wide_bar_ratio,
                print_interpretation_line, print_interpretation_line_above,
            } => {
                self.render_linear(
                    image, *x, *y, *kind, data, *orientation, *height, *module_width, *wide_bar_ratio,
                    *print_interpretation_line, *print_interpretation_line_above,
                );
            }
            DrawInstruction::DrawQrCode { x, y, data, magnification, bottom, .. } => {
                self.render_qr_code(image, *x, *y, data, *magnification, *bottom);
            }
            DrawInstruction::DrawDataMatrix {
                x, y, data, orientation, module_height, symbol_height, columns, rows, escape, rectangular, bottom,
            } => {
                if let Ok(m) = matrix::datamatrix(data.as_bytes(), *rows, *columns, *rectangular, *escape) {
                    // Without an explicit module size the ^BY height is the approximate symbol height.
                    let module = module_height.unwrap_or(symbol_height / m.height.max(1)).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, module, *bottom);
                }
            }
            DrawInstruction::DrawPdf417 {
                x, y, data, orientation, row_height, module_width, symbol_height, security, columns, rows,
                truncated, bottom,
            } => {
                if let Ok(m) = matrix::pdf417(data.as_bytes(), *security, *columns, *rows, *truncated) {
                    let module = (*module_width).max(1);
                    // Row height is a multiple of the module width, or the ^BY height shared by all rows.
                    let row = row_height.map(|h| h * module).unwrap_or(symbol_height / m.height.max(1)).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, row, *bottom);
                }
            }
            DrawInstruction::DrawAztec { x, y, data, orientation, magnification, ecc, bottom } => {
                if let Ok(m) = matrix::aztec(data.as_bytes(), *ecc) {
                    let module = (*magnification).max(1);
                    self.render_matrix(image, &m, *x, *y, *orientation, module, module, *bottom);
                }
            }
            DrawInstruction::DrawBox { x, y, width, height, thickness, color, .. } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 255, 0]));
                }
                self.render_box(image, *x, *y, *width, *height, *thickness, color);
            }
            DrawInstruction::DrawCircle { x, y, diameter, thickness, color } => {
                self.render_ellipse(image, *x, *y, *diameter, *diameter, *thickness, color);
            }
            DrawInstruction::DrawEllipse { x, y, width, height, thickness, color } => {
                self.render_ellipse(image, *x, *y, *width, *height, *thickness, color);
            }
            DrawInstruction::DrawDiagonalLine { x, y, width, height, thickness, color, direction } => {
                self.render_diagonal_line(image, *x, *y, *width, *height, *thickness, color, direction);
            }
            DrawInstruction::DrawGraphicField { x, y, bytes_per_row, data, magnification_x, magnification_y } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 0, 255]));
                }
                self.render_graphic_field(image, *x, *y, *bytes_per_row, data, *magnification_x, *magnification_y);
            }
            DrawInstruction::Reverse(inner) => self.render_reverse(image, inner),
        }
    }

    /// ^FR / ^LR: draw `inner` on a blank page, then invert every target pixel it blackened.
    fn render_reverse(&self, image: &mut RgbImage, inner: &DrawInstruction) {
        // No debug markers on the scratch page: they would be inverted too.
        let plain = PngRenderer { width: image.width(), height: image.height(), debug: false };
        let mut scratch: RgbImage = ImageBuffer::from_pixel(image.width(), image.height(), WHITE);
        plain.render_instruction(&mut scratch, inner);
        for (target, drawn) in image.pixels_mut().zip(scratch.pixels()) {
            let luma = (drawn[0] as u32 * 299 + drawn[1] as u32 * 587 + drawn[2] as u32 * 114) / 1000;
            if luma < 128 {
                *target = Rgb([255 - target[0], 255 - target[1], 255 - target[2]]);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_text(
        &self,
        image: &mut RgbImage,
        x: u32,
        y: u32,
        text: &str,
        height: u32,
        width: u32,
        orientation: Orientation,
        justification: Justification,
        anchor: TextAnchor,
        block: Option<&TextBlock>,
    ) {
        let font = TextFont::new(height, width);

        // Height of the unrotated frame per orientation; the placement below is tuned to these.
        let frame_extra = match orientation {
            Orientation::Rotated270 => height + 10,
            _ => height * 2,
        };

        // (x offset, top of capitals, text) per row, in the unrotated frame.
        let mut rows: Vec<(i32, i32, String)> = Vec::new();
        let (frame_w, frame_h, last_row_top, justification) = match block {
            None => {
                let w = font.measure(text).ceil() as u32 + 10;
                rows.push((0, 0, text.to_string()));
                (w, frame_extra, 0, justification)
            }
            Some(b) => {
                let line_height = (height as i32 + b.line_spacing).max(1);
                let mut lines = wrap_lines(&font, text, b.width, b.hanging_indent);
                if let Some(clip) = b.clip_height {
                    // ^TB: drop lines that do not fit in the block height (keep at least one).
                    let fit = ((clip as i32 - height as i32) / line_height + 1).max(1) as usize;
                    lines.truncate(fit);
                }
                let max_rows = b.max_lines.max(1) as usize;
                let mut last_row = 0;
                for (i, line) in lines.into_iter().enumerate() {
                    // ^FB: lines beyond the maximum overprint the last one.
                    let row = i.min(max_rows - 1);
                    last_row = last_row.max(row);
                    let indent = if i > 0 { b.hanging_indent as f32 } else { 0.0 };
                    let line_w = font.measure(&line);
                    let free = (b.width as f32 - indent - line_w).max(0.0);
                    let x_off = match b.justification {
                        BlockJustification::Left | BlockJustification::Justified => indent,
                        BlockJustification::Center => indent + free / 2.0,
                        BlockJustification::Right => indent + free,
                    };
                    rows.push((x_off.round() as i32, row as i32 * line_height, line));
                }
                let last_top = last_row as i32 * line_height;
                (b.width.max(1), last_top as u32 + frame_extra, last_top, Justification::Left)
            }
        };
        if frame_w == 0 || frame_h == 0 {
            return;
        }

        let mut temp: RgbImage = ImageBuffer::from_pixel(frame_w, frame_h, WHITE);
        for (x_off, top, line) in &rows {
            drawing::draw_text_mut(&mut temp, BLACK, *x_off, top - font.y_offset, font.scale, &font.font, line);
        }

        // ^FT: the origin is the baseline of the (last) line; move it to where the top would be.
        let (mut x, mut y) = (x as i64, y as i64);
        if anchor == TextAnchor::Baseline {
            let up = (last_row_top + font.cap) as i64;
            let below = frame_h as i64 - up;
            match orientation {
                Orientation::Normal => y -= up,
                Orientation::Rotated90 => x -= below,
                Orientation::Rotated180 => y -= below,
                Orientation::Rotated270 => x += below,
            }
        }

        self.place_text(image, &temp, x, y, orientation, justification);
    }

    /// Places an unrotated text frame at the field origin, rotated and justified.
    fn place_text(
        &self,
        image: &mut RgbImage,
        temp: &RgbImage,
        x: i64,
        y: i64,
        orientation: Orientation,
        justification: Justification,
    ) {
        let justify = |x: i64, w: u32| match justification {
            Justification::Left => x,
            Justification::Right => x - w as i64,
            Justification::Auto => x - (w / 2) as i64,
        };
        match orientation {
            Orientation::Normal => {
                self.overlay_at(image, temp, justify(x, temp.width()), y);
            }
            Orientation::Rotated90 => {
                let rotated = image::imageops::rotate90(temp);
                self.overlay_at(image, &rotated, justify(x, rotated.width()), y);
            }
            Orientation::Rotated180 => {
                let rotated = image::imageops::rotate180(temp);
                self.overlay_at(image, &rotated, justify(x, temp.width()), y);
            }
            Orientation::Rotated270 => {
                let rotated = image::imageops::rotate270(temp);
                let adjusted_x = match justification {
                    Justification::Left => x - rotated.width() as i64,
                    Justification::Right => x,
                    Justification::Auto => x - (rotated.width() / 2) as i64,
                };
                let adjusted_y = y - rotated.height() as i64;

                if self.debug && adjusted_x >= 0 && adjusted_y >= 0 {
                    self.draw_debug_box(
                        image, adjusted_x as u32, adjusted_y as u32, rotated.width(), rotated.height(), Rgb([255, 0, 0]),
                    );
                }

                self.overlay_at(image, &rotated, adjusted_x, adjusted_y);
            }
        }
    }

    fn overlay_image(&self, target: &mut RgbImage, source: &RgbImage, x: u32, y: u32) {
        self.overlay_at(target, source, x as i64, y as i64);
    }

    /// Copies the non-white pixels of `source` onto `target` at (`x`, `y`), clipping at the edges.
    fn overlay_at(&self, target: &mut RgbImage, source: &RgbImage, x: i64, y: i64) {
        for (src_x, src_y, pixel) in source.enumerate_pixels() {
            let target_x = x + src_x as i64;
            let target_y = y + src_y as i64;

            if target_x >= 0
                && target_y >= 0
                && target_x < target.width() as i64
                && target_y < target.height() as i64
                && (pixel[0] < 250 || pixel[1] < 250 || pixel[2] < 250)
            {
                target.put_pixel(target_x as u32, target_y as u32, *pixel);
            }
        }
    }

    /// Places an unrotated symbol image whose bars start `pad_l` dots from its left edge (with
    /// `pad_r` dots of text overhang on the right) at the field origin, rotated like the bars.
    #[allow(clippy::too_many_arguments)]
    fn place_symbol(
        &self,
        image: &mut RgbImage,
        temp: &RgbImage,
        x: u32,
        y: u32,
        orientation: Orientation,
        pad_l: u32,
        pad_r: u32,
    ) {
        let (x, y) = (x as i64, y as i64);
        match orientation {
            Orientation::Normal => self.overlay_at(image, temp, x - pad_l as i64, y),
            Orientation::Rotated90 => {
                let rotated = image::imageops::rotate90(temp);
                self.overlay_at(image, &rotated, x, y - pad_l as i64);
            }
            Orientation::Rotated180 => {
                let rotated = image::imageops::rotate180(temp);
                self.overlay_at(image, &rotated, x - pad_r as i64, y);
            }
            Orientation::Rotated270 => {
                let rotated = image::imageops::rotate270(temp);
                let adjusted_x = x - rotated.width() as i64;
                let adjusted_y = y - rotated.height() as i64 + pad_l as i64;
                self.overlay_at(image, &rotated, adjusted_x, adjusted_y);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_code128(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, orientation: &Orientation, height: u32, module_width: u32, print_interpretation_line: bool, print_interpretation_line_above: bool, mode: Code128Mode) {
        let code = code128::encode(data, mode);
        let bar_width = module_width.max(1);
        let widths = code.widths();
        let barcode_width: u32 = widths.iter().map(|&w| w as u32 * bar_width).sum();

        let ip = Interpretation::new(bar_width);
        let text_gap = (bar_width * 2) as f32;

        let show_text = print_interpretation_line && !code.text.is_empty();
        let text_width = if show_text { ip.text_width(&code.text).ceil() as u32 } else { 0 };
        let text_area = if show_text { (text_gap + ip.digit_height - ip.descent).ceil() as u32 } else { 0 };

        let pad = text_width.saturating_sub(barcode_width) / 2;
        let total_width = barcode_width + 2 * pad;
        let total_height = height + text_area;
        let (bars_y, baseline) = if print_interpretation_line_above {
            (text_area, ip.digit_height)
        } else {
            (0, height as f32 + text_gap + ip.digit_height)
        };

        let mut temp_image: RgbImage = ImageBuffer::from_pixel(total_width.max(1), total_height.max(1), WHITE);

        let mut x_pos = pad;
        for (i, &width) in widths.iter().enumerate() {
            let pixel_width = width as u32 * bar_width;
            if i % 2 == 0 {
                for px in x_pos..x_pos + pixel_width {
                    for py in bars_y..bars_y + height {
                        temp_image.put_pixel(px, py, BLACK);
                    }
                }
            }
            x_pos += pixel_width;
        }

        if show_text {
            let text_x = pad as f32 + (barcode_width as f32 - text_width as f32) / 2.0;
            ip.draw(&mut temp_image, &code.text, text_x, baseline);
        }

        match orientation {
            Orientation::Normal => {
                self.overlay_image(image, &temp_image, x.saturating_sub(pad), y);
            }
            Orientation::Rotated90 => {
                let rotated = image::imageops::rotate90(&temp_image);
                self.overlay_image(image, &rotated, x, y.saturating_sub(pad));
            }
            Orientation::Rotated180 => {
                let rotated = image::imageops::rotate180(&temp_image);
                self.overlay_image(image, &rotated, x.saturating_sub(pad), y);
            }
            Orientation::Rotated270 => {
                let rotated = image::imageops::rotate270(&temp_image);
                let adjusted_x = x.saturating_sub(rotated.width());
                let adjusted_y = y.saturating_sub(rotated.height()).saturating_sub(pad);
                self.overlay_image(image, &rotated, adjusted_x, adjusted_y);
            }
        }
    }

    /// All 1D symbologies except Code 128. Bars start at the field origin; the interpretation
    /// line goes below, or above when requested. EAN/UPC guard bars extend halfway into the digits.
    #[allow(clippy::too_many_arguments)]
    fn render_linear(
        &self,
        image: &mut RgbImage,
        x: u32,
        y: u32,
        kind: LinearKind,
        data: &str,
        orientation: Orientation,
        height: u32,
        module_width: u32,
        ratio: f32,
        print_line: bool,
        line_above: bool,
    ) {
        let Some(symbol) = encode_linear(kind, data) else { return };
        let module = module_width.max(1);
        let width = symbol.width_dots(module, ratio);

        let ip = Interpretation::new(module);
        let text_gap = (module * 2) as f32;
        let show_text = print_line && !symbol.text.is_empty();
        let text_area = if show_text { (text_gap + ip.digit_height - ip.descent).ceil() as u32 } else { 0 };
        let guard_ext = if symbol.has_guards && show_text && !line_above {
            (text_gap + ip.digit_height / 2.0).round() as u32
        } else {
            0
        };
        let bars_y = if line_above { text_area } else { 0 };

        // Text segments (e.g. the EAN-13 leading digit) may stick out of the bars.
        let mut segments = Vec::new();
        let (mut left, mut right) = (0.0f32, width as f32);
        if show_text {
            for seg in &symbol.text {
                let (start, end) = symbol.text_extent_dots(seg, module, ratio);
                let text_x = (start + end - ip.text_width(&seg.text)) / 2.0;
                left = left.min(text_x);
                right = right.max(text_x + ip.text_width(&seg.text));
                segments.push((text_x, seg.text.as_str()));
            }
        }
        let pad_l = (-left).ceil() as u32;
        let pad_r = (right - width as f32).ceil() as u32;

        let total_w = (pad_l + width + pad_r).max(1);
        let total_h = (height + text_area).max(1);
        let mut temp: RgbImage = ImageBuffer::from_pixel(total_w, total_h, WHITE);

        let mut x_pos = pad_l;
        for (element, dots) in symbol.element_dots(module, ratio) {
            if element.bar {
                let bar_h = height + if element.guard { guard_ext } else { 0 };
                for px in x_pos..(x_pos + dots).min(total_w) {
                    for py in bars_y..(bars_y + bar_h).min(total_h) {
                        temp.put_pixel(px, py, BLACK);
                    }
                }
            }
            x_pos += dots;
        }

        if show_text {
            let baseline = if line_above { ip.digit_height } else { height as f32 + text_gap + ip.digit_height };
            for (text_x, text) in segments {
                ip.draw(&mut temp, text, pad_l as f32 + text_x, baseline);
            }
        }

        self.place_symbol(image, &temp, x, y, orientation, pad_l, pad_r);
    }

    /// 2D symbol with `module_w` x `module_h` dot modules; with `bottom` (^FT) `y` is the
    /// symbol's bottom edge.
    #[allow(clippy::too_many_arguments)]
    fn render_matrix(
        &self,
        image: &mut RgbImage,
        m: &Matrix,
        x: u32,
        y: u32,
        orientation: Orientation,
        module_w: u32,
        module_h: u32,
        bottom: bool,
    ) {
        let (w, h) = (m.width * module_w, m.height * module_h);
        if w == 0 || h == 0 {
            return;
        }
        let y = if bottom { y.saturating_sub(h) } else { y };
        let mut temp: RgbImage = ImageBuffer::from_pixel(w, h, WHITE);
        for row in 0..m.height {
            for col in 0..m.width {
                if m.get(col, row) {
                    for py in row * module_h..(row + 1) * module_h {
                        for px in col * module_w..(col + 1) * module_w {
                            temp.put_pixel(px, py, BLACK);
                        }
                    }
                }
            }
        }
        self.place_symbol(image, &temp, x, y, orientation, 0, 0);
    }

    fn render_qr_code(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, magnification: u32, bottom: bool) {
        use qrcode::{EcLevel, QrCode};

        // ^FD for QR codes starts with "<error correction><input mode>," e.g. "QA,data"
        let (ec_level, payload) = match data.get(..3) {
            Some(prefix) if prefix.ends_with(',') => {
                let ec_level = match &prefix[..1] {
                    "H" => EcLevel::H,
                    "Q" => EcLevel::Q,
                    "L" => EcLevel::L,
                    _ => EcLevel::M,
                };
                (ec_level, &data[3..])
            }
            _ => (EcLevel::M, data),
        };

        let module_size = magnification.max(1);

        if let Ok(code) = QrCode::with_error_correction_level(payload.as_bytes(), ec_level) {
            let qr_image = code.render::<Rgb<u8>>()
                .quiet_zone(false)
                .module_dimensions(module_size, module_size)
                .build();

            // ^FT: y is the bottom edge. ^FO: Zebra printers draw QR codes 10 dots below the field origin.
            let y = if bottom {
                y.saturating_sub(code.width() as u32 * module_size)
            } else {
                y + 10
            };

            for (qr_x, qr_y, pixel) in qr_image.enumerate_pixels() {
                let target_x = x + qr_x;
                let target_y = y + qr_y;

                if target_x < image.width() && target_y < image.height() {
                    image.put_pixel(target_x, target_y, *pixel);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_box(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor) {
        let rgb_color = self.line_color_to_rgb(color);

        for t in 0..thickness {
            let new_width = width.saturating_sub(2 * t);
            let new_height = height.saturating_sub(2 * t);

            if new_width == 0 || new_height == 0 {
                break;
            }

            let rect = imageproc::rect::Rect::at(x as i32 + t as i32, y as i32 + t as i32)
                .of_size(new_width, new_height);
            drawing::draw_hollow_rect_mut(image, rect, rgb_color);
        }
    }

    /// Ring of `thickness` dots inside the `width` x `height` box; filled when the thickness
    /// reaches half the smaller side. Also used for circles.
    #[allow(clippy::too_many_arguments)]
    fn render_ellipse(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor) {
        if width == 0 || height == 0 {
            return;
        }
        let rgb_color = self.line_color_to_rgb(color);
        let (a, b) = (width as f64 / 2.0, height as f64 / 2.0);
        let (cx, cy) = (x as f64 + a, y as f64 + b);
        let t = thickness as f64;
        let (ia, ib) = (a - t, b - t);
        let filled = ia <= 0.0 || ib <= 0.0;

        for py in y..(y + height).min(image.height()) {
            let dy = py as f64 + 0.5 - cy;
            for px in x..(x + width).min(image.width()) {
                let dx = px as f64 + 0.5 - cx;
                let outer = (dx / a).powi(2) + (dy / b).powi(2) <= 1.0;
                if !outer {
                    continue;
                }
                let inner = !filled && (dx / ia).powi(2) + (dy / ib).powi(2) < 1.0;
                if !inner {
                    image.put_pixel(px, py, rgb_color);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_diagonal_line(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor, direction: &DiagonalDirection) {
        let rgb_color = self.line_color_to_rgb(color);

        let (x1, y1, x2, y2) = match direction {
            DiagonalDirection::LeftToRight => (x as f32, y as f32, (x + width) as f32, (y + height) as f32),
            DiagonalDirection::RightToLeft => ((x + width) as f32, y as f32, x as f32, (y + height) as f32),
        };

        for t in 0..thickness {
            let offset = t as f32;
            drawing::draw_line_segment_mut(
                image,
                (x1 + offset, y1 + offset),
                (x2 + offset, y2 + offset),
                rgb_color,
            );
        }
    }

    /// 1-bit bitmap, MSB first; each source dot becomes `mag_x` x `mag_y` pixels.
    #[allow(clippy::too_many_arguments)]
    fn render_graphic_field(&self, image: &mut RgbImage, x: u32, y: u32, bytes_per_row: u32, data: &[u8], mag_x: u32, mag_y: u32) {
        let (mag_x, mag_y) = (mag_x.max(1), mag_y.max(1));
        let width = bytes_per_row * 8;
        let height = (data.len() as u32).checked_div(bytes_per_row).unwrap_or(0);

        if width == 0 || height == 0 {
            return;
        }

        let mut temp_image: RgbImage = ImageBuffer::from_pixel(width * mag_x, height * mag_y, WHITE);

        for (row, row_bytes) in data.chunks(bytes_per_row as usize).take(height as usize).enumerate() {
            for (byte_offset, byte) in row_bytes.iter().enumerate() {
                for bit in 0..8 {
                    if byte & (1 << (7 - bit)) != 0 {
                        let src_x = (byte_offset * 8 + bit) as u32;
                        for py in row as u32 * mag_y..(row as u32 + 1) * mag_y {
                            for px in src_x * mag_x..(src_x + 1) * mag_x {
                                temp_image.put_pixel(px, py, BLACK);
                            }
                        }
                    }
                }
            }
        }

        if self.debug {
            self.draw_debug_box(image, x, y, temp_image.width(), temp_image.height(), Rgb([0, 0, 255]));
        }

        self.overlay_image(image, &temp_image, x, y);
    }

    fn line_color_to_rgb(&self, color: &LineColor) -> Rgb<u8> {
        match color {
            LineColor::Black => BLACK,
            LineColor::White => WHITE,
        }
    }

    fn draw_origin_cross(&self, image: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
        let size = 10;
        // Draw horizontal line
        for dx in 0..size {
            let px = x.saturating_sub(size / 2) + dx;
            if px < image.width() && y < image.height() {
                image.put_pixel(px, y, color);
            }
        }
        // Draw vertical line
        for dy in 0..size {
            let py = y.saturating_sub(size / 2) + dy;
            if x < image.width() && py < image.height() {
                image.put_pixel(x, py, color);
            }
        }
    }

    fn draw_debug_box(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, color: Rgb<u8>) {
        let rect = imageproc::rect::Rect::at(x as i32, y as i32).of_size(width, height);
        drawing::draw_hollow_rect_mut(image, rect, color);
    }
}

pub fn save_png(image: &RgbImage, path: &str) -> Result<(), image::ImageError> {
    image.save(path)
}

/// Encodes `image` in memory in the format implied by `path`'s extension, like
/// [`save_png`] does, without writing it to disk.
pub fn encode_image(image: &RgbImage, path: &str) -> Result<Vec<u8>, image::ImageError> {
    encode_as(image, image::ImageFormat::from_path(path)?)
}

/// Encodes `image` as PNG in memory.
pub fn encode_png(image: &RgbImage) -> Result<Vec<u8>, image::ImageError> {
    encode_as(image, image::ImageFormat::Png)
}

fn encode_as(image: &RgbImage, format: image::ImageFormat) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut bytes), format)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn black(image: &RgbImage, x: u32, y: u32) -> bool {
        image.get_pixel(x, y)[0] < 128
    }

    #[test]
    fn reverse_inverts_underlying_pixels() {
        let solid = DrawInstruction::DrawBox {
            x: 0, y: 0, width: 10, height: 10, thickness: 10, color: LineColor::Black, rounding: 0,
        };
        let reversed = DrawInstruction::Reverse(Box::new(DrawInstruction::DrawBox {
            x: 5, y: 0, width: 10, height: 10, thickness: 10, color: LineColor::Black, rounding: 0,
        }));
        let image = PngRenderer::new(20, 10).render(&[solid, reversed]);
        assert!(black(&image, 2, 5));
        assert!(!black(&image, 7, 5));
        assert!(black(&image, 12, 5));
        assert!(!black(&image, 17, 5));
    }

    #[test]
    fn ellipse_ring_and_fill() {
        let ring = DrawInstruction::DrawEllipse {
            x: 0, y: 0, width: 40, height: 20, thickness: 2, color: LineColor::Black,
        };
        let image = PngRenderer::new(40, 20).render(&[ring]);
        assert!(black(&image, 0, 10));
        assert!(!black(&image, 20, 10));
        let filled = DrawInstruction::DrawEllipse {
            x: 0, y: 0, width: 40, height: 20, thickness: 10, color: LineColor::Black,
        };
        let image = PngRenderer::new(40, 20).render(&[filled]);
        assert!(black(&image, 20, 10));
    }

    #[test]
    fn graphic_field_magnification() {
        let gf = DrawInstruction::DrawGraphicField {
            x: 0, y: 0, bytes_per_row: 1, data: vec![0x80], magnification_x: 3, magnification_y: 2,
        };
        let image = PngRenderer::new(10, 10).render(&[gf]);
        assert!(black(&image, 2, 1));
        assert!(!black(&image, 3, 1));
        assert!(!black(&image, 2, 2));
    }

    #[test]
    fn linear_ean13_draws_bars() {
        let ean = DrawInstruction::DrawLinear {
            x: 20, y: 0, kind: LinearKind::Ean13, data: "590123412345".into(),
            orientation: Orientation::Normal, height: 50, module_width: 2, wide_bar_ratio: 3.0,
            print_interpretation_line: true, print_interpretation_line_above: false,
        };
        let image = PngRenderer::new(300, 100).render(&[ean]);
        // Start guard: bar at module 0, space at module 1.
        assert!(black(&image, 20, 10));
        assert!(!black(&image, 22, 10));
        // Leading digit sits left of the bars.
        assert!((0..20).any(|x| (50..80).any(|y| black(&image, x, y))));
    }

    #[test]
    fn wrap_respects_width_and_forced_breaks() {
        let font = TextFont::new(30, 30);
        let lines = wrap_lines(&font, "aaa bbb ccc\\&ddd", font.measure("aaa bbb") as u32 + 1, 0);
        assert_eq!(lines, vec!["aaa bbb", "ccc", "ddd"]);
    }
}
