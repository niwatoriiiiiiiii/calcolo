//! `calcolo` is a CLI calculator that lets you write math expressions
//! using LaTeX-style notation and evaluate them from script files.

/// Type definition of the abstract syntax tree.
pub mod ast;

/// The process of evaluating an AST to obtain the calculation result.
pub mod eval;

#[cfg(test)]
mod tests {
    use crate::ast::Expr;
    use crate::eval::{EvalError, eval};

    #[test]
    fn test_add() {
        // 3 + 4 = 7
        let expr = Expr::Add(Box::new(Expr::Number(3.0)), Box::new(Expr::Number(4.0)));
        assert_eq!(eval(&expr), Ok(7.0));
    }

    #[test]
    fn test_sub() {
        // 3 - 4 = -1
        let expr = Expr::Sub(Box::new(Expr::Number(3.0)), Box::new(Expr::Number(4.0)));
        assert_eq!(eval(&expr), Ok(-1.0));
    }

    #[test]
    fn test_mul() {
        // 3 * 4 = 12
        let expr = Expr::Mul(Box::new(Expr::Number(3.0)), Box::new(Expr::Number(4.0)));
        assert_eq!(eval(&expr), Ok(12.0));
    }

    #[test]
    fn test_div() {
        // 12 / 4 = 3
        let expr = Expr::Div(Box::new(Expr::Number(12.0)), Box::new(Expr::Number(4.0)));
        assert_eq!(eval(&expr), Ok(3.0));
    }

    #[test]
    fn test_add_mul_precedence() {
        // 2 + 3 * 4 = 14
        let expr = Expr::Add(
            Box::new(Expr::Number(2.0)),
            Box::new(Expr::Mul(
                Box::new(Expr::Number(3.0)),
                Box::new(Expr::Number(4.0)),
            )),
        );
        assert_eq!(eval(&expr), Ok(14.0));
    }

    #[test]
    fn test_negative_numbers() {
        // -3 + -4 = -7
        let expr = Expr::Add(Box::new(Expr::Number(-3.0)), Box::new(Expr::Number(-4.0)));
        assert_eq!(eval(&expr), Ok(-7.0));
    }

    #[test]
    fn test_sub_order() {
        // 4 - 3 = 1
        let expr = Expr::Sub(Box::new(Expr::Number(4.0)), Box::new(Expr::Number(3.0)));
        assert_eq!(eval(&expr), Ok(1.0));

        // 3 - 4 = -1
        let expr = Expr::Sub(Box::new(Expr::Number(3.0)), Box::new(Expr::Number(4.0)));
        assert_eq!(eval(&expr), Ok(-1.0));
    }

    #[test]
    fn test_left_to_right_evaluation() {
        // 2 - 3 + 4 = 3
        let expr = Expr::Add(
            Box::new(Expr::Sub(
                Box::new(Expr::Number(2.0)),
                Box::new(Expr::Number(3.0)),
            )),
            Box::new(Expr::Number(4.0)),
        );
        assert_eq!(eval(&expr), Ok(3.0));
    }

    #[test]
    fn test_nested_expr() {
        // (2 + 3) * (4 - 1) = 15
        let expr = Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Number(2.0)),
                Box::new(Expr::Number(3.0)),
            )),
            Box::new(Expr::Sub(
                Box::new(Expr::Number(4.0)),
                Box::new(Expr::Number(1.0)),
            )),
        );
        assert_eq!(eval(&expr), Ok(15.0));
    }

    #[test]
    fn test_div_error() {
        // 3 / 0 = EvalError::DivisionByZero
        let expr = Expr::Div(Box::new(Expr::Number(3.0)), Box::new(Expr::Number(0.0)));
        assert_eq!(eval(&expr), Err(EvalError::DivisionByZero));
    }

    #[test]
    fn test_zero_divided_by_number() {
        // 0 / 3 = 0
        let expr = Expr::Div(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(3.0)));
        assert_eq!(eval(&expr), Ok(0.0));
    }
}
