pub enum Expr {
    Number(f64),
    Operation(Box<Expr>, Operand, Box<Expr>),
    Function(Func, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Operand {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Copy, Clone)]
pub enum Func {
    Sqrt,
}
