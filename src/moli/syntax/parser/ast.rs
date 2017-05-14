#[derive(Debug, Clone)]
pub enum Expression {
    Block(Box<Vec<Statement>>),

    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),

    Identifier(String),
    Call(Box<Vec<Expression>>),

    Function {
        params: Vec<(Type, String)>,
        ret: Type,
        body:   Vec<Statement>,
    },

    Operation {
        left:  Box<Expression>,
        op:    Operand,
        right: Box<Expression>,
    },
    
    EOF,    
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Box<Expression>),

    Definition(String, Box<Expression>, Type),
    Assignment(Box<Expression>, Box<Expression>),
    
    If(Box<Expression>, Box<Vec<Statement>>),
    
    Return(Option<Box<Expression>>),
}

#[derive(Debug, Clone)]
pub enum Type {
    Str,
    Int08,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    Obj,
    Table,
    Any,
}

impl Type {
    pub fn from_str(s: &str) -> Type {
        match s {
            "str"   => Type::Str,
            "i08"   => Type::Int08,
            "i16"   => Type::Int16,
            "i32"   => Type::Int32,
            "i64"   => Type::Int64,
            "f32"   => Type::Float32,
            "f64"   => Type::Float64,
            "int"   => Type::Int32,
            "bool"  => Type::Bool,
            "obj"   => Type::Obj,
            "table" => Type::Table,
            t => panic!("non-existing type: {}", t),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Mul,
    Div,
    Mod,
    XOR,
    Plus,
    Minus,
    Equals,
    NEquals,
    Lt,
    Gt,
    LtEquals,
    GtEquals,
    And,
    Or,
    Dot,
    Assign,
}

pub fn operand(v: &str) -> Option<(Operand, u8)> {
    match v {
        "*"   => Some((Operand::Mul, 1)),
        "/"   => Some((Operand::Div, 1)),
        "%"   => Some((Operand::Mod, 1)),
        "^"   => Some((Operand::XOR, 1)),
        "+"   => Some((Operand::Plus, 2)),
        "-"   => Some((Operand::Minus, 2)),
        "=="  => Some((Operand::Equals, 3)),
        "!="  => Some((Operand::NEquals, 3)),
        "<"   => Some((Operand::Lt, 4)),
        ">"   => Some((Operand::Gt, 4)),
        "<="  => Some((Operand::LtEquals, 4)),
        ">="  => Some((Operand::GtEquals, 4)),
        "and" => Some((Operand::And, 4)),
        "or"  => Some((Operand::Or, 4)),
        "."   => Some((Operand::Dot, 4)),
        "="   => Some((Operand::Assign, 5)),
        _ => None,
    }
}