use std::{fmt};


#[derive(Clone)]
pub struct Instruction {
    pub op: Operator,
    pub arg: Option<Ptr>,
    pub typ: Option<Type>,
    pub val: Option<[u8; 4]>,
    pub pos: Pos,
}

impl Instruction {
    pub fn new(
        op: Operator,
        typ: Option<Type>,
        val: Option<[u8; 4]>,
        pos: Pos,
    ) -> Self {
        Instruction {
            op,
            arg: None,
            typ,
            val,
            pos,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, ", self.op).unwrap();
        if let Some(x) = self.arg {
            write!(f, "{:?}, ", x).unwrap();
        }
        if let Some(x) = self.typ {
            write!(f, "{:?}, ", x).unwrap();
        }
        if let Some(x) = &self.val {
            write!(f, "{:?}, ", x).unwrap();
        }
        write!(f, "{:?}", self.pos)
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int,// (i32)
    Float, // (f32)
    Bool, // (bool)
    Str, // (Ptr)index in str_heap
    //Type(str) "Int", "Float" ...
}

#[derive(Debug)]
pub struct Value {
    pub typ: Type,
    pub val: [u8; 4]
}
pub type Pos = (usize, usize, String);
pub type Ptr = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Literal, // Value

    Add,
    Sub,
    Mult,
    Div,

    Print,
    Input,

    Not,
    And,
    Or,

    Eq,
    Lt,
    Le,
    Gt,
    Ge,

    If,   // Option<Ptr>
    Else, // Option<Ptr>
    End,  // Option<Ptr>
    Do,   // Option<Ptr>
    While,

    Dup,
    Drop,
    Swap,
    // Null
}
