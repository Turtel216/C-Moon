use lexer::Lexer;
use parser::Parser;
use printer::ast_printer::AstPrinter;

mod ast;
mod lexer;
mod parser;
mod printer;

fn main() {
    let lexer = Lexer::new("int main() { return 0 + 1; } ");
    let mut parser = Parser::from_lexer(lexer).expect("lexing failed");
    let tu = parser.parse_translation_unit().expect("parse failed");

    let mut output = String::new();
    let mut ast_printer = AstPrinter::new();

    for decl in tu {
        let _ = ast_printer.print_decl(&decl, &mut output);
        println!("{}", output);
    }
}
