const std = @import("std");
const token = @import("token.zig");
const Token = token.Token;
const mem = std.mem;
const testing = std.testing;

//TODO fix lexer. Tests failing

pub const Lexer = struct {
    input: []const u8,
    position: usize = 0,
    read_position: usize = 0,
    ch: u8 = 0,

    pub fn init(input: []const u8) Lexer {
        var lexer = Lexer{ .input = input };
        lexer.read_char();
        return lexer;
    }

    fn read_char(self: *Lexer) void {
        if (self.read_position >= self.input.len) {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(self: *Lexer) u8 {
        if (self.read_position >= self.input.len) {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn skip_whitespace(self: *Lexer) void {
        while (self.ch == ' ' or self.ch == '\t' or self.ch == '\n' or self.ch == '\r') {
            self.read_char();
        }
    }

    fn read_identifier(self: *Lexer) []const u8 {
        const start = self.position;
        while (std.ascii.isAlphabetic(self.ch) or self.ch == '_') {
            self.read_char();
        }
        return self.input[start..self.position];
    }

    fn read_number(self: *Lexer) isize {
        const start = self.position;
        while (std.ascii.isDigit(self.ch)) {
            self.read_char();
        }
        const num_str = self.input[start..self.position];
        return std.fmt.parseInt(isize, num_str, 10) catch unreachable;
    }

    fn read_string(self: *Lexer) []const u8 {
        self.read_char(); // Skip opening quote
        const start = self.position;
        while (self.ch != '"' and self.ch != 0) {
            self.read_char();
        }
        const str = self.input[start..self.position];
        self.read_char(); // Skip closing quote
        return str;
    }

    pub fn next_token(self: *Lexer) Token {
        self.skip_whitespace();

        const tok: Token = switch (self.ch) {
            '=' => blk: {
                if (self.peek_char() == '=') {
                    self.read_char();
                    break :blk Token{ .EqualEqual = {} };
                } else {
                    break :blk Token{ .Equal = {} };
                }
            },
            '+' => Token{ .Plus = {} },
            '-' => Token{ .Minus = {} },
            '*' => Token{ .Star = {} },
            '/' => Token{ .Slash = {} },
            '!' => blk: {
                if (self.peek_char() == '=') {
                    self.read_char();
                    break :blk Token{ .Neq = {} };
                } else {
                    break :blk Token{ .Bang = {} };
                }
            },
            '<' => Token{ .Lt = {} },
            '>' => Token{ .Gt = {} },
            ',' => Token{ .Comma = {} },
            ';' => Token{ .Semicolon = {} },
            '(' => Token{ .Lparen = {} },
            ')' => Token{ .Rparen = {} },
            '{' => Token{ .Lbrace = {} },
            '}' => Token{ .Rbrace = {} },
            '"' => Token{ .String = self.read_string() },
            0 => Token{ .Eof = {} },
            else => {
                if (std.ascii.isAlphabetic(self.ch) or self.ch == '_') {
                    const ident = self.read_identifier();
                    return switch (ident[0]) {
                        'i' => if (mem.eql(u8, ident, "if")) Token{ .If = {} } else Token{ .Identifier = ident },
                        'e' => if (mem.eql(u8, ident, "else")) Token{ .Else = {} } else Token{ .Identifier = ident },
                        'r' => if (mem.eql(u8, ident, "return")) Token{ .Return = {} } else Token{ .Identifier = ident },
                        'v' => if (mem.eql(u8, ident, "void")) Token{ .Void = {} } else Token{ .Identifier = ident },
                        else => Token{ .Identifier = ident },
                    };
                } else if (std.ascii.isDigit(self.ch)) {
                    return Token{ .Int = self.read_number() };
                } else {
                    return Token{ .Illegal = {} };
                }
            },
        };

        self.read_char();
        return tok;
    }
};

// Unit Tests
test "lexer" {
    const input =
        \\if (x == 10) {
        \\  return 42;
        \\} else {
        \\  return 22;
        \\}
    ;

    var lexer = Lexer.init(input);
    const expected_tokens = [_]Token{
        Token{ .If = {} },
        Token{ .Lparen = {} },
        Token{ .Identifier = "x" },
        Token{ .EqualEqual = {} },
        Token{ .Int = 10 },
        Token{ .Rparen = {} },
        Token{ .Lbrace = {} },
        Token{ .Return = {} },
        Token{ .Int = 42 },
        Token{ .Semicolon = {} },
        Token{ .Rbrace = {} },
        Token{ .Else = {} },
        Token{ .Lbrace = {} },
        Token{ .Return = {} },
        Token{ .String = "hello" },
        Token{ .Semicolon = {} },
        Token{ .Rbrace = {} },
        Token{ .Eof = {} },
    };

    for (expected_tokens) |expected| {
        const tok = lexer.next_token();
        testing.expectEqual(expected, tok);
    }
}
