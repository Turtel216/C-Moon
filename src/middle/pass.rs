//! Chained optimization passes

use super::{
    algsimpl::simplify_algebraic, constfold::fold_constants, constprop::propagate_constants,
    ir::CFG,
};

pub fn run_local_optimizations(cfg: &mut CFG) -> bool {
    let mut changed_any = false;
    let mut loop_changed = true;

    // Keep running the trinity of passes until no more changes are made
    while loop_changed {
        loop_changed = false;

        for block in cfg.blocks.values_mut() {
            // Fold constants (e.g. t1 = 5 + 5 -> t1 = 10)
            for instr in &mut block.instructions {
                loop_changed |= fold_constants(instr);
                loop_changed |= simplify_algebraic(instr);
            }

            // Propagate constants (e.g. replace t1 with 10 downstream)
            loop_changed |= propagate_constants(block);

            // Eliminate dead code (e.g. remove t1 = 10 if t1 is no longer used)
            //loop_changed |= eliminate_dead_code_local(block);
        }

        changed_any |= loop_changed;
    }
    changed_any
}
