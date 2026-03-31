use std::collections::BTreeMap;

use crate::frontend::ast::{
    BinaryOp, BlockItem, Decl, DeclKind, Expr, ExprKind, Literal, Stmt, StmtKind,
};

// ### TAC IR ###

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Var(String),
    Temp(String),
    ImmInt(i64),
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Relational / equality (result is 0/1)
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,

    // Data movement
    Mov, // dest = arg1

    // Control flow
    Jump,        // goto arg1(label)
    BranchIf,    // if arg1 != 0 goto arg2(label)
    BranchIfNot, // if arg1 == 0 goto arg2(label)

    // Function calls and returns
    Param, // pass arg1 as a parameter
    Call,  // dest = call arg1 (func label), arg2 (number of args)
    Ret,   // return arg1
}

#[derive(Debug, Clone, PartialEq)]
pub struct TACInstruction {
    pub opcode: Opcode,
    pub dest: Option<Operand>,
    pub arg1: Option<Operand>,
    pub arg2: Option<Operand>,
}

impl TACInstruction {
    pub fn new(
        opcode: Opcode,
        dest: Option<Operand>,
        arg1: Option<Operand>,
        arg2: Option<Operand>,
    ) -> Self {
        Self {
            opcode,
            dest,
            arg1,
            arg2,
        }
    }
}

// ### CFG ###

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<TACInstruction>,
    pub predecessors: Vec<String>,
    pub successors: Vec<String>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label,
            instructions: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn emit(&mut self, instr: TACInstruction) {
        self.instructions.push(instr);
    }
}

#[derive(Debug, Clone)]
pub struct CFG {
    pub entry: String,
    pub exit: String,
    pub blocks: BTreeMap<String, BasicBlock>,
}

impl CFG {
    pub fn new(entry: String, exit: String) -> Self {
        Self {
            entry,
            exit,
            blocks: BTreeMap::new(),
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.insert(block.label.clone(), block);
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if let Some(f) = self.blocks.get_mut(from) {
            if !f.successors.iter().any(|s| s == to) {
                f.successors.push(to.to_string());
            }
        }
        if let Some(t) = self.blocks.get_mut(to) {
            if !t.predecessors.iter().any(|p| p == from) {
                t.predecessors.push(from.to_string());
            }
        }
    }
}

// ### Lowering ###

#[derive(Debug, Clone)]
pub struct ProgramIr {
    pub functions: BTreeMap<String, CFG>,
}

impl ProgramIr {
    pub fn new() -> Self {
        Self {
            functions: BTreeMap::new(),
        }
    }
}

pub struct LoweringContext {
    pub program: ProgramIr,
    current_cfg: Option<CFG>,
    current_block: String,
    temp_counter: usize,
    label_counter: usize,
}

impl LoweringContext {
    pub fn new() -> Self {
        Self {
            program: ProgramIr::new(),
            current_cfg: None,
            current_block: String::new(),
            temp_counter: 0,
            label_counter: 0,
        }
    }

    pub fn lower_program(mut self, decls: &[Decl]) -> ProgramIr {
        for decl in decls {
            match &decl.kind {
                DeclKind::Function { name, body, .. } => {
                    let bod = body.clone().unwrap(); // TODO: Fix unsafe unwrap and clone
                    self.lower_function(name, &bod);
                }
                _ => { /* Handle global variables later */ }
            }
        }
        self.program
    }

