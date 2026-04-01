use frontend::lexer::Lexer;
use frontend::parser::Parser;
use frontend::renamer::resolve_names;
use frontend::semantic::SemanticAnalyzer;
use middle::desuger::LoweringContext;
use printer::ast_printer::AstPrinter;
use printer::ir_printer::IrPrinter;

mod frontend;
mod middle;
mod printer;

fn main() {
    // Tokenize program
    let lexer = Lexer::new(
        "
int foo(int x, int y) {
 return x + 1;
}

int main() {
 int x = 1;
 int y = foo(x, 2);
 {
   int x = 5;
 }

 if (x < y) {
   return 0;
 }
 return 1;
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
    let lowered_program = ctx.lower_program(&tu);

    let mut output = String::new();
    let mut ast_printer = AstPrinter::new();

    println!("=== AST ===");
    for decl in tu {
        let _ = ast_printer.print_decl(&decl, &mut output);
        println!("{}", output);
    }

    output.clear();
    let _ = IrPrinter::print_ir(&lowered_program, &mut output);
    println!("{}", output);
}
