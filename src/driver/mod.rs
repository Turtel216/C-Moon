use std::{fs, process::Command};

use cli::get_arguments;

use crate::{
    backend,
    frontend::{lexer::Lexer, parser::Parser, renamer::resolve_names, semantic::SemanticAnalyzer},
    middle::desuger::LoweringContext,
    printer::{ast_printer::AstPrinter, ir_printer::IrPrinter},
};

mod cli;

/// Assambly output
const ASM_OUTPUT: &str = "asm.s";

// TODO: Add proper error reporting for parser and semantantic analysis errors

pub fn run() -> () {
    // Get command line arguments
    let cli = get_arguments();

    // Read source file
    // TODO: Dont crash the program when file not. Print proper message
    let source_program = fs::read_to_string(cli.source_file).expect("File not found");

    // Tokenize program
    let lexer = Lexer::new(&source_program);

    // Parse Program
    let mut parser = Parser::from_lexer(lexer).expect("lexing failed");
    let ast = parser.parse_translation_unit().expect("parse failed");

    // Semantic analysis
    let mut sem = SemanticAnalyzer::new();
    sem.analyze_program(&ast).expect("semantic analysis failed");

    // Identifier Renaming
    let resolution_map = resolve_names(&ast).expect("Name resolution failed");

    // Desuger AST into IR
    let ctx = LoweringContext::new(&resolution_map);
    let mut ir = ctx.lower_program(&ast);

    // Optimization passes if optimizations are enabled
    if cli.opt {
        ir.optimize();
    }

    let mut output = String::new();
    // Print AST to console
    if cli.printast {
        let mut ast_printer = AstPrinter::new();

        println!("=== AST ===");
        for decl in &ast {
            let _ = ast_printer.print_decl(decl, &mut output);
        }

        println!("{}", output);
        output.clear();
    }

    // Print IR to console
    if cli.printir {
        let _ = IrPrinter::print_ir(&ir, &mut output);
        println!("{}", output);
    }

    // Emit x86 assembly
    let asm = backend::pipeline::compile_program(&ir);

    // Output assembly to file
    let out_path = std::path::Path::new(ASM_OUTPUT);
    let asm_program = backend::emit::emit_asm(&asm);
    fs::write(out_path, asm_program).expect("Failed to write assembly to file");

    assamble_program(&cli.output_file);

    if !cli.asm {
        clean_up();
    }
}

/// Produce executable through GCC
fn assamble_program(output_path: &str) -> () {
    let _ = Command::new("gcc")
        .args(["-no-pie", "-o", output_path, ASM_OUTPUT])
        .output()
        .expect("Failed to link program");
}

fn clean_up() -> () {
    let _ = Command::new("rm")
        .arg(ASM_OUTPUT)
        .output()
        .expect("Failed to delete intermediete files");
}
