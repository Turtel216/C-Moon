// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.
//
// Lexer for Sinners-C.
// This code tokenizes a source string into a series of tokens, allowing the parser to analyze it.
// Each token corresponds to a keyword, identifier, or symbol (like parentheses or braces).
// The `Tokenizer` struct holds the scanning logic, and `TokenType` represents the recognized tokens.

use std::fmt;

// Represents the different types of tokens that can be found in the source code.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum TokenType {
    IntKeyword,         // Represents the 'int' keyword.
    VoidKeyword,        // Represents the 'void' keyword.
    Identifier(String), // Any valid identifier (e.g., function/variable names).
    OpenParenthesis,    // Represents '('.
    CloseParenthesis,   // Represents ')'.
    OpenBrace,          // Represents '{'.
    CloseBrace,         // Represents '}'.
    Constant(isize),    // Represents numeric constants (e.g., 123).
    Semicolon,          // Represents ';'.
    ReturnKeyword,      // Represents the 'return' keyword.
}

// For debuging
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::IntKeyword => write!(f, "IntKeyword"),
            TokenType::VoidKeyword => write!(f, "VoidKeyword"),
            TokenType::Identifier(s) => write!(f, "Identifier '{}'", s),
            TokenType::Constant(v) => write!(f, "Constant '{}'", v),
            TokenType::OpenParenthesis => write!(f, "OpenParenthesis"),
            TokenType::CloseParenthesis => write!(f, "CloseParenthesis"),
            TokenType::OpenBrace => write!(f, "OpenBrace"),
            TokenType::CloseBrace => write!(f, "CloseBrace"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::ReturnKeyword => write!(f, "ReturnKeyword"),
        }
    }
}

// Struct for tokenizing source string
pub struct Tokenizer<'s> {
    tokens: Vec<TokenType>, // Vector holding generated tokens
    source: &'s str,        // Source string to be scanner
    start: usize,           // Start of current lexeme
    current: usize,         // Index of current character
    line: usize,
}

