use std::env;
use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::codegen::CodeGen;
use inkwell::context::Context;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source file>", args[0]);
        return;
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Unable to read file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let function = parser.parse_function();
    println!("{:#?}", function);

    let mut analyzer = SemanticAnalyzer::new();
    match analyzer.analyze_function(&function) {
        Ok(_) => println!("Semantic analysis passed"),
        Err(err) => println!("Semantic analysis failed: {}", err),
    }

    let context = Context::create();
    let codegen = CodeGen::new(&context, "my_module");
    codegen.codegen_function(&function);

    codegen.module.print_to_stderr();
}
