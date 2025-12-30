use std::env;
use std::io::{self, Read};
use zpl_parser::Parser;
use zpl_state_manager::StateManager;
use png_renderer::PngRenderer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_file = args.get(1).map(|s| s.as_str()).unwrap_or("output.png");
    let debug = args.iter().any(|arg| arg == "--debug" || arg == "-d");

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let parser = Parser::new(&input);

    let commands = match parser.parse() {
        Ok(cmds) => cmds,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            std::process::exit(1);
        }
    };

    let mut state_manager = StateManager::new();
    let draw_instructions = state_manager.process(commands);

    let (width, height) = state_manager.get_label_dimensions();

    let renderer = if debug {
        PngRenderer::with_debug(width, height, true)
    } else {
        PngRenderer::new(width, height)
    };
    let image = renderer.render(&draw_instructions);

    match png_renderer::save_png(&image, output_file) {
        Ok(_) => println!("PNG saved to: {}", output_file),
        Err(e) => {
            eprintln!("Failed to save PNG: {:?}", e);
            std::process::exit(1);
        }
    }
}
