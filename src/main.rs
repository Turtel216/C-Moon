use lexer::Lexer;

mod ast;
mod lexer;

fn main() {
    let mut scanner = Lexer::new("int main() { return 1 + 2; }");
}
