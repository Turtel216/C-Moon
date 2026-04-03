use frontend::lexer::Lexer;
use frontend::parser::Parser;
use frontend::renamer::resolve_names;
use frontend::semantic::SemanticAnalyzer;
use middle::constfold::constant_folding_pass;
use middle::desuger::LoweringContext;
use printer::ast_printer::AstPrinter;
use printer::ir_printer::IrPrinter;

mod backend;
mod driver;
mod frontend;
mod middle;
mod printer;

fn main() {
    driver::run();
}
