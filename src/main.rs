use frontend::lexer::Lexer;
use frontend::parser::Parser;
use frontend::semantic::SemanticAnalyzer;
use middle::lowering::LoweringContext;
use printer::ast_printer::AstPrinter;
use printer::ir_printer::IrPrinter;

mod frontend;
mod middle;
mod printer;

fn main() {
    let lexer = Lexer::new(
        "
int main() {
 int x = 2;
 int y = x;
 x = x + y;
 return x;
}

",
    );
    let mut parser = Parser::from_lexer(lexer).expect("lexing failed");
    let tu = parser.parse_translation_unit().expect("parse failed");
    let mut sem = SemanticAnalyzer::new();
    sem.analyze_program(&tu).expect("semantic analysis failed");
    let ctx = LoweringContext::new();
    let lowered_program = ctx.lower_program(&tu);

    let mut output = String::new();
    let mut ast_printer = AstPrinter::new();

    println!("AST:");
    for decl in tu {
        let _ = ast_printer.print_decl(&decl, &mut output);
        println!("{}", output);
    }

    println!("\nIR:");
    output.clear();
    let _ = IrPrinter::print_ir(&lowered_program, &mut output);
    println!("{}", output);
}
