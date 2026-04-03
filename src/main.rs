use frontend::lexer::Lexer;
use frontend::parser::Parser;
use frontend::renamer::resolve_names;
use frontend::semantic::SemanticAnalyzer;
use middle::constfold::constant_folding_pass;
use middle::desuger::LoweringContext;
use printer::ast_printer::AstPrinter;
use printer::ir_printer::IrPrinter;

mod backend;
mod frontend;
mod middle;
mod printer;

fn main() {
    // Tokenize program
    let lexer = Lexer::new(
        "
int main() {
 return 1 + 2;
}
",
    );
    // Parse program
    let mut parser = Parser::from_lexer(lexer).expect("lexing failed");
    let tu = parser.parse_translation_unit().expect("parse failed");

    // Semantic analysis
    let mut sem = SemanticAnalyzer::new();
    sem.analyze_program(&tu).expect("semantic analysis failed");

    // Identifier Renaming
    let resolution_map = resolve_names(&tu).expect("Name resolution failed");

    // Desuger AST into IR
    let ctx = LoweringContext::new(&resolution_map);
    let mut lowered_program = ctx.lower_program(&tu);

    // Constant folding
    let _ = constant_folding_pass(&mut lowered_program);

    let mut output = String::new();
    let mut ast_printer = AstPrinter::new();

    println!("=== AST ===");
    for decl in &tu {
        let _ = ast_printer.print_decl(decl, &mut output);
        println!("{}", output);
    }

    output.clear();
    let _ = IrPrinter::print_ir(&lowered_program, &mut output);
    println!("{}", output);

    // Backend: compile TAC/CFG => x86-64 assembly
    let x86_program = backend::pipeline::compile_program(&lowered_program);

    println!("=== x86-64 Assembly (Intel syntax) ===");
    println!("{}", x86_program);

    // Write assembly to file
    let out_path = std::path::Path::new("output.s");
    backend::emit::emit_to_file(&x86_program, out_path).expect("Failed to write output.s");
    println!("Assembly written to output.s");
    println!("Assemble with: gcc -no-pie -o output output.s");
}
