use crate::ast::Expr;

/// Errors that may occur during expression evaluation.
#[derive(Debug, PartialEq)]
pub enum EvalError {
    /// Division by zero occurred.
    DivisionByZero,
}

/// Evaluates the expression (`Expr`) and returns the calculation result.
///
/// # Errors
/// Returns `EvalError::DivisionByZero` if division by zero occurs.
pub fn eval(expr: &Expr) -> Result<f64, EvalError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Add(lhs, rhs) => Ok(eval(lhs)? + eval(rhs)?),
        Expr::Sub(lhs, rhs) => Ok(eval(lhs)? - eval(rhs)?),
        Expr::Mul(lhs, rhs) => Ok(eval(lhs)? * eval(rhs)?),
        Expr::Div(lhs, rhs) => {
            let l = eval(lhs)?;
            let r = eval(rhs)?;
            if r == 0.0 {
                Err(EvalError::DivisionByZero)
            } else {
                Ok(l / r)
            }
        }
    }
}
