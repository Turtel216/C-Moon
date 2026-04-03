//! Constant propagation optimization pass

use std::collections::HashMap;

use super::ir::{BasicBlock, Opcode, Operand};

/// Propagates constants locally within the basic block.
/// Returns true if any operands were replaced.
pub fn propagate_constants(block: &mut BasicBlock) -> bool {
    let mut changed = false;
    // Tracks variables with known constant integer values
    let mut known_constants: HashMap<Operand, i64> = HashMap::new();

    for instr in &mut block.instructions {
        // Replace arguments if they are known constants
        if let Some(arg1) = &instr.arg1 {
            if let Some(&val) = known_constants.get(arg1) {
                instr.arg1 = Some(Operand::ImmInt(val));
                changed = true;
            }
        }
        if let Some(arg2) = &instr.arg2 {
            if let Some(&val) = known_constants.get(arg2) {
                instr.arg2 = Some(Operand::ImmInt(val));
                changed = true;
            }
        }

        // Update the known constants map based on the destination
        if let Some(dest) = &instr.dest {
            // If it's a direct move of an immediate, record it!
            if instr.opcode == Opcode::Mov {
                if let Some(Operand::ImmInt(val)) = &instr.arg1 {
                    known_constants.insert(dest.clone(), *val);
                    continue;
                }
            }

            // Otherwise, the destination is being overwritten with an UNKNOWN value.
            // Invalidate any previously known constant for this destination.
            known_constants.remove(dest);
        }
    }

    changed
}
