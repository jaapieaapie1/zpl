# PNG Renderer

A PNG renderer for ZPL (Zebra Programming Language) that converts ZPL draw instructions into PNG images.

## Overview

The PNG renderer takes `DrawInstruction` objects from the `zpl-state-manager` and renders them to PNG images. Each x and y coordinate in the draw instructions is treated as a pixel position.

## Features

- **Text Rendering**: Renders text as simple rectangles (placeholder implementation)
- **Barcodes**: Simple barcode rendering for Code128 and Code39
- **QR Codes**: Full QR code rendering using the `qrcode` crate
- **Graphic Primitives**:
  - Boxes (with optional rounding)
  - Circles
  - Diagonal lines
- **Graphic Fields**: Raw bitmap data rendering

## Usage

```rust
use png_renderer::PngRenderer;
use zpl_state_manager::StateManager;
use zpl_parser::Parser;

// Parse ZPL
let parser = Parser::new(zpl_string);
let commands = parser.parse()?;

// Process commands through state manager
let mut state_manager = StateManager::new();
let draw_instructions = state_manager.process(commands);

// Render to PNG
let renderer = PngRenderer::new(812, 1218); // 4" x 6" at 203 DPI
let image = renderer.render(&draw_instructions);

// Save to file
png_renderer::save_png(&image, "output.png")?;
```

## Default Label Dimensions

The default label size is based on standard Zebra printer settings:
- Width: 4 inches × 203 DPI = 812 pixels
- Height: 6 inches × 203 DPI = 1218 pixels

## Implementation Notes

### Text Rendering
The current implementation uses simple rectangles to represent characters. For production use, you should integrate a proper font rendering library.

### Barcode Rendering
Barcodes (Code128 and Code39) are rendered as simple vertical line patterns. For production use, you should integrate a proper barcode encoding library.

### QR Codes
QR codes are fully implemented using the `qrcode` crate and render correctly.

### Coordinate System
- Origin (0, 0) is at the top-left corner
- X increases to the right
- Y increases downward
- Each unit represents one pixel

## Dependencies

- `image`: Core image manipulation
- `imageproc`: Drawing primitives
- `qrcode`: QR code generation
