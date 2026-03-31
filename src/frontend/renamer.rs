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
                let _ = self.env.declare_var(name)?;
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