    fn lower_function(&mut self, name: &str, body: &Stmt) {
        // 1. Setup a new CFG for this function
        let entry = format!("{}_entry", name);
        let exit = format!("{}_exit", name);

        let mut cfg = CFG::new(entry.clone(), exit.clone());
        cfg.add_block(BasicBlock::new(entry.clone()));
        cfg.add_block(BasicBlock::new(exit.clone()));

        self.current_cfg = Some(cfg);
        self.current_block = entry.clone();

        // 2. Lower the body
        self.lower_statement(body);

        // 3. Fallthrough to exit if the last block didn't explicitly return
        let cur = self.current_block.clone();
        if cur != exit {
            self.emit(TACInstruction::new(
                Opcode::Jump,
                None,
                Some(Operand::Label(exit.clone())),
                None,
            ));
            self.add_edge(&cur, &exit);
        }

        // 4. Save the finished CFG into the Program
        if let Some(finished_cfg) = self.current_cfg.take() {
            self.program
                .functions
                .insert(name.to_string(), finished_cfg);
        }
    }

    pub fn lower_stmt_tree(mut self, root: &Stmt) -> CFG {
        self.lower_statement(root);

        // fallthrough to exit if not already there
        // let cur = self.current_block.clone();
        // if cur != self.cfg.exit {
        //     self.emit(TACInstruction::new(
        //         Opcode::Jump,
        //         None,
        //         Some(Operand::Label(self.cfg.exit.clone())),
        //         None,
        //     ));
        //     let exit = self.cfg.exit.clone();
        //     self.cfg.add_edge(&cur, &exit);
        // }

        self.current_cfg.unwrap()
    }

    fn fresh_temp(&mut self) -> Operand {
        self.temp_counter += 1;
        Operand::Temp(format!("t{}", self.temp_counter))
    }

    fn fresh_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{}_L{}", prefix, self.label_counter)
    }

    fn create_block(&mut self, prefix: &str) -> String {
        let label = self.fresh_label(prefix);
        let cfg = self.current_cfg.as_mut().expect("Not inside a function!");
        cfg.add_block(BasicBlock::new(label.clone()));
        label
    }

    fn set_current_block(&mut self, label: String) {
        self.current_block = label;
    }

    fn emit(&mut self, instr: TACInstruction) {
        let cfg = self.current_cfg.as_mut().expect("Not inside a function!");
        let blk = cfg.blocks.get_mut(&self.current_block).unwrap();
        blk.emit(instr);
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let cfg = self.current_cfg.as_mut().expect("Not inside a function!");
        cfg.add_edge(from, to);
    }

    fn lower_statement(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                // Includes assignments represented as BinaryOp::Assign.
                let _ = self.lower_expression(expr);
            }

            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let then_label = self.create_block("if_then");
                let else_label = self.create_block("if_else");
                let end_label = self.create_block("if_end");

                let cond = self.lower_expression(condition);

                let cur = self.current_block.clone();
                // if !cond -> else
                self.emit(TACInstruction::new(
                    Opcode::BranchIfNot,
                    None,
                    Some(cond),
                    Some(Operand::Label(else_label.clone())),
                ));
                self.add_edge(&cur, &else_label);

