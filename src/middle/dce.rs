//! Dead code elimination optimization pass

use std::collections::HashSet;

use crate::middle::ir::Operand;

use super::ir::{BasicBlock, Opcode};

/// Eliminates dead instructions within the basic block.
/// Returns true if any instructions were removed.
pub fn eliminate_dead_code_local(block: &mut BasicBlock) -> bool {
    let mut live_vars: HashSet<Operand> = HashSet::new();
    let mut new_instructions = Vec::new();
    let original_len = block.instructions.len();

    for instr in block.instructions.drain(..).rev() {
        let mut is_dead = false;

        if let Some(dest) = &instr.dest {
            // Only attempt to eliminate if the destination is a Temporary.
            // We assume Var() escapes the block and must be preserved.
            let is_temporary = matches!(dest, Operand::Temp(_));

            if is_temporary && !live_vars.contains(dest) && !has_side_effects(&instr.opcode) {
                is_dead = true;
            } else {
                live_vars.remove(dest);
            }
        }

        if !is_dead {
            if let Some(arg1) = &instr.arg1 {
                if matches!(arg1, Operand::Var(_) | Operand::Temp(_)) {
                    live_vars.insert(arg1.clone());
                }
            }
            if let Some(arg2) = &instr.arg2 {
                if matches!(arg2, Operand::Var(_) | Operand::Temp(_)) {
                    live_vars.insert(arg2.clone());
                }
            }
            new_instructions.push(instr);
        }
    }

    new_instructions.reverse();
    block.instructions = new_instructions;
    block.instructions.len() < original_len
}

/// Helper function
/// Cannot delete instructions with side effects,
/// even if their destination is unused (e.g., `call` might write to a memory address).
fn has_side_effects(opcode: &Opcode) -> bool {
    matches!(
        opcode,
        Opcode::Call
            | Opcode::Param
            | Opcode::Ret
            | Opcode::Jump
            | Opcode::BranchIf
            | Opcode::BranchIfNot
    )
}
