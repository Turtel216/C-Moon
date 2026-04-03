//! Assembly Emission
//!
//! `Display` implementations that produce Intel-syntax x86-64 assembly text.
//! A complete `.s` file can be obtained by printing an `X86Program`.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use crate::backend::x86::*;

// ### X86Operand ###

impl fmt::Display for X86Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X86Operand::Reg(r) => write!(f, "{}", r),
            X86Operand::Mem(base, disp) => {
                if *disp == 0 {
                    write!(f, "QWORD PTR [{}]", base)
                } else if *disp > 0 {
                    write!(f, "QWORD PTR [{} + {}]", base, disp)
                } else {
                    write!(f, "QWORD PTR [{} - {}]", base, -disp)
                }
            }
            X86Operand::Imm(v) => write!(f, "{}", v),
            X86Operand::Label(l) => write!(f, "{}", l),
        }
    }
}

// ### X86Instruction ###

impl fmt::Display for X86Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X86Instruction::Mov(dst, src) => write!(f, "    mov {}, {}", dst, src),
            X86Instruction::Add(dst, src) => write!(f, "    add {}, {}", dst, src),
            X86Instruction::Sub(dst, src) => write!(f, "    sub {}, {}", dst, src),
            X86Instruction::Imul(dst, src) => write!(f, "    imul {}, {}", dst, src),
            X86Instruction::Cqo => write!(f, "    cqo"),
            X86Instruction::Idiv(src) => write!(f, "    idiv {}", src),

            X86Instruction::Cmp(a, b) => write!(f, "    cmp {}, {}", a, b),
            X86Instruction::Test(a, b) => write!(f, "    test {}, {}", a, b),

            X86Instruction::SetCC(cc, dst) => {
                // SetCC writes to the low byte of the register.
                let byte_name = match dst {
                    X86Operand::Reg(r) => r.low_byte_name(),
                    _ => panic!("SetCC destination must be a register"),
                };
                write!(f, "    set{} {}", cc, byte_name)
            }
            X86Instruction::Movzx(dst, src) => {
                // movzx with 32-bit destination zero-extends to 64-bit implicitly.
                let dst_dword = match dst {
                    X86Operand::Reg(r) => r.dword_name(),
                    _ => panic!("Movzx destination must be a register"),
                };
                let src_byte = match src {
                    X86Operand::Reg(r) => r.low_byte_name(),
                    _ => panic!("Movzx source must be a register"),
                };
                write!(f, "    movzx {}, {}", dst_dword, src_byte)
            }

            X86Instruction::Xor(a, b) => write!(f, "    xor {}, {}", a, b),
            X86Instruction::Neg(dst) => write!(f, "    neg {}", dst),

            X86Instruction::Push(src) => write!(f, "    push {}", src),
            X86Instruction::Pop(dst) => write!(f, "    pop {}", dst),

            X86Instruction::Jmp(label) => write!(f, "    jmp {}", label),
            X86Instruction::Jcc(cc, label) => write!(f, "    j{} {}", cc, label),
            X86Instruction::Call(label) => write!(f, "    call {}", label),
            X86Instruction::Ret => write!(f, "    ret"),

            X86Instruction::Label(label) => write!(f, "{}:", label),

            X86Instruction::Lea(dst, src) => write!(f, "    lea {}, {}", dst, src),

            X86Instruction::Comment(text) => write!(f, "    # {}", text),
        }
    }
}

// ### X86Function ###

impl fmt::Display for X86Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".globl {}", self.name)?;
        writeln!(f, ".type {}, @function", self.name)?;
        writeln!(f, "{}:", self.name)?;
        for instr in &self.instructions {
            writeln!(f, "{}", instr)?;
        }
        Ok(())
    }
}

// ### X86Program ###

impl fmt::Display for X86Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".intel_syntax noprefix")?;
        writeln!(f, ".section .text")?;
        writeln!(f)?;
        for func in &self.functions {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}

// ### File output ###

/// Write the complete assembly program to a file.
pub fn emit_to_file(program: &X86Program, path: &Path) -> io::Result<()> {
    let content = format!("{}", program);
    fs::write(path, content)
}
