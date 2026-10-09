use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use ab_glyph::{FontRef, PxScale};
use zpl_parser::command::{Code128Mode, LineColor, Orientation, DiagonalDirection, Justification};
use zpl_state_manager::DrawInstruction;

pub mod code128;

pub struct PngRenderer {
    width: u32,
    height: u32,
    debug: bool,
}

impl PngRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, debug: false }
    }

    pub fn with_debug(width: u32, height: u32, debug: bool) -> Self {
        Self { width, height, debug }
    }

    pub fn render(&self, instructions: &[DrawInstruction]) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::from_pixel(self.width, self.height, Rgb([255, 255, 255]));

        for instruction in instructions {
            self.render_instruction(&mut image, instruction);
        }

        image
    }

    fn render_instruction(&self, image: &mut RgbImage, instruction: &DrawInstruction) {
        match instruction {
            DrawInstruction::DrawText { x, y, text, height, width, orientation, justification, .. } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([255, 0, 0]));
                }
                self.render_text(image, *x, *y, text, *height, *width, orientation, justification);
            }
            DrawInstruction::DrawBarcode { x, y, data, orientation, height, module_width, print_interpretation_line, print_interpretation_line_above, mode, .. } => {
                self.render_code128(image, *x, *y, data, orientation, *height, *module_width, *print_interpretation_line, *print_interpretation_line_above, *mode);
            }
            DrawInstruction::DrawCode39 { x, y, data, orientation, height, module_width, .. } => {
                self.render_code39(image, *x, *y, data, *orientation, *height, *module_width);
            }
            DrawInstruction::DrawQrCode { x, y, data, magnification, .. } => {
                self.render_qr_code(image, *x, *y, data, *magnification);
            }
            DrawInstruction::DrawBox { x, y, width, height, thickness, color, rounding, reverse } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 255, 0]));
                }
                self.render_box(image, *x, *y, *width, *height, *thickness, color, *rounding, *reverse);
            }
            DrawInstruction::DrawCircle { x, y, diameter, thickness, color } => {
                self.render_circle(image, *x, *y, *diameter, *thickness, color);
            }
            DrawInstruction::DrawDiagonalLine { x, y, width, height, thickness, color, direction } => {
                self.render_diagonal_line(image, *x, *y, *width, *height, *thickness, color, direction);
            }
            DrawInstruction::DrawGraphicField { x, y, bytes_per_row, data, .. } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 0, 255]));
                }
                self.render_graphic_field(image, *x, *y, *bytes_per_row, data);
            }
        }
    }

    fn render_text(&self, image: &mut RgbImage, x: u32, y: u32, text: &str, height: u32, width: u32, orientation: &Orientation, justification: &Justification) {
        use ab_glyph::Font;

        let font_data = include_bytes!("../fonts/ATTriumvirate-BoldCd.otf");
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
        let color = Rgb([0, 0, 0]);

        match orientation {
            Orientation::Normal => {
                let text_width = self.estimate_text_width(&font, scale, text);
                let adjusted_x = match justification {
                    Justification::Left => x,
                    Justification::Right => x.saturating_sub(text_width),
                    Justification::Auto => x.saturating_sub(text_width / 2),
                };
                drawing::draw_text_mut(image, color, adjusted_x as i32, y as i32 - y_offset, scale, &font, text);
            }
            Orientation::Rotated90 => {
                let text_width = self.estimate_text_width(&font, scale, text);
                let text_height = height * 2;

                if text_width > 0 && text_height > 0 {
                    let mut temp_image: RgbImage = ImageBuffer::from_pixel(text_width, text_height, Rgb([255, 255, 255]));
                    drawing::draw_text_mut(&mut temp_image, color, 0, -y_offset, scale, &font, text);

                    let rotated = image::imageops::rotate90(&temp_image);

                    let adjusted_x = match justification {
                        Justification::Left => x,
                        Justification::Right => x.saturating_sub(rotated.width()),
                        Justification::Auto => x.saturating_sub(rotated.width() / 2),
                    };
                    self.overlay_image(image, &rotated, adjusted_x, y);
                }
            }
            Orientation::Rotated180 => {
                let text_width = self.estimate_text_width(&font, scale, text);
                let text_height = height * 2;

                if text_width > 0 && text_height > 0 {
                    let mut temp_image: RgbImage = ImageBuffer::from_pixel(text_width, text_height, Rgb([255, 255, 255]));
                    drawing::draw_text_mut(&mut temp_image, color, 0, -y_offset, scale, &font, text);

                    let rotated = image::imageops::rotate180(&temp_image);

                    let adjusted_x = match justification {
                        Justification::Left => x,
                        Justification::Right => x.saturating_sub(text_width),
                        Justification::Auto => x.saturating_sub(text_width / 2),
                    };
                    self.overlay_image(image, &rotated, adjusted_x, y);
                }
            }
            Orientation::Rotated270 => {
                let text_width = self.estimate_text_width(&font, scale, text);
                let text_height = height + 10;

                if text_width > 0 && text_height > 0 {
                    let mut temp_image: RgbImage = ImageBuffer::from_pixel(text_width, text_height, Rgb([255, 255, 255]));
                    drawing::draw_text_mut(&mut temp_image, color, 0, -y_offset, scale, &font, text);

                    let rotated = image::imageops::rotate270(&temp_image);

                    let adjusted_x = match justification {
                        Justification::Left => x.saturating_sub(rotated.width()),
                        Justification::Right => x,
                        Justification::Auto => x.saturating_sub(rotated.width() / 2),
                    };
                    let adjusted_y = y.saturating_sub(rotated.height());

                    if self.debug {
                        self.draw_debug_box(image, adjusted_x, adjusted_y, rotated.width(), rotated.height(), Rgb([255, 0, 0]));
                    }

                    self.overlay_image(image, &rotated, adjusted_x, adjusted_y);
                }
            }
        }
    }

    fn estimate_text_width(&self, font: &FontRef, scale: PxScale, text: &str) -> u32 {
        use ab_glyph::{Font, ScaleFont};

        let scaled_font = font.as_scaled(scale);

        let mut width = 0.0;
        for c in text.chars() {
            let glyph_id = font.glyph_id(c);
            width += scaled_font.h_advance(glyph_id);
        }

        width.ceil() as u32 + 10
    }

    fn overlay_image(&self, target: &mut RgbImage, source: &RgbImage, x: u32, y: u32) {
        for (src_x, src_y, pixel) in source.enumerate_pixels() {
            let target_x = x + src_x;
            let target_y = y + src_y;

            if target_x < target.width() && target_y < target.height() {
                if pixel[0] < 250 || pixel[1] < 250 || pixel[2] < 250 {
                    target.put_pixel(target_x, target_y, *pixel);
                }
            }
        }
    }

    fn render_code128(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, orientation: &Orientation, height: u32, module_width: u32, print_interpretation_line: bool, print_interpretation_line_above: bool, mode: Code128Mode) {
        use ab_glyph::{Font, ScaleFont};

        let code = code128::encode(data, mode);
        let bar_width = module_width.max(1);
        let widths = code.widths();
        let barcode_width: u32 = widths.iter().map(|&w| w as u32 * bar_width).sum();

        let font_data = include_bytes!("../fonts/DejaVuSansMono.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Error loading font");

        // Interpretation line: monospaced; pitch, digit height and gap scale with the module width (matches Labelary).
        let char_advance = bar_width as f32 * 143.0 / 24.0;
        let digit_height = (bar_width * 7) as f32;
        let zero = font.glyph_id('0');
        let zero_height_unscaled = font.outline(zero)
            .map(|o| (o.bounds.max.y - o.bounds.min.y).abs())
            .unwrap_or(font.ascent_unscaled());
        let scale = PxScale {
            x: char_advance * font.height_unscaled() / font.h_advance_unscaled(zero),
            y: digit_height * font.height_unscaled() / zero_height_unscaled,
        };
        let scaled_font = font.as_scaled(scale);
        let text_gap = (bar_width * 2) as f32;

        let show_text = print_interpretation_line && !code.text.is_empty();
        let text_width = if show_text { (char_advance * code.text.chars().count() as f32).ceil() as u32 } else { 0 };
        let text_area = if show_text { (text_gap + digit_height - scaled_font.descent()).ceil() as u32 } else { 0 };

        let pad = text_width.saturating_sub(barcode_width) / 2;
        let total_width = barcode_width + 2 * pad;
        let total_height = height + text_area;
        let (bars_y, baseline) = if print_interpretation_line_above {
            (text_area, digit_height)
        } else {
            (0, height as f32 + text_gap + digit_height)
        };

        let mut temp_image: RgbImage = ImageBuffer::from_pixel(total_width.max(1), total_height.max(1), Rgb([255, 255, 255]));

        let mut x_pos = pad;
        for (i, &width) in widths.iter().enumerate() {
            let pixel_width = width as u32 * bar_width;
            if i % 2 == 0 {
                for px in x_pos..x_pos + pixel_width {
                    for py in bars_y..bars_y + height {
                        temp_image.put_pixel(px, py, Rgb([0, 0, 0]));
                    }
                }
            }
            x_pos += pixel_width;
        }

        if show_text {
            let text_x = pad as f32 + (barcode_width as f32 - text_width as f32) / 2.0;
            let text_y = baseline - scaled_font.ascent();
            for (i, ch) in code.text.chars().enumerate() {
                let ch_x = text_x + i as f32 * char_advance;
                drawing::draw_text_mut(&mut temp_image, Rgb([0, 0, 0]), ch_x.round() as i32, text_y.round() as i32, scale, &font, &ch.to_string());
            }
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

    fn render_code39(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, orientation: Orientation, height: u32, module_width: u32) {
        use barcoders::sym::code39::Code39;

        let barcode = match Code39::new(data) {
            Ok(bc) => bc,
            Err(_) => return,
        };

        let encoded = barcode.encode();
        let bar_width = module_width.max(1);
        let barcode_width = encoded.len() * bar_width as usize;

        let mut temp_image: RgbImage = ImageBuffer::from_pixel(barcode_width as u32, height, Rgb([255, 255, 255]));

        for (i, &byte) in encoded.iter().enumerate() {
            if byte == 1 {
                for w in 0..bar_width {
                    let x_pos = (i * bar_width as usize) + w as usize;
                    if x_pos < barcode_width {
                        for h in 0..height {
                            temp_image.put_pixel(x_pos as u32, h, Rgb([0, 0, 0]));
                        }
                    }
                }
            }
        }

        match orientation {
            Orientation::Normal => {
                self.overlay_image(image, &temp_image, x, y);
            }
            Orientation::Rotated90 => {
                let rotated = image::imageops::rotate90(&temp_image);
                self.overlay_image(image, &rotated, x, y);
            }
            Orientation::Rotated180 => {
                let rotated = image::imageops::rotate180(&temp_image);
                self.overlay_image(image, &rotated, x, y);
            }
            Orientation::Rotated270 => {
                let rotated = image::imageops::rotate270(&temp_image);
                let adjusted_x = x.saturating_sub(rotated.width());
                let adjusted_y = y.saturating_sub(rotated.height());
                self.overlay_image(image, &rotated, adjusted_x, adjusted_y);
            }
        }
    }

    fn render_qr_code(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, magnification: u32) {
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

            // Zebra printers draw QR codes 10 dots below the field origin
            let y = y + 10;

            for (qr_x, qr_y, pixel) in qr_image.enumerate_pixels() {
                let target_x = x + qr_x;
                let target_y = y + qr_y;

                if target_x < image.width() && target_y < image.height() {
                    image.put_pixel(target_x, target_y, *pixel);
                }
            }
        }
    }

    fn render_box(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor, rounding: u32, reverse: bool) {
        if reverse {
            for py in y..(y + height).min(image.height()) {
                for px in x..(x + width).min(image.width()) {
                    let pixel = image.get_pixel(px, py);
                    let inverted = Rgb([255 - pixel[0], 255 - pixel[1], 255 - pixel[2]]);
                    image.put_pixel(px, py, inverted);
                }
            }
        } else {
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
    }

    fn render_circle(&self, image: &mut RgbImage, x: u32, y: u32, diameter: u32, thickness: u32, color: &LineColor) {
        let rgb_color = self.line_color_to_rgb(color);
        let radius = diameter / 2;
        let center_x = x + radius;
        let center_y = y + radius;

        for t in 0..thickness {
            let r = radius.saturating_sub(t);
            if r > 0 {
                drawing::draw_hollow_circle_mut(image, (center_x as i32, center_y as i32), r as i32, rgb_color);
            }
        }
    }

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

    fn render_graphic_field(&self, image: &mut RgbImage, x: u32, y: u32, bytes_per_row: u32, data: &[u8]) {
        let width = bytes_per_row * 8;
        let height = if bytes_per_row > 0 {
            (data.len() as u32) / bytes_per_row
        } else {
            0
        };

        if width == 0 || height == 0 {
            return;
        }

        let mut temp_image: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));

        let mut byte_idx = 0;
        let mut current_y = 0;

        while byte_idx < data.len() && current_y < height {
            let row_end = (byte_idx + bytes_per_row as usize).min(data.len());
            let row_bytes = &data[byte_idx..row_end];

            for (byte_offset, byte) in row_bytes.iter().enumerate() {
                for bit in 0..8 {
                    if byte & (1 << (7 - bit)) != 0 {
                        let pixel_x = (byte_offset * 8 + bit) as u32;
                        if pixel_x < width {
                            temp_image.put_pixel(pixel_x, current_y, Rgb([0, 0, 0]));
                        }
                    }
                }
            }

            byte_idx = row_end;
            current_y += 1;
        }

        if self.debug {
            self.draw_debug_box(image, x, y, temp_image.width(), temp_image.height(), Rgb([0, 0, 255]));
        }

        self.overlay_image(image, &temp_image, x, y);
    }

    fn draw_vertical_line(&self, image: &mut RgbImage, x: u32, y: u32, height: u32) {
        for dy in 0..height {
            let pixel_y = y + dy;
            if x < image.width() && pixel_y < image.height() {
                image.put_pixel(x, pixel_y, Rgb([0, 0, 0]));
            }
        }
    }

    fn line_color_to_rgb(&self, color: &LineColor) -> Rgb<u8> {
        match color {
            LineColor::Black => Rgb([0, 0, 0]),
            LineColor::White => Rgb([255, 255, 255]),
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
