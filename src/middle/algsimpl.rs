//! Algebraic simplification optmization pass

use super::ir::{Opcode, Operand, TACInstruction};

/// Attempts to apply algebraic identities to simplify the instruction in place.
/// Returns `true` if the instruction was successfully simplified.
pub fn simplify_algebraic(instr: &mut TACInstruction) -> bool {
    // check if an operand is a specific immediate integer
    let is_imm = |op: &Option<Operand>, val: i64| -> bool {
        matches!(op, Some(Operand::ImmInt(v)) if *v == val)
    };

    // check if two operands are exactly the same (e.g., %t1 and %t1)
    let are_equal = |op1: &Option<Operand>, op2: &Option<Operand>| -> bool {
        match (op1, op2) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    };

    let mut simplified = false;

    match instr.opcode {
        Opcode::Add => {
            if is_imm(&instr.arg1, 0) {
                // 0 + x -> Mov x
                instr.opcode = Opcode::Mov;
                instr.arg1 = instr.arg2.take();
                simplified = true;
            } else if is_imm(&instr.arg2, 0) {
                // x + 0 -> Mov x
                instr.opcode = Opcode::Mov;
                instr.arg2 = None;
                simplified = true;
            }
        }

        Opcode::Sub => {
            if is_imm(&instr.arg2, 0) {
                // x - 0 -> Mov x
                instr.opcode = Opcode::Mov;
                instr.arg2 = None;
                simplified = true;
            } else if are_equal(&instr.arg1, &instr.arg2) {
                // x - x -> Mov 0
                instr.opcode = Opcode::Mov;
                instr.arg1 = Some(Operand::ImmInt(0));
                instr.arg2 = None;
                simplified = true;
            }
        }

        Opcode::Mul => {
            if is_imm(&instr.arg1, 1) {
                // 1 * x -> Mov x
                instr.opcode = Opcode::Mov;
                instr.arg1 = instr.arg2.take();
                simplified = true;
            } else if is_imm(&instr.arg2, 1) {
                // x * 1 -> Mov x
                instr.opcode = Opcode::Mov;
                instr.arg2 = None;
                simplified = true;
            } else if is_imm(&instr.arg1, 0) || is_imm(&instr.arg2, 0) {
                // 0 * x OR x * 0 -> Mov 0
                instr.opcode = Opcode::Mov;
                instr.arg1 = Some(Operand::ImmInt(0));
                instr.arg2 = None;
                simplified = true;
            } else if is_imm(&instr.arg2, 2) {
                // Strength Reduction: x * 2 -> x + x
                // Addition is cheaper on CPU cycles than multiplication
                instr.opcode = Opcode::Add;
                instr.arg2 = instr.arg1.clone();
                simplified = true;
            } else if is_imm(&instr.arg1, 2) {
                // Strength Reduction: 2 * x -> x + x
                instr.opcode = Opcode::Add;
                instr.arg1 = instr.arg2.clone();
                // arg2 is already x, so x + x is achieved
                simplified = true;
            }
        }

        Opcode::Div => {
            if is_imm(&instr.arg2, 1) {
                // x / 1 -> Mov x
                instr.opcode = Opcode::Mov;
                instr.arg2 = None;
                simplified = true;
            } else if are_equal(&instr.arg1, &instr.arg2) {
                // x / x -> Mov 1
                instr.opcode = Opcode::Mov;
                instr.arg1 = Some(Operand::ImmInt(1));
                instr.arg2 = None;
                simplified = true;
            }
        }

        Opcode::Eq | Opcode::Lte | Opcode::Gte => {
            if are_equal(&instr.arg1, &instr.arg2) {
                // x == x, x <= x, x >= x -> Mov 1 (True)
                instr.opcode = Opcode::Mov;
                instr.arg1 = Some(Operand::ImmInt(1));
                instr.arg2 = None;
                simplified = true;
            }
        }
        Opcode::Neq | Opcode::Lt | Opcode::Gt => {
            if are_equal(&instr.arg1, &instr.arg2) {
                // x != x, x < x, x > x -> Mov 0 (False)
                instr.opcode = Opcode::Mov;
                instr.arg1 = Some(Operand::ImmInt(0));
                instr.arg2 = None;
                simplified = true;
            }
        }

        _ => {}
    }

    simplified
}
