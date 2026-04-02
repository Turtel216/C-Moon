//! x86-64 data structures.
//!
//! Every register, operand, and instruction is represented by a Rust enum so
//! that mis-encodings are caught at compile time.  Raw strings are only produced
//! in the final `Display` / emission phase.

use std::fmt;

//
// ### Registers ###
//

/// All 16 general-purpose 64-bit x86-64 registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum X86Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl X86Register {
    /// The 8-bit "low byte" name used for `setCC` instructions.
    pub fn low_byte_name(self) -> &'static str {
        match self {
            Self::Rax => "al",
            Self::Rbx => "bl",
            Self::Rcx => "cl",
            Self::Rdx => "dl",
            Self::Rsi => "sil",
            Self::Rdi => "dil",
            Self::Rbp => "bpl",
            Self::Rsp => "spl",
            Self::R8 => "r8b",
            Self::R9 => "r9b",
            Self::R10 => "r10b",
            Self::R11 => "r11b",
            Self::R12 => "r12b",
            Self::R13 => "r13b",
            Self::R14 => "r14b",
            Self::R15 => "r15b",
        }
    }

    /// The 32-bit "double-word" name (used for `movzx` with 32-bit dest zero-extending).
    pub fn dword_name(self) -> &'static str {
        match self {
            Self::Rax => "eax",
            Self::Rbx => "ebx",
            Self::Rcx => "ecx",
            Self::Rdx => "edx",
            Self::Rsi => "esi",
            Self::Rdi => "edi",
            Self::Rbp => "ebp",
            Self::Rsp => "esp",
            Self::R8 => "r8d",
            Self::R9 => "r9d",
            Self::R10 => "r10d",
            Self::R11 => "r11d",
            Self::R12 => "r12d",
            Self::R13 => "r13d",
            Self::R14 => "r14d",
            Self::R15 => "r15d",
        }
    }
}

impl fmt::Display for X86Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Rax => "rax",
            Self::Rbx => "rbx",
            Self::Rcx => "rcx",
            Self::Rdx => "rdx",
            Self::Rsi => "rsi",
            Self::Rdi => "rdi",
            Self::Rbp => "rbp",
            Self::Rsp => "rsp",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        };
        write!(f, "{}", name)
    }
}

// ### Condition codes  (for Jcc / SETcc) ###

/// Condition codes used by conditional jumps and `setCC` instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionCode {
    /// Equal  (ZF=1)
    E,
    /// Not equal  (ZF=0)
    Ne,
    /// Less  (SF≠OF)  — signed
    L,
    /// Less or equal  (ZF=1 ∨ SF≠OF)
    Le,
    /// Greater  (ZF=0 ∧ SF=OF)
    G,
    /// Greater or equal  (SF=OF)
    Ge,
}

impl fmt::Display for ConditionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::E => "e",
            Self::Ne => "ne",
            Self::L => "l",
            Self::Le => "le",
            Self::G => "g",
            Self::Ge => "ge",
        };
        write!(f, "{}", s)
    }
}

// ### Storage location  (output of register allocation) ###

/// Where a virtual register ended up after allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageLocation {
    /// Allocated to a physical register.
    Register(X86Register),
    /// Spilled to a stack slot at `[rbp - offset]`.
    Stack(i32),
}

// ### Operands ###

/// An operand that can appear in an x86-64 instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X86Operand {
    /// A physical register, e.g. `rax`.
    Reg(X86Register),
    /// A memory reference `[base + disp]`.  Displacement may be negative.
    Mem(X86Register, i32),
    /// An immediate integer value.
    Imm(i64),
    /// A label reference (for `call` / `jmp` / `lea`).
    Label(String),
}

// ### Instructions ###

/// A single x86-64 instruction, strongly typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X86Instruction {
    /// `mov dst, src`
    Mov(X86Operand, X86Operand),
    /// `add dst, src`  — dst += src
    Add(X86Operand, X86Operand),
    /// `sub dst, src`  — dst -= src
    Sub(X86Operand, X86Operand),
    /// `imul dst, src` — signed multiply dst *= src
    Imul(X86Operand, X86Operand),
    /// `cqo` — sign-extend RAX into RDX:RAX
    Cqo,
    /// `idiv src` — signed divide RDX:RAX by src
    Idiv(X86Operand),
    /// `cmp lhs, rhs` — set flags from lhs - rhs
    Cmp(X86Operand, X86Operand),
    /// `test lhs, rhs` — set flags from lhs & rhs
    Test(X86Operand, X86Operand),
    /// `setCC dst_byte` — set byte register from condition
    SetCC(ConditionCode, X86Operand),
    /// `movzx dst, src_byte` — zero-extend byte to qword
    Movzx(X86Operand, X86Operand),
    /// `xor dst, src`
    Xor(X86Operand, X86Operand),
    /// `neg dst` — two's complement negate
    Neg(X86Operand),
    /// `push src`
    Push(X86Operand),
    /// `pop dst`
    Pop(X86Operand),
    /// `jmp label`
    Jmp(String),
    /// `jCC label`
    Jcc(ConditionCode, String),
    /// `call label`
    Call(String),
    /// `ret`
    Ret,
    /// A label definition, e.g. `.L1:`
    Label(String),
    /// `lea dst, [rip + label]`
    Lea(X86Operand, X86Operand),
    /// A comment line for debugging, e.g. `; some text`
    Comment(String),
}

// ### Function / Program containers ###

/// A single compiled function.
#[derive(Debug, Clone)]
pub struct X86Function {
    pub name: String,
    pub instructions: Vec<X86Instruction>,
}

/// A complete compiled program (collection of functions).
#[derive(Debug, Clone)]
pub struct X86Program {
    pub functions: Vec<X86Function>,
}
