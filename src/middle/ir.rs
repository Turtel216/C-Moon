use std::collections::BTreeMap;

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
    Param,    // pass arg1 as a parameter
    Call,     // dest = call arg1 (func label), arg2 (number of args)
    Ret,      // return arg1
    GetParam, // dest = get incoming parameter at index arg1
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
