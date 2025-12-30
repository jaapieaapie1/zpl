use std::io::{self, Read};
use zpl_parser::Parser;
use zpl_state_manager::StateManager;

fn main() {
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

    for instruction in draw_instructions {
        println!("{:#?}", instruction);
    }
}
