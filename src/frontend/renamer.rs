use crate::frontend::ast::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ResolutionMap {
    /// Maps *identifier-expression node id* -> globally unique variable id
    pub expr_to_var: HashMap<NodeId, usize>,
    /// Maps *declaration node id* -> globally unique variable id
    pub decl_to_var: HashMap<NodeId, usize>,
}

impl ResolutionMap {
    pub fn new() -> Self {
        Self {
            expr_to_var: HashMap::new(),
            decl_to_var: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RenameErrorKind {
    UndeclaredVariable { name: String },
    RedeclarationInSameScope { name: String },
    UndeclaredFunction { name: String },
}

#[derive(Debug, Clone)]
pub struct RenameError {
    pub kind: RenameErrorKind,
}

type RenameResult<T> = Result<T, RenameError>;

/// Stack-based lexical environment.
/// Vec back = innermost scope.
#[derive(Debug, Default)]
pub struct Environment {
    scopes: Vec<HashMap<String, usize>>,
    next_var_id: usize,
    functions: HashSet<String>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            scopes: Vec::new(),
            next_var_id: 0,
            functions: HashSet::new(),
        };
        env.push_scope(); // global var scope
        env
    }

    #[inline]
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    #[inline]
    pub fn pop_scope(&mut self) {
        self.scopes.pop().expect("scope underflow");
    }

    pub fn declare_var(&mut self, name: &str) -> RenameResult<usize> {
        let cur = self.scopes.last_mut().expect("no active scope");
        if cur.contains_key(name) {
            return Err(RenameError {
                kind: RenameErrorKind::RedeclarationInSameScope {
                    name: name.to_owned(),
                },
            });
        }
        let id = self.next_var_id;
        self.next_var_id += 1;
        cur.insert(name.to_owned(), id);
        Ok(id)
    }

    pub fn lookup_var(&self, name: &str) -> Option<usize> {
        self.scopes.iter().rev().find_map(|s| s.get(name).copied())
    }

    pub fn declare_function(&mut self, name: &str) {
        self.functions.insert(name.to_owned());
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains(name)
    }
}

/// Visitor trait (minimal for this pass).
pub trait AstVisitor {
    fn visit_decl(&mut self, decl: &Decl) -> RenameResult<()>;
    fn visit_stmt(&mut self, stmt: &Stmt) -> RenameResult<()>;
    fn visit_expr(&mut self, expr: &Expr) -> RenameResult<()>;
}

pub struct Renamer {
    env: Environment,
    out: ResolutionMap,
}

impl Renamer {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            out: ResolutionMap::new(),
        }
    }

    pub fn into_resolution_map(self) -> ResolutionMap {
        self.out
    }

    fn bind_decl_node(&mut self, node_id: NodeId, var_id: usize) {
        self.out.decl_to_var.insert(node_id, var_id);
    }

    fn bind_expr_node(&mut self, node_id: NodeId, var_id: usize) {
        self.out.expr_to_var.insert(node_id, var_id);
    }

    /// Enter function scope: parameters + body.
    fn visit_function(
        &mut self,
        _return_ty: &CType,
        _name: &str,
        params: &[ParamDecl],
        body: Option<&Stmt>,
    ) -> RenameResult<()> {
        self.env.push_scope();

        for p in params {
            if let Some(name) = p.name.as_deref() {
                let param_id = self.env.declare_var(name)?;
                self.bind_decl_node(p.id, param_id);
            }
        }

        if let Some(b) = body {
            self.visit_stmt(b)?;
        }

        self.env.pop_scope();
        Ok(())
    }
}

