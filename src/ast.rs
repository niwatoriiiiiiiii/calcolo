/// AST node for mathematical expressions.
pub enum Expr {
    /// Numeric literal
    Number(f64),

    /// Addition (left-hand side + right-hand side)
    Add(Box<Expr>, Box<Expr>),

    /// Subtraction (left-hand side - right-hand side)
    Sub(Box<Expr>, Box<Expr>),

    /// Multiplication (left-hand side * right-hand side)
    Mul(Box<Expr>, Box<Expr>),

    /// Division (left-hand side / right-hand side)
    Div(Box<Expr>, Box<Expr>),
}