// Implementation of the Tokenizer struct, which scans a source string and tokenizes it.
impl<'s> Tokenizer<'s> {
    // Constructs a new Tokenizer with the provided source string.
    // Initializes an empty token list and scanning pointers (start, current, line).
    pub fn new(source: &'s str) -> Self {
        Self {
            tokens: Vec::new(),
            source,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    // Scans the entire source string and returns a reference to the list of tokens found.
    // This function loops through the source string, invoking `scan_token()` until the end.
    pub fn scan_source(&mut self) -> &Vec<TokenType> {
        // Tokenize the source string
        // and add each token to the token vector
        while !self.is_at_end() {
            self.scan_token();
        }

        return &self.tokens;
    }

    // Scans a portion of the source string and generates the appropriate token.
    // Adds the token to the tokens vector. Handles numeric constants, identifiers, and symbols.
    fn scan_token(&mut self) -> () {
        // Remove all white space
        self.skip_whitespace();
        // Set start of current lexeme
        self.start = self.current;

        match self.advance() {
            '(' => self.tokens.push(TokenType::OpenParenthesis),
            ')' => self.tokens.push(TokenType::CloseParenthesis),
            '{' => self.tokens.push(TokenType::OpenBrace),
            '}' => self.tokens.push(TokenType::CloseBrace),
            ';' => self.tokens.push(TokenType::Semicolon),
            c => {
                if c.is_numeric() {
                    self.lex_constant(); // Tokenize a number.
                    return;
                } else if c.is_alphabetic() {
                    self.lex_identifier(); // Tokenize an identifier or keyword.
                    return;
                }
                // Panic if an unrecognized character is encountered
                panic!(
                    "Lexer error: Could'nt recognise character on line {}",
                    self.line
                );
            }
        }
    }

    // Processes an identifier or keyword and appends its token type to the tokens vector.
    // Consumes alphanumeric characters and checks if the lexeme is a keyword.
    fn lex_identifier(&mut self) -> () {
        // Consume all alphanumeric characters
        while self.peek().is_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }

        // Create current lexeme
        let value: &str = self.source[self.start..self.current].into();

        // Check if the token is an identifier
        // or a keyword and add it to the vector
        self.tokens.push(self.match_keyword(value));
    }

    // Checks if the given lexeme matches a keyword; if not, it is treated as an identifier.
    fn match_keyword(&self, word: &str) -> TokenType {
        match word {
            "int" => TokenType::IntKeyword,
            "void" => TokenType::VoidKeyword,
            "return" => TokenType::ReturnKeyword,
            _ => TokenType::Identifier(self.get_lexeme().to_string()), //TODO get proper lexeme
        }
    }

    // Processes a numeric constant and appends it as a `TokenType::Constant`.
    fn lex_constant(&mut self) -> () {
        // Consume all numeric characters
        while self.peek().is_numeric() && !self.is_at_end() {
            self.advance();
        }

        let constant = self.get_constant();

        self.tokens.push(TokenType::Constant(constant)); //TODO get constant value
    }

    // Skips whitespace and comments, advancing the current character index appropriately.
    // Handles single-line comments starting with '//'.
    fn skip_whitespace(&mut self) -> () {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek_next() == '/' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    // Get the constant value from the source string.
    // This method converts the current lexeme (substring) into an integer.
    fn get_constant(&self) -> isize {
        // Get lexeme
        let constant = self.get_lexeme();

        // Attempt to parse the lexeme as an integer. If parsing fails, panic with an error message.
        match constant.parse() {
            Ok(v) => v,
            Err(e) => panic!("Error when trying to get consant from string: {}", e),
        }
    }

    // Get the lexeme (substring) between `start` and `current` indices.
    // This is used to retrieve the current identifier or constant from the source string.
    fn get_lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    // Returns the current character in the source string, or '\0' if at the end.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        // Get the character at the current index. Panic if the index is out of bounds.
        return self
            .source
            .chars()
            .nth(self.current)
            .unwrap_or_else(|| panic!("Error in peek(). No character at index {}", self.current));
    }

    // Returns the current character and advances the scanner to the next character.
    fn advance(&mut self) -> char {
        let char = self.peek(); // Get the current character
        self.current += 1; // Move to the next character
        char
    }

    // Returns the next character after the current one, or '\0' if at the end.
    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        // Get the next character after the current one, or panic if out of bounds.
        // This is a safeguard in case `is_at_end` fails to catch it.
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .unwrap_or_else(|| {
                panic!(
                    "Error in peek_next(). No character at index {}. Current character was {}",
                    self.current,
                    self.source.chars().nth(self.current).unwrap()
                )
            });
    }

    // Checks if the scanner has reached the end of the source string.
    fn is_at_end(&self) -> bool {
        return self.current == self.source.len();
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer;
    use crate::lexer::TokenType;

    // This test verifies that the lexer correctly tokenizes a simple function definition.
    #[test]
    fn test_lexer() {
        // Initialise lexer
        let mut scanner = lexer::Tokenizer::new("int main(void) { return 0;}");
        // Generate vector of TokenTypes
        let tokens = scanner.scan_source();

        // Vector holding the expected TokenTypes
        let mut expected_tokens = vec![
            TokenType::IntKeyword,
            TokenType::Identifier("main".to_string()),
            TokenType::OpenParenthesis,
            TokenType::VoidKeyword,
            TokenType::CloseParenthesis,
            TokenType::OpenBrace,
            TokenType::ReturnKeyword,
            TokenType::Constant(0),
            TokenType::Semicolon,
            TokenType::CloseBrace,
        ];

        // Assert that the number of tokens produced matches the expected count.
        assert!(tokens.len() == expected_tokens.len());

        let mut etoken_iter = expected_tokens.iter_mut(); // Expected Token Iterator

        // Compare each generated token with the expected tokens.
        for token in tokens {
            // Get expected token
            let etoken = match etoken_iter.next() {
                Some(v) => v,
                None => panic!("Couldn't get next token"),
            };

            // Assure that the token generated
            // by the lexer is equal to the expected token
            assert!(token == etoken);
        }
    }
}
