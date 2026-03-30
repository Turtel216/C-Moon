use std::collections::BTreeMap;

use crate::frontend::ast::{
    BinaryOp, BlockItem, DeclKind, Expr, ExprKind, Literal, Stmt, StmtKind,
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

pub struct LoweringContext {
    pub cfg: CFG,
    current_block: String,
    temp_counter: usize,
    label_counter: usize,
}

impl LoweringContext {
    pub fn new() -> Self {
        let entry = "entry".to_string();
        let exit = "exit".to_string();

        let mut cfg = CFG::new(entry.clone(), exit.clone());
        cfg.add_block(BasicBlock::new(entry.clone()));
        cfg.add_block(BasicBlock::new(exit.clone()));

        Self {
            cfg,
            current_block: entry,
            temp_counter: 0,
            label_counter: 0,
        }
    }

    pub fn lower_stmt_tree(mut self, root: &Stmt) -> CFG {
        self.lower_statement(root);

        // fallthrough to exit if not already there
        let cur = self.current_block.clone();
        if cur != self.cfg.exit {
            self.emit(TACInstruction::new(
                Opcode::Jump,
                None,
                Some(Operand::Label(self.cfg.exit.clone())),
                None,
            ));
            let exit = self.cfg.exit.clone();
            self.cfg.add_edge(&cur, &exit);
        }

        self.cfg
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
        self.cfg.add_block(BasicBlock::new(label.clone()));
        label
    }

    fn set_current_block(&mut self, label: String) {
        self.current_block = label;
    }

    fn emit(&mut self, instr: TACInstruction) {
        let blk = self.cfg.blocks.get_mut(&self.current_block).unwrap();
        blk.emit(instr);
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
                self.cfg.add_edge(&cur, &else_label);

                // otherwise -> then
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(then_label.clone())),
                    None,
                ));
                self.cfg.add_edge(&cur, &then_label);

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
                self.cfg.add_edge(&then_end, &end_label);

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
                self.cfg.add_edge(&else_end, &end_label);

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
                self.cfg.add_edge(&preheader, &cond_label);

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
                self.cfg.add_edge(&cond_label, &end_label);

                // else -> body
                self.emit(TACInstruction::new(
                    Opcode::Jump,
                    None,
                    Some(Operand::Label(body_label.clone())),
                    None,
                ));
                self.cfg.add_edge(&cond_label, &body_label);

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
                self.cfg.add_edge(&body_end, &cond_label);

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

            // Not in current scope; no-op for now
            StmtKind::Return(_) | StmtKind::For { .. } => {}
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
    use crate::frontend::ast::{
        BinaryOp, BlockItem, CType, Decl, DeclKind, Expr, ExprKind, Literal, Stmt, StmtKind,
    };
    use crate::frontend::lexer::Span;

    fn dummy_span() -> Span {
        Span {
            line: 0,
            column: 0,
            length: 0,
        }
    }

    fn int_lit(v: i64) -> Expr {
        Expr {
            kind: ExprKind::Literal(Literal::Int(v)),
            span: dummy_span(),
        }
    }

    fn ident(name: &str) -> Expr {
        Expr {
            kind: ExprKind::Identifier(name.to_string()),
            span: dummy_span(),
        }
    }

    fn bin(op: BinaryOp, lhs: Expr, rhs: Expr) -> Expr {
        Expr {
            kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
            span: dummy_span(),
        }
    }

    fn expr_stmt(e: Expr) -> Stmt {
        Stmt {
            kind: StmtKind::Expr(e),
            span: dummy_span(),
        }
    }

    fn block(stmts: Vec<Stmt>) -> Stmt {
        Stmt {
            kind: StmtKind::Block(stmts.into_iter().map(BlockItem::Stmt).collect()),
            span: dummy_span(),
        }
    }

    #[test]
    fn lowers_assignment_with_add_into_tac() {
        // x = 1 + 2;
        let stmt = expr_stmt(bin(
            BinaryOp::Assign,
            ident("x"),
            bin(BinaryOp::Add, int_lit(1), int_lit(2)),
        ));

        let cfg = LoweringContext::new().lower_stmt_tree(&stmt);
        let entry = cfg.blocks.get("entry").expect("entry block missing");

        // We expect:
        // t1 = 1 + 2
        // x = t1
        // jump exit
        assert!(entry.instructions.len() >= 3);

        let inst0 = &entry.instructions[0];
        assert_eq!(inst0.opcode, Opcode::Add);
        assert!(matches!(inst0.dest, Some(Operand::Temp(_))));
        assert_eq!(inst0.arg1, Some(Operand::ImmInt(1)));
        assert_eq!(inst0.arg2, Some(Operand::ImmInt(2)));

        let inst1 = &entry.instructions[1];
        assert_eq!(inst1.opcode, Opcode::Mov);
        assert_eq!(inst1.dest, Some(Operand::Var("x".to_string())));
        assert!(matches!(inst1.arg1, Some(Operand::Temp(_))));

        assert!(
            entry.successors.contains(&"exit".to_string()),
            "entry should flow to exit"
        );
    }

    #[test]
    fn lowers_if_else_creates_expected_blocks_and_join_edges() {
        // if (x < 10) { y = 1; } else { y = 2; }
        let cond = bin(BinaryOp::Lt, ident("x"), int_lit(10));
        let then_stmt = expr_stmt(bin(BinaryOp::Assign, ident("y"), int_lit(1)));
        let else_stmt = expr_stmt(bin(BinaryOp::Assign, ident("y"), int_lit(2)));

        let ast = Stmt {
            kind: StmtKind::If {
                condition: cond,
                then_branch: Box::new(block(vec![then_stmt])),
                else_branch: Some(Box::new(block(vec![else_stmt]))),
            },
            span: dummy_span(),
        };

        let cfg = LoweringContext::new().lower_stmt_tree(&ast);

        let then_label = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("if_then_"))
            .expect("missing then block")
            .clone();
        let else_label = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("if_else_"))
            .expect("missing else block")
            .clone();
        let end_label = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("if_end_"))
            .expect("missing if_end block")
            .clone();

        let entry = cfg.blocks.get("entry").unwrap();
        assert!(entry.successors.contains(&then_label));
        assert!(entry.successors.contains(&else_label));

        let then_blk = cfg.blocks.get(&then_label).unwrap();
        let else_blk = cfg.blocks.get(&else_label).unwrap();
        assert!(then_blk.successors.contains(&end_label));
        assert!(else_blk.successors.contains(&end_label));

        let end_blk = cfg.blocks.get(&end_label).unwrap();
        assert!(
            end_blk.predecessors.contains(&then_label)
                && end_blk.predecessors.contains(&else_label),
            "if_end should have both then/else predecessors"
        );
    }

    #[test]
    fn lowers_while_creates_back_edge_from_body_to_condition() {
        // while (x < 3) { x = x + 1; }
        let cond = bin(BinaryOp::Lt, ident("x"), int_lit(3));
        let step = expr_stmt(bin(
            BinaryOp::Assign,
            ident("x"),
            bin(BinaryOp::Add, ident("x"), int_lit(1)),
        ));

        let ast = Stmt {
            kind: StmtKind::While {
                condition: cond,
                body: Box::new(block(vec![step])),
            },
            span: dummy_span(),
        };

        let cfg = LoweringContext::new().lower_stmt_tree(&ast);

        let cond_label = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("while_cond_"))
            .expect("missing while_cond block")
            .clone();
        let body_label = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("while_body_"))
            .expect("missing while_body block")
            .clone();
        let end_label = cfg
            .blocks
            .keys()
            .find(|k| k.starts_with("while_end_"))
            .expect("missing while_end block")
            .clone();

        let entry = cfg.blocks.get("entry").unwrap();
        assert!(entry.successors.contains(&cond_label));

        let cond_blk = cfg.blocks.get(&cond_label).unwrap();
        assert!(cond_blk.successors.contains(&body_label));
        assert!(cond_blk.successors.contains(&end_label));

        let body_blk = cfg.blocks.get(&body_label).unwrap();
        assert!(
            body_blk.successors.contains(&cond_label),
            "while body should have back-edge to while_cond"
        );
    }

    #[test]
    fn lowers_decl_initializer_in_block() {
        // { int x = 7; }
        let decl = Decl {
            kind: DeclKind::Variable {
                ty: CType::Int,
                name: "x".to_string(),
                initializer: Some(int_lit(7)),
            },
            span: dummy_span(),
        };

        let ast = Stmt {
            kind: StmtKind::Block(vec![BlockItem::Decl(decl)]),
            span: dummy_span(),
        };

        let cfg = LoweringContext::new().lower_stmt_tree(&ast);
        let entry = cfg.blocks.get("entry").unwrap();

        // Expect MOV x, 7
        let mov = entry
            .instructions
            .iter()
            .find(|i| i.opcode == Opcode::Mov)
            .expect("expected Mov for decl initializer");
        assert_eq!(mov.dest, Some(Operand::Var("x".to_string())));
        assert_eq!(mov.arg1, Some(Operand::ImmInt(7)));
    }
}
