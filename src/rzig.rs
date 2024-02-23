mod lexer;
mod parser;
mod codegen;
mod writer;

use crate::parser::Parser;
use crate::codegen::CodeGen;

pub fn main() {
	env_logger::init();
	let args: Vec<String> = std::env::args().collect();

	if args.len() != 2 {
        eprintln!("Usage: {} <input_file.zig>", args[0]);
        std::process::exit(1);
	}

    // read file
    let input_filename = &args[1];
    let source_code = std::fs::read_to_string(input_filename)
        .expect("Failed to read input file");

    // front end 
    let tokens = lexer::tokenize(&source_code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    // back end
	let codegen = CodeGen::new();
    let asm_code = codegen.generate(&ast);
	let _ = writer::write_to_file("tmp.s", &asm_code);
}
