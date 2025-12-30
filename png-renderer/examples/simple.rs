use png_renderer::PngRenderer;
use zpl_state_manager::DrawInstruction;
use zpl_parser::command::{FontName, Orientation, LineColor};

fn main() {
    // Create some simple draw instructions manually
    let instructions = vec![
        DrawInstruction::DrawBox {
            x: 100,
            y: 100,
            width: 200,
            height: 200,
            thickness: 3,
            color: LineColor::Black,
            rounding: 0,
        },
        DrawInstruction::DrawText {
            x: 120,
            y: 120,
            text: "Hello ZPL".to_string(),
            font: FontName::FontA,
            height: 20,
            width: 10,
            orientation: Orientation::Normal,
        },
        DrawInstruction::DrawCircle {
            x: 400,
            y: 400,
            diameter: 100,
            thickness: 2,
            color: LineColor::Black,
        },
    ];

    // Create renderer with standard 4x6 label size at 203 DPI
    let renderer = PngRenderer::new(812, 1218);

    // Render to image
    let image = renderer.render(&instructions);

    // Save to file
    match png_renderer::save_png(&image, "simple_example.png") {
        Ok(_) => println!("PNG saved to simple_example.png"),
        Err(e) => eprintln!("Error saving PNG: {:?}", e),
    }
}
