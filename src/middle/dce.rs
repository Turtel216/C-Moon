use crate::middle::desuger::ProgramIr;
use crate::middle::ir::{CFG, Opcode, Operand};
use std::collections::{HashMap, HashSet};

// Function to get variables from an operand
fn extract_var(op: &Operand) -> Option<Operand> {
    match op {
        Operand::Var(_) | Operand::Temp(_) => Some(op.clone()),
        _ => None,
    }
}

// Check if an instruction has side effects
fn has_side_effects(opcode: &Opcode) -> bool {
    matches!(
        opcode,
        Opcode::Jump
            | Opcode::BranchIf
            | Opcode::BranchIfNot
            | Opcode::Call
            | Opcode::Param
            | Opcode::Ret
    )
}

pub fn eliminate_dead_code(cfg: &mut CFG) -> bool {
    let mut changed = false;

    // 1. Unreachable Code Elimination (Control-Flow)
    // Perform reachability sweep starting from the CFG `entry` block
    let mut reachable = HashSet::new();
    let mut worklist = vec![cfg.entry.clone()];

    while let Some(block_label) = worklist.pop() {
        if reachable.insert(block_label.clone()) {
            if let Some(block) = cfg.blocks.get(&block_label) {
                for succ in &block.successors {
                    if !reachable.contains(succ) {
                        worklist.push(succ.clone());
                    }
                }
            }
        }
    }

    // Maintain a list to remove out-of-graph unreachable blocks
    let initial_len = cfg.blocks.len();
    cfg.blocks.retain(|label, _| reachable.contains(label));
    if initial_len != cfg.blocks.len() {
        changed = true;
    }

    // Clean up predecessors in remaining reachable blocks
    for block in cfg.blocks.values_mut() {
        let orig_len = block.predecessors.len();
        block.predecessors.retain(|p| reachable.contains(p));
        if orig_len != block.predecessors.len() {
            changed = true;
        }
    }

    // 2. Global Data-Flow Analysis (Liveness)
    // We implement a standard iterative data-flow algorithm to compute the `IN` and `OUT`
    // liveness sets for every BasicBlock.

    let mut use_sets: HashMap<String, HashSet<Operand>> = HashMap::new();
    let mut def_sets: HashMap<String, HashSet<Operand>> = HashMap::new();
    let mut in_sets: HashMap<String, HashSet<Operand>> = HashMap::new();
    let mut out_sets: HashMap<String, HashSet<Operand>> = HashMap::new();

    for (label, block) in &cfg.blocks {
        let mut b_use = HashSet::new();
        let mut b_def = HashSet::new();

        for instr in &block.instructions {
            // Determine `USE` within block before definitions
            let uses = vec![&instr.arg1, &instr.arg2]
                .into_iter()
                .filter_map(|arg| arg.as_ref().and_then(extract_var));

            for u in uses {
                if !b_def.contains(&u) {
                    b_use.insert(u);
                }
            }

            // Determine unconditional `DEF` within block
            if let Some(d) = instr.dest.as_ref().and_then(extract_var) {
                b_def.insert(d);
            }
        }

        use_sets.insert(label.clone(), b_use);
        def_sets.insert(label.clone(), b_def);
        in_sets.insert(label.clone(), HashSet::new());
        out_sets.insert(label.clone(), HashSet::new());
    }

    let mut dataflow_changed = true;
    while dataflow_changed {
        dataflow_changed = false;

        for label in cfg.blocks.keys().cloned().collect::<Vec<_>>() {
            let mut new_out = HashSet::new();
            if let Some(block) = cfg.blocks.get(&label) {
                // OUT[B] = Union over S in Successors(B) of IN[S]
                for succ in &block.successors {
                    if let Some(succ_in) = in_sets.get(succ) {
                        new_out.extend(succ_in.iter().cloned());
                    }
                }
            }

            if out_sets.get(&label) != Some(&new_out) {
                out_sets.insert(label.clone(), new_out.clone());
                dataflow_changed = true;
            }

            let mut new_in = use_sets.get(&label).unwrap().clone();
            // IN[B] = USE[B] U (OUT[B] - DEF[B])
            for d in new_out.difference(def_sets.get(&label).unwrap()) {
                new_in.insert(d.clone());
            }

            if in_sets.get(&label) != Some(&new_in) {
                in_sets.insert(label.clone(), new_in);
                dataflow_changed = true;
            }
        }
    }

    // 3. Local Dead Code Elimination
    // Backward sweeping local DCE pass within each block using the computed `OUT` sets
    for (label, block) in cfg.blocks.iter_mut() {
        let mut live = out_sets.get(label).unwrap().clone();
        let mut new_instructions = Vec::with_capacity(block.instructions.len());

        for instr in block.instructions.iter().rev() {
            let dest_var = instr.dest.as_ref().and_then(extract_var);

            let is_dead = dest_var.as_ref().map_or(false, |d| !live.contains(d));
            let has_side_effects = has_side_effects(&instr.opcode);

            // Instructions with side effects must never be eliminated
            if is_dead && !has_side_effects {
                changed = true;
                continue; // Eliminate the instruction
            }

            // If kept, update the live set.
            // 1. Remove defined destination
            if let Some(d) = dest_var {
                live.remove(&d);
            }

            // 2. Add used arguments
            if let Some(u) = instr.arg1.as_ref().and_then(extract_var) {
                live.insert(u);
            }
            if let Some(u) = instr.arg2.as_ref().and_then(extract_var) {
                live.insert(u);
            }

            new_instructions.push(instr.clone());
        }

        // Restore the original order for the basic block
        new_instructions.reverse();
        block.instructions = new_instructions;
    }

    changed
}
