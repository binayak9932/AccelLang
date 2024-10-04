mod lexer;
mod parser;
mod semantic;
mod codegen;
mod cli;

use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use codegen::CodeGen;
use inkwell::context::Context;

fn main() {
    let input = "
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
    ";
    let lexer = Lexer::new(input);
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
    cli::run();
}
