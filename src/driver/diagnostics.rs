//! Compiler Diagnostics/Error messages

use crate::frontend::lexer::Span;

// TODO: Add Support for line snippet and fancy arrows etc.

/// Common behaviour of Compiler Errors for later reporting
pub trait CompilerError {
    fn get_span(&self) -> Span;
    fn get_message(&self) -> String;
    fn error_prefix(&self) -> String;
}

pub struct Diagnostics {
    comp_errors: Vec<Box<dyn CompilerError>>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics {
            comp_errors: Vec::new(),
        }
    }

    /// Report a compiler error
    pub fn report<T: CompilerError + 'static>(&mut self, comp_error: T) -> () {
        self.comp_errors.push(Box::new(comp_error));
    }

    /// Check if a Compiler has accured and if the compilation proccess should be stoped.
    pub fn panic(&self) -> bool {
        self.comp_errors.is_empty()
    }

    /// Print Compilation errors to stdout
    pub fn print(&self) -> () {
        let mut output: Vec<String> = Vec::new();
        for err in &self.comp_errors {
            let span = err.get_span();
            let message = format!(
                "{} {}:{} {}",
                err.error_prefix(),
                span.line,
                span.column,
                err.get_message()
            );

            output.push(message);
        }

        println!("{}", output.join("\n\n"));
    }
}
