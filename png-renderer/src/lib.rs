use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use ab_glyph::{FontRef, PxScale};
use zpl_parser::command::{LineColor, Orientation, DiagonalDirection, Justification};
use zpl_state_manager::DrawInstruction;

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
                    self.draw_origin_cross(image, *x, *y, Rgb([255, 0, 0])); // Red cross for text origin
                }
                self.render_text(image, *x, *y, text, *height, *width, orientation, justification);
            }
            DrawInstruction::DrawBarcode { x, y, data, orientation, height, module_width, .. } => {
                self.render_code128(image, *x, *y, data, orientation, *height, *module_width);
            }
            DrawInstruction::DrawCode39 { x, y, data, orientation, height, module_width, .. } => {
                self.render_code39(image, *x, *y, data, *orientation, *height, *module_width);
            }
            DrawInstruction::DrawQrCode { x, y, data, magnification, .. } => {
                self.render_qr_code(image, *x, *y, data, *magnification);
            }
            DrawInstruction::DrawBox { x, y, width, height, thickness, color, rounding } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 255, 0])); // Green cross for box origin
                }
                self.render_box(image, *x, *y, *width, *height, *thickness, color, *rounding);
            }
            DrawInstruction::DrawCircle { x, y, diameter, thickness, color } => {
                self.render_circle(image, *x, *y, *diameter, *thickness, color);
            }
            DrawInstruction::DrawDiagonalLine { x, y, width, height, thickness, color, direction } => {
                self.render_diagonal_line(image, *x, *y, *width, *height, *thickness, color, direction);
            }
            DrawInstruction::DrawGraphicField { x, y, bytes_per_row, data, .. } => {
                if self.debug {
                    self.draw_origin_cross(image, *x, *y, Rgb([0, 0, 255])); // Blue cross for graphic field origin
                }
                self.render_graphic_field(image, *x, *y, *bytes_per_row, data);
            }
        }
    }

    fn render_text(&self, image: &mut RgbImage, x: u32, y: u32, text: &str, height: u32, _width: u32, orientation: &Orientation, justification: &Justification) {
        // Load the embedded Roboto Mono font
        let font_data = include_bytes!("../fonts/RobotoMono-VariableFont_wght.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Error loading font");

        let scale = PxScale::from(height as f32);
        let color = Rgb([0, 0, 0]);

        match orientation {
            Orientation::Normal => {
                // Normal text (horizontal, left to right)
                let text_width = self.estimate_text_width(&font, scale, text);
                let adjusted_x = match justification {
                    Justification::Left => x,
                    Justification::Right => x.saturating_sub(text_width),
                    Justification::Auto => x.saturating_sub(text_width / 2),
                };
                drawing::draw_text_mut(image, color, adjusted_x as i32, y as i32, scale, &font, text);
            }
            Orientation::Rotated90 => {
                // Rotated 90 degrees clockwise (vertical, bottom to top)
                // Create a temporary image for the text
                let text_width = self.estimate_text_width(&font, scale, text);
                let text_height = height * 2; // Give extra space for descenders

                if text_width > 0 && text_height > 0 {
                    let mut temp_image: RgbImage = ImageBuffer::from_pixel(text_width, text_height, Rgb([255, 255, 255]));
                    drawing::draw_text_mut(&mut temp_image, color, 0, 0, scale, &font, text);

                    // Rotate 90 degrees and draw onto main image
                    let rotated = image::imageops::rotate90(&temp_image);

                    // Justification affects X axis even for rotated text (relative to field origin)
                    let adjusted_x = match justification {
                        Justification::Left => x,
                        Justification::Right => x.saturating_sub(rotated.width()),
                        Justification::Auto => x.saturating_sub(rotated.width() / 2),
                    };
                    self.overlay_image(image, &rotated, adjusted_x, y);
                }
            }
            Orientation::Rotated180 => {
                // Rotated 180 degrees (upside down)
                let text_width = self.estimate_text_width(&font, scale, text);
                let text_height = height * 2;

                if text_width > 0 && text_height > 0 {
                    let mut temp_image: RgbImage = ImageBuffer::from_pixel(text_width, text_height, Rgb([255, 255, 255]));
                    drawing::draw_text_mut(&mut temp_image, color, 0, 0, scale, &font, text);

                    // Rotate 180 degrees and draw onto main image
                    let rotated = image::imageops::rotate180(&temp_image);

                    // Apply justification (flipped for 180° rotation)
                    let adjusted_x = match justification {
                        Justification::Left => x,
                        Justification::Right => x.saturating_sub(text_width),
                        Justification::Auto => x.saturating_sub(text_width / 2),
                    };
                    self.overlay_image(image, &rotated, adjusted_x, y);
                }
            }
            Orientation::Rotated270 => {
                // Rotated 270 degrees clockwise (vertical, bottom to top)
                let text_width = self.estimate_text_width(&font, scale, text);
                let text_height = height + 10; // Use font height plus small padding

                if text_width > 0 && text_height > 0 {
                    let mut temp_image: RgbImage = ImageBuffer::from_pixel(text_width, text_height, Rgb([255, 255, 255]));
                    drawing::draw_text_mut(&mut temp_image, color, 0, 0, scale, &font, text);

                    // Rotate 270 degrees and draw onto main image
                    let rotated = image::imageops::rotate270(&temp_image);

                    // For Rotated270, origin should be at bottom-right of the rendered text
                    // Text extends upward (negative Y) and based on justification
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

        width.ceil() as u32 + 10 // Add some padding
    }

    fn overlay_image(&self, target: &mut RgbImage, source: &RgbImage, x: u32, y: u32) {
        for (src_x, src_y, pixel) in source.enumerate_pixels() {
            let target_x = x + src_x;
            let target_y = y + src_y;

            if target_x < target.width() && target_y < target.height() {
                // Only copy non-white pixels (transparent background)
                if pixel[0] < 250 || pixel[1] < 250 || pixel[2] < 250 {
                    target.put_pixel(target_x, target_y, *pixel);
                }
            }
        }
    }

    fn render_code128(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, orientation: &Orientation, height: u32, module_width: u32) {
        // Simple barcode rendering - just draw vertical lines
        // In a real implementation, you would use a barcode library to encode the data
        let bar_width = module_width.max(1);
        let num_bars = data.len() * 11; // Rough approximation
        let barcode_width = num_bars as u32 * bar_width;

        // Create barcode on temporary image
        let mut temp_image: RgbImage = ImageBuffer::from_pixel(barcode_width, height, Rgb([255, 255, 255]));

        for i in 0..num_bars {
            if i % 2 == 0 {
                for w in 0..bar_width {
                    let offset = (i as u32 * bar_width) + w;
                    for h in 0..height {
                        if offset < barcode_width {
                            temp_image.put_pixel(offset, h, Rgb([0, 0, 0]));
                        }
                    }
                }
            }
        }

        // Rotate and position based on orientation
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
                // 270° rotation (bottom to top)
                let rotated = image::imageops::rotate270(&temp_image);
                self.overlay_image(image, &rotated, x, y);
            }
        }
    }

    fn render_code39(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, orientation: Orientation, height: u32, module_width: u32) {
        // Simple barcode rendering similar to Code128
        let bar_width = module_width.max(1);
        let num_bars = data.len() * 13; // Code39 has wider encoding
        let barcode_width = num_bars as u32 * bar_width;

        // Create barcode on temporary image
        let mut temp_image: RgbImage = ImageBuffer::from_pixel(barcode_width, height, Rgb([255, 255, 255]));

        for i in 0..num_bars {
            if i % 3 != 0 {
                for w in 0..bar_width {
                    let offset = (i as u32 * bar_width) + w;
                    for h in 0..height {
                        if offset < barcode_width {
                            temp_image.put_pixel(offset, h, Rgb([0, 0, 0]));
                        }
                    }
                }
            }
        }

        // Rotate and position based on orientation
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
                // 270° rotation (bottom to top)
                let rotated = image::imageops::rotate270(&temp_image);
                self.overlay_image(image, &rotated, x, y);
            }
        }
    }

    fn render_qr_code(&self, image: &mut RgbImage, x: u32, y: u32, data: &str, magnification: u32) {
        use qrcode::QrCode;

        if let Ok(code) = QrCode::new(data.as_bytes()) {
            let qr_image = code.render::<Rgb<u8>>()
                .min_dimensions(magnification, magnification)
                .build();

            // Copy QR code onto the main image
            for (qr_x, qr_y, pixel) in qr_image.enumerate_pixels() {
                let target_x = x + qr_x;
                let target_y = y + qr_y;

                if target_x < image.width() && target_y < image.height() {
                    image.put_pixel(target_x, target_y, *pixel);
                }
            }
        }
    }

    fn render_box(&self, image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: &LineColor, rounding: u32) {
        let rgb_color = self.line_color_to_rgb(color);

        if rounding == 0 {
            // Draw rectangle with thickness
            for t in 0..thickness {
                let rect = imageproc::rect::Rect::at(x as i32 + t as i32, y as i32 + t as i32)
                    .of_size(width.saturating_sub(2 * t), height.saturating_sub(2 * t));
                drawing::draw_hollow_rect_mut(image, rect, rgb_color);
            }
        } else {
            // For rounded corners, draw a simple rectangle for now
            // A full implementation would draw rounded corners
            for t in 0..thickness {
                let rect = imageproc::rect::Rect::at(x as i32 + t as i32, y as i32 + t as i32)
                    .of_size(width.saturating_sub(2 * t), height.saturating_sub(2 * t));
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
        // Calculate dimensions
        let width = bytes_per_row * 8;
        let height = if bytes_per_row > 0 {
            (data.len() as u32) / bytes_per_row
        } else {
            0
        };

        if width == 0 || height == 0 {
            return;
        }

        // Create temporary image to draw the bitmap
        let mut temp_image: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));

        // Render bitmap data to temp image
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

        // Draw debug bounding box if debug mode is enabled
        if self.debug {
            self.draw_debug_box(image, x, y, temp_image.width(), temp_image.height(), Rgb([0, 0, 255]));
        }

        // Overlay image onto main image (no rotation)
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
        // Draw a thin rectangle outline
        let rect = imageproc::rect::Rect::at(x as i32, y as i32).of_size(width, height);
        drawing::draw_hollow_rect_mut(image, rect, color);
    }
}

pub fn save_png(image: &RgbImage, path: &str) -> Result<(), image::ImageError> {
    image.save(path)
}
