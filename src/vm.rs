enum OpCode {
    Load(String),
    And,
    Or,
    Not,
}

struct VM {
    opcodes: Vec<OpCode>,
    cur: usize,
}
