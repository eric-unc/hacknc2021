#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator);
mod util;

use util::Expr;
use util::Expr::Number;
use util::Expr::Operation;
use util::Expr::Error;
use util::Operand;

pub fn calculate(input: &str) -> Result<f64, String> {
    let expr = calculator::ExprParser::new()
        .parse(input)
        .unwrap();

    calculate_expr(&*expr)
}

// recursively evaluate each expression
fn calculate_expr(expr: &Expr) -> Result<f64, String> {
    match &*expr {
        Number(n) => return Ok(*n),
        Operation(expr1, operand, expr2) => {
            let result1 = calculate_expr(expr1).unwrap();
            let result2 = calculate_expr(expr2).unwrap();
            return Ok(evaluate_operation(result1, result2, *operand));
        }
        Error => {
            return Err(String::from("calculate error"))
        }
    }
}

fn evaluate_operation(x1: f64, x2: f64, operand: Operand) -> f64 {
    match operand {
        Operand::Add => x1 + x2,
        Operand::Sub => x1 - x2,
        Operand::Mul => x1 * x2,
        Operand::Div => x1 / x2
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn test_calculate() {        
        assert_eq!(2.0, calculate("2").unwrap());
        assert_eq!(4.0, calculate("2 + 2").unwrap());
        assert_eq!(6.0, calculate("2 * 3").unwrap());
        assert_eq!(8.0, calculate("2 * 3 + 2").unwrap());
    }
}
