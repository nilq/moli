#[derive(Debug, Clone)]
pub enum Expression {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),

    Identifier(String),
    Call(Box<Vec<Expression>>),
    
    Function {
        name: Option<String>,
        args: Vec<String>,
        body: Vec<Statement>,
    },

    Operation {
        left:  Box<Expression>,
        op:    Operand,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Box<Vec<Statement>>),
    Expression(Box<Expression>),

    Definition(String, Box<Expression>, Type),
    Assignment(Box<Expression>, Box<Expression>),
    
    If(Box<Expression>, Box<Vec<Statement>>),
    
    Return(Option<Box<Expression>>),    
}

#[derive(Debug, Clone)]
pub enum Type {
    Str, Int, Float, Bool, Obj,
}

impl Type {
    pub fn from_str(s: &str) -> Type {
        match s {
            "str" => Type::Str,
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