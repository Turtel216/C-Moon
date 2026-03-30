use frontend::lexer::Lexer;
use frontend::parser::Parser;
use frontend::semantic::SemanticAnalyzer;
use printer::ast_printer::AstPrinter;

mod frontend;
mod middle;
mod printer;

fn main() {
    let lexer = Lexer::new("int main() { int x = 0 << 1; return x + 1; } ");
    let mut parser = Parser::from_lexer(lexer).expect("lexing failed");
    let tu = parser.parse_translation_unit().expect("parse failed");
    let mut sem = SemanticAnalyzer::new();
    sem.analyze_program(&tu).expect("semantic analysis failed");

    let mut output = String::new();
    let mut ast_printer = AstPrinter::new();

    for decl in tu {
        let _ = ast_printer.print_decl(&decl, &mut output);
        println!("{}", output);
    }
}
