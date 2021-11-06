pub enum Expr {
    Number(f64),
    Operation(Box<Expr>, Operand, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Operand {
    Mul,
    Div,
    Add,
    Sub,
}
