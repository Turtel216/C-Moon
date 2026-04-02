//! Intermediate Representation used for middle-end optimizations and code generation

use std::collections::BTreeMap;

// ### TAC IR ###

/// TAC Operand
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// TAC Variable
    Var(usize),
    /// TAC Temporary Variable
    Temp(String),
    /// TAC Ineger literal
    ImmInt(i64),
    /// TAC Label
    Label(String),
}

/// TAC Opcode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    // Arithmetic
    /// TAC Addition e.g. %r1 + %r2
    Add,
    /// TAC subste.g. %r1 - %r2
    Sub,
    /// TAC Multiplication e.g. %r1 * %r2
    Mul,
    /// TAC Divition e.g. %r1 / %r2
    Div,

    // Relational / equality (result is 0/1)
    /// TAC Equalality e.g. %r1 == %r2
    Eq,
    /// TAC e.g. %r1 != %r2
    Neq,
    /// TAC Less then operator e.g. %r1 < %r2
    Lt,
    /// TAC Lees then or equal operator e.g. %r1 <= %r2
    Lte,
    /// TAC Greater then operator e.g. %r1 > %r2
    Gt,
    /// TAC Greater then or equal operator e.g. %r1 >= %r2
    Gte,

    // Data movement
    /// TAC Move operator e.g. dest = arg1
    Mov,

    // Control flow
    /// TAC Jump instruction goto arg1(label)
    Jump,
    /// TAC If branch if arg1 != 0 goto arg2(label)
    BranchIf,
    /// TAC If not branch if arg1 == 0 goto arg2(label)
    BranchIfNot,

    // Function calls and returns
    /// TAC Instruction to pass arguments to functions as parameters. pass arg1
    Param,
    /// TAC Instruction function calls. dest = call arg1 (func label), arg2 (number of args)
    Call,
    /// TAC Return instruction e.g. ret arg1
    Ret,
    /// Get incoming parameter at index e.g. dest = get_param 0
    GetParam,
}

/// TAC Instruction representation
#[derive(Debug, Clone, PartialEq)]
pub struct TACInstruction {
    /// Instruction operation
    pub opcode: Opcode,
    /// Instruction destination e.g. dest = 1 + 1
    pub dest: Option<Operand>,
    /// Instuctions first argument
    pub arg1: Option<Operand>,
    /// Instuctions first argument
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

/// Control Flow graph nod
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

/// Control Flow Graph representation
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
