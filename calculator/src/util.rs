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
    Power,
}

#[derive(Copy, Clone)]
pub enum Func {
    Sqrt,
    Sin,
    Cos,
    Tan,
    Abs,
    Round,
    Factorial,
}
