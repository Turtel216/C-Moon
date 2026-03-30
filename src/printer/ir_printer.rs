use crate::middle::lowering::*;
use core::fmt::Write;
use std::fmt;

pub struct IrPrinter;

impl IrPrinter {
    pub fn print_decl(&mut self, cfg: &CFG, w: &mut impl Write) -> fmt::Result {
        write!(w, "{}", cfg)
    }
}

// Operand Formatting
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Var(name) => write!(f, "{}", name),
            // Prefix temps with '%' to easily distinguish them from source variables
            Operand::Temp(name) => write!(f, "%{}", name),
            Operand::ImmInt(val) => write!(f, "{}", val),
            // Prefix labels with '.' (standard assembly convention)
            Operand::Label(name) => write!(f, ".{}", name),
        }
    }
}

// Opcode Formatting
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            Opcode::Add => "+",
            Opcode::Sub => "-",
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Eq => "==",
            Opcode::Neq => "!=",
            Opcode::Lt => "<",
            Opcode::Lte => "<=",
            Opcode::Gt => ">",
            Opcode::Gte => ">=",
            Opcode::Mov => "=",
            Opcode::Jump => "jmp",
            Opcode::BranchIf => "br_if",
            Opcode::BranchIfNot => "br_if_not",
        };
        write!(f, "{}", op_str)
    }
}

// TAC Instruction Formatting
impl fmt::Display for TACInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Helper closure to safely format optional operands,
        // printing "_" if an expected operand is missing due to a compiler bug.
        let format_op = |op: &Option<Operand>| -> String {
            op.as_ref()
                .map_or_else(|| "_".to_string(), |o| o.to_string())
        };

        match self.opcode {
            // Binary Operations
            Opcode::Add
            | Opcode::Sub
            | Opcode::Mul
            | Opcode::Div
            | Opcode::Eq
            | Opcode::Neq
            | Opcode::Lt
            | Opcode::Lte
            | Opcode::Gt
            | Opcode::Gte => {
                write!(
                    f,
                    "{} = {} {} {}",
                    format_op(&self.dest),
                    format_op(&self.arg1),
                    self.opcode,
                    format_op(&self.arg2)
                )
            }
            // Data Movement
            Opcode::Mov => {
                write!(f, "{} = {}", format_op(&self.dest), format_op(&self.arg1))
            }
            // Unary Control Flow
            Opcode::Jump => {
                write!(f, "jmp {}", format_op(&self.arg1))
            }
            // Binary Control Flow
            Opcode::BranchIf | Opcode::BranchIfNot => {
                write!(
                    f,
                    "{} {} goto {}",
                    self.opcode,
                    format_op(&self.arg1),
                    format_op(&self.arg2)
                )
            }
        }
    }
}

// Basic Block Formatting
impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".{}:", self.label)?;

        // Print edges as comments to make CFG debugging easier
        if !self.predecessors.is_empty() {
            writeln!(f, "    /* preds: {} */", self.predecessors.join(", "))?;
        }

        for instr in &self.instructions {
            writeln!(f, "    {}", instr)?;
        }

        if !self.successors.is_empty() {
            writeln!(f, "    /* succs: {} */", self.successors.join(", "))?;
        }

        Ok(())
    }
}

// CFG Formatting
impl fmt::Display for CFG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== CFG ===")?;
        writeln!(f, "Entry: .{}", self.entry)?;
        writeln!(f, "Exit:  .{}\n", self.exit)?;

        for block in self.blocks.values() {
            writeln!(f, "{}", block)?;
        }

        Ok(())
    }
}