                // otherwise -> then
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(then_label.clone())),
                    None,
                ));
                self.add_edge(&cur, &then_label);

                // then branch
                self.set_current_block(then_label.clone());
                self.lower_statement(then_branch);
                let then_end = self.current_block.clone();
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(end_label.clone())),
                    None,
                ));
                self.add_edge(&then_end, &end_label);

                // else branch (or empty else)
                self.set_current_block(else_label.clone());
                if let Some(e) = else_branch {
                    self.lower_statement(e);
                }
                let else_end = self.current_block.clone();
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(end_label.clone())),
                    None,
                ));
                self.add_edge(&else_end, &end_label);

                self.set_current_block(end_label);
            }

            StmtKind::While { condition, body } => {
                let cond_label = self.create_block("while_cond");
                let body_label = self.create_block("while_body");
                let end_label = self.create_block("while_end");

                // preheader -> cond
                let preheader = self.current_block.clone();
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(cond_label.clone())),
                    None,
                ));
                self.add_edge(&preheader, &cond_label);

                // cond block
                self.set_current_block(cond_label.clone());
                let cond = self.lower_expression(condition);

                // if !cond -> end
                self.emit(TACInstruction::new(
                    Opcode::BranchIfNot,
                    None,
                    Some(cond),
                    Some(Operand::Label(end_label.clone())),
                ));
                self.add_edge(&cond_label, &end_label);

                // else -> body
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(body_label.clone())),
                    None,
                ));
                self.add_edge(&cond_label, &body_label);

                // body block
                self.set_current_block(body_label.clone());
                self.lower_statement(body);

                // back-edge body -> cond
                let body_end = self.current_block.clone();
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(cond_label.clone())),
                    None,
                ));
                self.add_edge(&body_end, &cond_label);

                // continue after loop
                self.set_current_block(end_label);
            }

            StmtKind::Block(items) => {
                for item in items {
                    match item {
                        BlockItem::Stmt(s) => self.lower_statement(s),
                        BlockItem::Decl(d) => {
                            // Minimal handling for variable decl initializer in this phase.
                            if let DeclKind::Variable {
                                name,
                                initializer: Some(init),
                                ..
                            } = &d.kind
                            {
                                let rhs = self.lower_expression(init);
                                self.emit(TACInstruction::new(
                                    Opcode::Mov,
                                    Some(Operand::Var(name.clone())),
                                    Some(rhs),
                                    None,
                                ));
                            }
                        }
                    }
                }
            }

            StmtKind::Return(expr_opt) => {
                let ret_val = if let Some(expr) = expr_opt {
                    Some(self.lower_expression(expr))
                } else {
                    None
                };

                self.emit(TACInstruction::new(Opcode::Ret, None, ret_val, None));

                let exit_label = self.current_cfg.as_ref().unwrap().exit.clone();
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(exit_label.clone())),
                    None,
                ));

                let cur = self.current_block.clone();
                self.add_edge(&cur, &exit_label);

                let dead_block = self.create_block("unreachable_after_ret");
                self.set_current_block(dead_block);
            }

            // Not in current scope; no-op for now
            StmtKind::For { .. } => {}
        }
    }

    fn lower_expression(&mut self, expr: &Expr) -> Operand {
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => Operand::ImmInt(*v),

            ExprKind::Identifier(name) => Operand::Var(name.clone()),

            // x = rhs
            ExprKind::Binary(BinaryOp::Assign, lhs, rhs) => {
                let rhs_op = self.lower_expression(rhs);
                let lhs_var = self.expect_lvalue_var(lhs);
                self.emit(TACInstruction::new(
                    Opcode::Mov,
                    Some(Operand::Var(lhs_var.clone())),
                    Some(rhs_op.clone()),
                    None,
                ));
                Operand::Var(lhs_var)
            }

            // Arithmetic + comparisons used by if/while conditions.
            ExprKind::Binary(op, lhs, rhs) => {
                let l = self.lower_expression(lhs);
                let r = self.lower_expression(rhs);
                let t = self.fresh_temp();

                let opcode = match op {
                    BinaryOp::Add => Opcode::Add,
                    BinaryOp::Sub => Opcode::Sub,
                    BinaryOp::Mul => Opcode::Mul,
                    BinaryOp::Div => Opcode::Div,
                    BinaryOp::Eq => Opcode::Eq,
                    BinaryOp::Neq => Opcode::Neq,
                    BinaryOp::Lt => Opcode::Lt,
                    BinaryOp::Lte => Opcode::Lte,
                    BinaryOp::Gt => Opcode::Gt,
                    BinaryOp::Gte => Opcode::Gte,
                    _ => panic!("Binary op {:?} not supported in this lowering phase", op),
                };

                self.emit(TACInstruction::new(
                    opcode,
                    Some(t.clone()),
                    Some(l),
                    Some(r),
                ));
                t
            }

            ExprKind::Call { callee, args } => {
                // Lower arguments
                let mut arg_operands = Vec::with_capacity(args.len());
                for arg in args {
                    arg_operands.push(self.lower_expression(arg));
                }

                // Determine the target of the call.
                // If it's a direct identifier, its treated as a static Label.
                // Otherwise lower the expression is lowred (e.g., for function pointers).
                let callee_op = match &callee.kind {
                    ExprKind::Identifier(name) => Operand::Label(name.clone()),
                    _ => self.lower_expression(callee),
                };

                // Emit Param instructions
                for arg_op in arg_operands {
                    self.emit(TACInstruction::new(Opcode::Param, None, Some(arg_op), None));
                }

                // Emit the Call instruction
                let ret_temp = self.fresh_temp();
                self.emit(TACInstruction::new(
                    Opcode::Call,
                    Some(ret_temp.clone()),
                    Some(callee_op),
                    Some(Operand::ImmInt(args.len() as i64)),
                ));

                ret_temp
            }

            _ => panic!("Expr {:?} not supported in this lowering phase", expr.kind),
        }
    }

    fn expect_lvalue_var(&self, expr: &Expr) -> String {
        match &expr.kind {
            ExprKind::Identifier(name) => name.clone(),
            _ => panic!("Expected assignable identifier lvalue, got {:?}", expr.kind),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Note: Assuming your AST module uses `Box` for recursive types,
    // which is standard practice in Rust compilers.
    use crate::frontend::{
        ast::{
            BinaryOp, BlockItem, CType, Decl, DeclKind, Expr, ExprKind, Literal, Stmt, StmtKind,
        },
        lexer::Span,
    };

    // ###  AST Builder Helpers (Ergonomics) ###

    fn dummy_span() -> Span {
        Span {
            line: 0,
            column: 0,
            length: 0,
        }
    }

    fn int(v: i64) -> Expr {
        Expr {
            kind: ExprKind::Literal(Literal::Int(v)),
            span: dummy_span(),
        }
    }

    fn var(name: &str) -> Expr {
        Expr {
            kind: ExprKind::Identifier(name.to_string()),
            span: dummy_span(),
        }
    }

    fn binop(op: BinaryOp, lhs: Expr, rhs: Expr) -> Expr {
        Expr {
            kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
            span: dummy_span(),
        }
    }

    fn assign(lhs: Expr, rhs: Expr) -> Expr {
        Expr {
            kind: ExprKind::Binary(BinaryOp::Assign, Box::new(lhs), Box::new(rhs)),
            span: dummy_span(),
        }
    }

    fn stmt_expr(expr: Expr) -> Stmt {
        Stmt {
            kind: StmtKind::Expr(expr),
            span: dummy_span(),
        }
    }

    fn stmt_block(stmts: Vec<Stmt>) -> Stmt {
        let items = stmts.into_iter().map(BlockItem::Stmt).collect();
        Stmt {
            kind: StmtKind::Block(items),
            span: dummy_span(),
        }
    }

    // ### TAC/Operand Helpers ###

    fn op_var(name: &str) -> Option<Operand> {
        Some(Operand::Var(name.to_string()))
    }
    fn op_imm(v: i64) -> Option<Operand> {
        Some(Operand::ImmInt(v))
    }
    fn op_temp(name: &str) -> Option<Operand> {
        Some(Operand::Temp(name.to_string()))
    }
    fn op_lbl(name: &str) -> Option<Operand> {
        Some(Operand::Label(name.to_string()))
    }

    // ### Unit Tests ###

    #[test]
    fn test_lower_simple_assignment() {
        let mut ctx = LoweringContext::new();
        ctx.current_cfg = Some(CFG::new("entry".into(), "exit".into()));

        let entry_lbl = ctx.create_block("entry");
        ctx.set_current_block(entry_lbl.clone());

        // AST: x = 42
        let ast = stmt_expr(assign(var("x"), int(42)));
        ctx.lower_statement(&ast);

        let cfg = ctx.current_cfg.as_ref().unwrap();
        let block = &cfg.blocks[&entry_lbl];

        assert_eq!(block.instructions.len(), 1);
        let instr = &block.instructions[0];

        assert_eq!(instr.opcode, Opcode::Mov);
        assert_eq!(instr.dest, op_var("x"));
        assert_eq!(instr.arg1, op_imm(42));
        assert_eq!(instr.arg2, None);
    }

    #[test]
    fn test_lower_binary_arithmetic() {
        let mut ctx = LoweringContext::new();
        ctx.current_cfg = Some(CFG::new("entry".into(), "exit".into()));

        let entry_lbl = ctx.create_block("entry");
        ctx.set_current_block(entry_lbl.clone());

        // AST: a = b + 10
        let ast = stmt_expr(assign(var("a"), binop(BinaryOp::Add, var("b"), int(10))));
        ctx.lower_statement(&ast);

        let cfg = ctx.current_cfg.as_ref().unwrap();
        let block = &cfg.blocks[&entry_lbl];

        // Should generate:
        // t1 = b + 10
        // a = t1
        assert_eq!(block.instructions.len(), 2);

        let instr1 = &block.instructions[0];
        assert_eq!(instr1.opcode, Opcode::Add);
        assert_eq!(instr1.dest, op_temp("t1"));
        assert_eq!(instr1.arg1, op_var("b"));
        assert_eq!(instr1.arg2, op_imm(10));

        let instr2 = &block.instructions[1];
        assert_eq!(instr2.opcode, Opcode::Mov);
        assert_eq!(instr2.dest, op_var("a"));
        assert_eq!(instr2.arg1, op_temp("t1"));
    }

    #[test]
    fn test_lower_if_else_statement_cfg() {
        let mut ctx = LoweringContext::new();
        ctx.current_cfg = Some(CFG::new("entry".into(), "exit".into()));
        let entry_lbl = ctx.create_block("entry");
        ctx.set_current_block(entry_lbl.clone());

        // AST: if (x < 5) { y = 1 } else { y = 2 }
        let condition = binop(BinaryOp::Lt, var("x"), int(5));
        let then_branch = stmt_expr(assign(var("y"), int(1)));
        let else_branch = stmt_expr(assign(var("y"), int(2)));

        let ast = Stmt {
            kind: StmtKind::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: Some(Box::new(else_branch)),
            },
            span: dummy_span(),
        };

        ctx.lower_statement(&ast);
        let cfg = ctx.current_cfg.as_ref().unwrap();

        // Check that blocks were created
        assert!(cfg.blocks.keys().any(|k| k.starts_with("if_then")));
        assert!(cfg.blocks.keys().any(|k| k.starts_with("if_else")));
        assert!(cfg.blocks.keys().any(|k| k.starts_with("if_end")));

        let entry_block = &cfg.blocks[&entry_lbl];

        // entry should branch to else on false
        let branch_instr = entry_block.instructions.last().unwrap();
        assert_eq!(branch_instr.opcode, Opcode::Jump); // The unconditional jump to 'then'
        let cond_instr = &entry_block.instructions[entry_block.instructions.len() - 2];
        assert_eq!(cond_instr.opcode, Opcode::BranchIfNot);

        // Verify edges
        assert_eq!(entry_block.successors.len(), 2); // Should go to then or else
    }

    #[test]
    fn test_lower_while_loop_cfg() {
        let mut ctx = LoweringContext::new();
        ctx.current_cfg = Some(CFG::new("entry".into(), "exit".into()));
        let entry_lbl = ctx.create_block("entry");
        ctx.set_current_block(entry_lbl.clone());

        // AST: while (x > 0) { x = x - 1 }
        let condition = binop(BinaryOp::Gt, var("x"), int(0));
        let body = stmt_expr(assign(var("x"), binop(BinaryOp::Sub, var("x"), int(1))));

        let ast = Stmt {
            kind: StmtKind::While {
                condition,
                body: Box::new(body),
            },
            span: dummy_span(),
        };

        ctx.lower_statement(&ast);
        let cfg = ctx.current_cfg.as_ref().unwrap();

        // Locate block names
        let cond_lbl = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("while_cond"))
            .unwrap()
            .clone();
        let body_lbl = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("while_body"))
            .unwrap()
            .clone();
        let end_lbl = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("while_end"))
            .unwrap()
            .clone();

        let cond_block = &cfg.blocks[&cond_lbl];
        let body_block = &cfg.blocks[&body_lbl];

        // Cond block should jump to end if false, or jump to body if true
        assert!(cond_block.successors.contains(&end_lbl));
        assert!(cond_block.successors.contains(&body_lbl));

        // Body block should have a back-edge to cond block
        assert_eq!(body_block.successors.len(), 1);
        assert_eq!(body_block.successors[0], cond_lbl);

        // Verify the jump instruction at the end of the body
        let body_jump = body_block.instructions.last().unwrap();
        assert_eq!(body_jump.opcode, Opcode::Jump);
        assert_eq!(body_jump.arg1, op_lbl(&cond_lbl));
    }

    #[test]
    fn test_lower_function() {
        let mut ctx = LoweringContext::new();

        // AST: function main() { return 42; }
        let body = stmt_block(vec![Stmt {
            kind: StmtKind::Return(Some(int(42))),
            span: dummy_span(),
        }]);

        let decl = Decl {
            kind: DeclKind::Function {
                name: "main".to_string(),
                body: Some(body),
                return_ty: CType::Int,
                params: Vec::new(),
            },
            span: dummy_span(),
        };

        let program = ctx.lower_program(&[decl]);

        assert!(program.functions.contains_key("main"));
        let cfg = &program.functions["main"];

        assert_eq!(cfg.entry, "main_entry");
        assert_eq!(cfg.exit, "main_exit");

        // Look for the return instruction in the CFG
        let has_ret = cfg.blocks.values().any(|blk| {
            blk.instructions
                .iter()
                .any(|inst| inst.opcode == Opcode::Ret && inst.arg1 == op_imm(42))
        });

        assert!(has_ret, "Should emit a Ret instruction with arg 42");
    }

    #[test]
    fn test_lower_function_call() {
        let mut ctx = LoweringContext::new();
        ctx.current_cfg = Some(CFG::new("entry".into(), "exit".into()));
        let entry_lbl = ctx.create_block("entry");
        ctx.set_current_block(entry_lbl.clone());

        fn call(callee: Expr, args: Vec<Expr>) -> Expr {
            Expr {
                kind: ExprKind::Call {
                    callee: Box::new(callee),
                    args,
                },
                span: dummy_span(),
            }
        }

        // AST: result = compute(x, 42)
        let ast = stmt_expr(assign(
            var("result"),
            call(var("compute"), vec![var("x"), int(42)]),
        ));

        ctx.lower_statement(&ast);

        let cfg = ctx.current_cfg.as_ref().unwrap();
        let block = &cfg.blocks[&entry_lbl];

        assert_eq!(block.instructions.len(), 4);

        let inst0 = &block.instructions[0];
        assert_eq!(inst0.opcode, Opcode::Param);
        assert_eq!(inst0.arg1, op_var("x"));

        let inst1 = &block.instructions[1];
        assert_eq!(inst1.opcode, Opcode::Param);
        assert_eq!(inst1.arg1, op_imm(42));

        let inst2 = &block.instructions[2];
        assert_eq!(inst2.opcode, Opcode::Call);
        assert_eq!(inst2.dest, op_temp("t1"));
        assert_eq!(inst2.arg1, op_lbl("compute")); // Verifies Identifier was turned into a Label
        assert_eq!(inst2.arg2, op_imm(2));

        let inst3 = &block.instructions[3];
        assert_eq!(inst3.opcode, Opcode::Mov);
        assert_eq!(inst3.dest, op_var("result"));
        assert_eq!(inst3.arg1, op_temp("t1"));
    }
}
