//! Constand folding optimization pass
//!
//! Implementation pass that performes constant folding on the CFG-TAC IR.

use crate::middle::ir::Operand;
use crate::middle::ir::{Opcode, TACInstruction, CFG};

use super::desuger::ProgramIr;

/// Attempts to fold constant operands in place.
/// Returns `true` if the instruction was successfully folded.
pub fn fold_constants(instr: &mut TACInstruction) -> bool {
    // We only care about instructions where both arguments are immediate integers.
    let (val1, val2) = match (&instr.arg1, &instr.arg2) {
        (Some(Operand::ImmInt(v1)), Some(Operand::ImmInt(v2))) => (*v1, *v2),
        _ => return false,
    };

    // Compute the folded value based on the opcode
    let folded_val = match instr.opcode {
        // Arithmetic
        // Wrapping operations to simulate standard x86 2's complement
        // arithmetic and avoid crashing the compiler on intentional/unintentional overflow.
        Opcode::Add => val1.wrapping_add(val2),
        Opcode::Sub => val1.wrapping_sub(val2),
        Opcode::Mul => val1.wrapping_mul(val2),
        Opcode::Div => {
            // Protect the compiler from a divide-by-zero panic during compilation.
            // TODO: Print warning of div by zero for user
            if val2 == 0 {
                return false;
            }
            val1.wrapping_div(val2)
        }

        // Relational (mapping true to 1 and false to 0 per your IR spec)
        Opcode::Eq => i64::from(val1 == val2),
        Opcode::Neq => i64::from(val1 != val2),
        Opcode::Lt => i64::from(val1 < val2),
        Opcode::Lte => i64::from(val1 <= val2),
        Opcode::Gt => i64::from(val1 > val2),
        Opcode::Gte => i64::from(val1 >= val2),

        // Opcodes that cannot be folded via binary immediate operations
        _ => return false,
    };

    // Transform the instruction into a standard Move of the new constant
    instr.opcode = Opcode::Mov;
    instr.arg1 = Some(Operand::ImmInt(folded_val));
    instr.arg2 = None;

    true
}

/// Constant folding optization pass operating on a CFG
pub fn constant_folding_cfg(cfg: &mut CFG) -> usize {
    let mut folded_count = 0;

    for block in cfg.blocks.values_mut() {
        for instr in &mut block.instructions {
            if fold_constants(instr) {
                folded_count += 1;
            }
        }
    }

    folded_count
}

/// Runs the constant folding optimization pass over all functions in the program.
/// Returns the total number of instructions that were folded across the entire program.
pub fn constant_folding_pass(program: &mut ProgramIr) -> usize {
    let mut total_folded = 0;

    // Iterate mutably over all CFGs in the BTreeMap
    for (_, cfg) in program.functions.iter_mut() {
        let folded_in_func = constant_folding_cfg(cfg);

        total_folded += folded_in_func;
    }

    total_folded
}
