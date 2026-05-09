// PELAKSANAAN SEMENTARA: Pengkompil ini ditulis dalam Rust.
// Ia akan digantikan dengan pengkompil ASA yang ditulis dalam ASA
// sebaik sahaja bahasa ini matang. Lihat SUPREMACY.txt

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, init: Expression },
    Assign { name: String, value: Expression },
    Expression(Expression),
    If { condition: Expression, then_branch: Vec<Statement>, else_branch: Option<Vec<Statement>> },
    For { var: String, start: Expression, end: Expression, body: Vec<Statement> },
    While { condition: Expression, body: Vec<Statement> },
    Return(Option<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(String),
    IntLiteral(i64),
    StringLiteral(String),
    Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression> },
    Unary { op: UnaryOp, expr: Box<Expression> },
    Call { func: String, args: Vec<Expression> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Eq, Neq, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int, String, Bool, Void, Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}