impl AstVisitor for Renamer {
    fn visit_decl(&mut self, decl: &Decl) -> RenameResult<()> {
        match &decl.kind {
            DeclKind::Variable {
                ty: _,
                name,
                initializer,
            } => {
                let var_id = self.env.declare_var(name)?;
                self.bind_decl_node(decl.id, var_id);
                if let Some(init) = initializer {
                    self.visit_expr(init)?;
                }
                Ok(())
            }
            DeclKind::Function {
                return_ty,
                name,
                params,
                body,
            } => {
                self.env.declare_function(name);
                self.visit_function(return_ty, name, params, body.as_ref())
            }
            DeclKind::Struct { .. } => Ok(()),
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> RenameResult<()> {
        match &stmt.kind {
            StmtKind::Expr(e) => self.visit_expr(e),
            StmtKind::Return(e) => {
                if let Some(e) = e {
                    self.visit_expr(e)?;
                }
                Ok(())
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expr(condition)?;
                self.visit_stmt(then_branch)?;
                if let Some(e) = else_branch {
                    self.visit_stmt(e)?;
                }
                Ok(())
            }
            StmtKind::While { condition, body } => {
                self.visit_expr(condition)?;
                self.visit_stmt(body)
            }
            StmtKind::For {
                init,
                condition,
                step,
                body,
            } => {
                self.env.push_scope();
                if let Some(i) = init {
                    self.visit_stmt(i)?;
                }
                if let Some(c) = condition {
                    self.visit_expr(c)?;
                }
                if let Some(s) = step {
                    self.visit_expr(s)?;
                }
                self.visit_stmt(body)?;
                self.env.pop_scope();
                Ok(())
            }
            StmtKind::Block(items) => {
                self.env.push_scope();
                for item in items {
                    match item {
                        BlockItem::Stmt(s) => self.visit_stmt(s)?,
                        BlockItem::Decl(d) => self.visit_decl(d)?,
                    }
                }
                self.env.pop_scope();
                Ok(())
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> RenameResult<()> {
        match &expr.kind {
            ExprKind::Literal(_) => Ok(()),

            ExprKind::Identifier(name) => {
                let var_id = self.env.lookup_var(name).ok_or_else(|| RenameError {
                    kind: RenameErrorKind::UndeclaredVariable { name: name.clone() },
                })?;
                self.bind_expr_node(expr.id, var_id);
                Ok(())
            }

            ExprKind::Binary(_, l, r) => {
                self.visit_expr(l)?;
                self.visit_expr(r)
            }

            ExprKind::Unary(_, e) => self.visit_expr(e),

            ExprKind::Call { callee, args } => {
                if let ExprKind::Identifier(name) = &callee.kind {
                    if !self.env.has_function(name) && self.env.lookup_var(name).is_none() {
                        return Err(RenameError {
                            kind: RenameErrorKind::UndeclaredFunction { name: name.clone() },
                        });
                    }
                } else {
                    self.visit_expr(callee)?;
                }

                for a in args {
                    self.visit_expr(a)?;
                }
                Ok(())
            }

            ExprKind::Index { array, index } => {
                self.visit_expr(array)?;
                self.visit_expr(index)
            }
            ExprKind::MemberAccess { base, .. } => self.visit_expr(base),
            ExprKind::Cast(_, e) => self.visit_expr(e),
            ExprKind::SizeOf(e) => self.visit_expr(e),
        }
    }
}

/// Entry point over translation-unit-level declarations.
///
/// Input: parsed + semantically-valid top-level declarations.
/// Output: side-table resolution map (decl refs + identifier refs) or first rename error.
pub fn resolve_names(program_decls: &[Decl]) -> RenameResult<ResolutionMap> {
    let mut renamer = Renamer::new();

    for d in program_decls {
        if let DeclKind::Function { name, .. } = &d.kind {
            renamer.env.declare_function(name);
        }
    }

    for d in program_decls {
        renamer.visit_decl(d)?;
    }

    Ok(renamer.into_resolution_map())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::Span;

    // ### Minimal test helpers ###
    fn nid(n: u32) -> NodeId {
        n
    }

    fn int_ty() -> CType {
        CType::Int
    }

    fn dummy_span() -> Span {
        Span {
            line: 0,
            column: 0,
            length: 0,
        }
    }

    fn ident(id: u32, name: &str) -> Expr {
        Expr {
            id: nid(id),
            span: dummy_span(),
            kind: ExprKind::Identifier(name.to_string()),
        }
    }

    fn int_lit(id: u32, v: i64) -> Expr {
        Expr {
            id: nid(id),
            span: dummy_span(),
            kind: ExprKind::Literal(Literal::Int(v)),
        }
    }

    fn add(id: u32, l: Expr, r: Expr) -> Expr {
        Expr {
            id: nid(id),
            span: dummy_span(),
            kind: ExprKind::Binary(BinaryOp::Add, Box::new(l), Box::new(r)),
        }
    }

    fn lt(id: u32, l: Expr, r: Expr) -> Expr {
        Expr {
            id: nid(id),
            span: dummy_span(),
            kind: ExprKind::Binary(BinaryOp::Lt, Box::new(l), Box::new(r)),
        }
    }

    fn call(id: u32, callee: Expr, args: Vec<Expr>) -> Expr {
        Expr {
            id: nid(id),
            span: dummy_span(),
            kind: ExprKind::Call {
                callee: Box::new(callee),
                args,
            },
        }
    }

    fn var_decl(id: u32, name: &str, init: Option<Expr>) -> Decl {
        Decl {
            id: nid(id),
            span: dummy_span(),
            kind: DeclKind::Variable {
                ty: int_ty(),
                name: name.to_string(),
                initializer: init,
            },
        }
    }

    fn ret_stmt(id: u32, e: Option<Expr>) -> Stmt {
        Stmt {
            id: nid(id),
            span: dummy_span(),
            kind: StmtKind::Return(e),
        }
    }

    fn expr_stmt(id: u32, e: Expr) -> Stmt {
        Stmt {
            id: nid(id),
            span: dummy_span(),
            kind: StmtKind::Expr(e),
        }
    }

    fn block_stmt(id: u32, items: Vec<BlockItem>) -> Stmt {
        Stmt {
            id: nid(id),
            span: dummy_span(),
            kind: StmtKind::Block(items),
        }
    }

    fn if_stmt(id: u32, cond: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Stmt {
        Stmt {
            id: nid(id),
            span: dummy_span(),
            kind: StmtKind::If {
                condition: cond,
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    fn fn_decl(id: u32, name: &str, params: Vec<ParamDecl>, body: Option<Stmt>) -> Decl {
        Decl {
            id: nid(id),
            span: dummy_span(),
            kind: DeclKind::Function {
                return_ty: int_ty(),
                name: name.to_string(),
                params,
                body,
            },
        }
    }

    fn param(name: &str) -> ParamDecl {
        ParamDecl {
            ty: int_ty(),
            name: Some(name.to_string()),
        }
    }

    // ### Tests ###

    #[test]
    fn resolves_function_call_and_shadowing_sample_program() {
        // int foo(int x, int y) { return x + 1; }
        let foo_body = block_stmt(
            10,
            vec![BlockItem::Stmt(ret_stmt(
                11,
                Some(add(12, ident(13, "x"), int_lit(14, 1))),
            ))],
        );
        let foo = fn_decl(1, "foo", vec![param("x"), param("y")], Some(foo_body));

        // int main() {
        //   int x = 1;
        //   int y = foo(x, 2);
        //   { int x = 5; }
        //   if (x < y) { return 0; }
        //   return 1;
        // }
        let inner_block = block_stmt(
            30,
            vec![BlockItem::Decl(var_decl(31, "x", Some(int_lit(32, 5))))],
        );

        let if_block = block_stmt(
            40,
            vec![BlockItem::Stmt(ret_stmt(41, Some(int_lit(42, 0))))],
        );

        let main_body = block_stmt(
            20,
            vec![
                BlockItem::Decl(var_decl(21, "x", Some(int_lit(22, 1)))),
                BlockItem::Decl(var_decl(
                    23,
                    "y",
                    Some(call(
                        24,
                        ident(25, "foo"),
                        vec![ident(26, "x"), int_lit(27, 2)],
                    )),
                )),
                BlockItem::Stmt(inner_block),
                BlockItem::Stmt(if_stmt(
                    50,
                    lt(51, ident(52, "x"), ident(53, "y")),
                    if_block,
                    None,
                )),
                BlockItem::Stmt(ret_stmt(60, Some(int_lit(61, 1)))),
            ],
        );

        let main_fn = fn_decl(2, "main", vec![], Some(main_body));

        let map = resolve_names(&[foo, main_fn]).expect("name resolution should succeed");

        // Ensure call callee `foo` is NOT treated as undeclared variable:
        assert!(
            map.expr_to_var.get(&25).is_none(),
            "function name callee should not be assigned a variable id"
        );

        // `x` in foo return should map to foo param x
        let foo_param_x_use = map.expr_to_var.get(&13).copied().expect("foo x use mapped");

        // `x` in foo(x, 2) should map to main's x
        let main_x_use_in_call = map
            .expr_to_var
            .get(&26)
            .copied()
            .expect("main x use mapped");

        // `x` and `y` in if (x < y)
        let main_x_use_in_if = map.expr_to_var.get(&52).copied().expect("if x mapped");
        let main_y_use_in_if = map.expr_to_var.get(&53).copied().expect("if y mapped");

        // shadowed x decl in inner block
        let inner_x_decl = map
            .decl_to_var
            .get(&31)
            .copied()
            .expect("inner x decl mapped");
        let main_x_decl = map
            .decl_to_var
            .get(&21)
            .copied()
            .expect("main x decl mapped");

        assert_eq!(main_x_use_in_call, main_x_decl);
        assert_eq!(main_x_use_in_if, main_x_decl);
        assert_eq!(main_y_use_in_if, map.decl_to_var[&23]);
        assert_ne!(
            inner_x_decl, main_x_decl,
            "shadowed x must have different id"
        );
        assert_ne!(
            foo_param_x_use, main_x_decl,
            "foo param x is distinct from main x"
        );
    }

    #[test]
    fn errors_on_undeclared_variable() {
        // int main() { return z; }
        let main_body = block_stmt(
            100,
            vec![BlockItem::Stmt(ret_stmt(101, Some(ident(102, "z"))))],
        );
        let main_fn = fn_decl(103, "main", vec![], Some(main_body));

        let err = resolve_names(&[main_fn]).unwrap_err();
        match err.kind {
            RenameErrorKind::UndeclaredVariable { name } => assert_eq!(name, "z"),
            other => panic!("expected UndeclaredVariable, got {other:?}"),
        }
    }

    #[test]
    fn errors_on_redeclaration_in_same_scope() {
        // int main() { int x = 1; int x = 2; return x; }
        let main_body = block_stmt(
            200,
            vec![
                BlockItem::Decl(var_decl(201, "x", Some(int_lit(202, 1)))),
                BlockItem::Decl(var_decl(203, "x", Some(int_lit(204, 2)))),
                BlockItem::Stmt(ret_stmt(205, Some(ident(206, "x")))),
            ],
        );
        let main_fn = fn_decl(207, "main", vec![], Some(main_body));

        let err = resolve_names(&[main_fn]).unwrap_err();
        match err.kind {
            RenameErrorKind::RedeclarationInSameScope { name } => assert_eq!(name, "x"),
            other => panic!("expected RedeclarationInSameScope, got {other:?}"),
        }
    }

    #[test]
    fn allows_shadowing_in_nested_block() {
        // int main() { int x = 1; { int x = 2; } return x; }
        let nested = block_stmt(
            300,
            vec![BlockItem::Decl(var_decl(301, "x", Some(int_lit(302, 2))))],
        );
        let body = block_stmt(
            303,
            vec![
                BlockItem::Decl(var_decl(304, "x", Some(int_lit(305, 1)))),
                BlockItem::Stmt(nested),
                BlockItem::Stmt(ret_stmt(306, Some(ident(307, "x")))),
            ],
        );
        let main_fn = fn_decl(308, "main", vec![], Some(body));

        let map = resolve_names(&[main_fn]).expect("should allow shadowing");
        let outer_x = map.decl_to_var[&304];
        let inner_x = map.decl_to_var[&301];
        let ret_x = map.expr_to_var[&307];

        assert_ne!(outer_x, inner_x);
        assert_eq!(ret_x, outer_x, "after nested block, x resolves to outer x");
    }
}
