#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator);
mod util;

use util::Expr;
use util::Expr::Number;
use util::Expr::Operation;
use util::Expr::Function;
use util::Expr::Constant;
use util::Expr::Error;
use util::Operand;
use util::Func;

// calculate the result based on input string
pub fn calculate(input: &str) -> Result<f64, String> {
    match calculator::ExprParser::new().parse(input) {
        Ok(v) => calculate_expr(&*v),
        Err(_) => Err(String::from("parsing error"))
    }
}

// recursively evaluate each expression
fn calculate_expr(expr: &Expr) -> Result<f64, String> {
    match &*expr {
        Number(n) => return Ok(*n),
        Operation(expr1, operand, expr2) => {
            let result1 = calculate_expr(expr1)?;
            let result2 = calculate_expr(expr2)?;
            return Ok(evaluate_operation(result1, result2, *operand));
        }
        Function(func, expr1) => {
            let result1 = calculate_expr(expr1)?;
            return evaluate_function(*func, result1)
        },
        Constant(constant) => Ok(evaluate_constant(*constant)),
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
        Operand::Div => x1 / x2,
        Operand::Rem => x1 % x2,
        Operand::Power => x1.powf(x2),
    }
}

fn evaluate_constant(constant: util::Constant) -> f64 {
    match constant {
        util::Constant::PI => std::f64::consts::PI,
        util::Constant::E => std::f64::consts::E,
    }
}

fn evaluate_function(func: Func, x: f64) -> Result<f64, String> {
    match func {
        Func::Sqrt => {
            let result = x.sqrt();
            if result.is_nan() {
                return Err(String::from("sqrt only operates on non-negative numbers"))
            }
            Ok(result)
        }
        Func::Sin => Ok(x.sin()),
        Func::Cos => Ok(x.cos()),
        Func::Tan => Ok(x.tan()),
        Func::Abs => Ok(x.abs()),
        Func::Round => Ok(x.round()),
        Func::Factorial => {
            if x.fract() != 0.0 {
                return Err(String::from("factorial only operates on positive integers"))
            }
            return Ok(factorial(x))
        },
    }
}

fn factorial(x: f64) -> f64 {
    if x <= 1.0 {
        return 1.0
    }

    factorial(x - 1.0) * x
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn test_calculate() {        
        assert_eq!(2.0, calculate("2").unwrap());
        assert_eq!(4.0, calculate("2 + 2").unwrap());
        assert_eq!(6.0, calculate("2 * 3").unwrap());
        assert_eq!(8.0, calculate("2 ^ 3").unwrap());
        assert_eq!(8.0, calculate("2 * 3 + 2").unwrap());
        assert_eq!(10.0, calculate("2 * 3 + 2 ^ 2").unwrap());

        assert_eq!(2.0, calculate("sqrt(4)").unwrap());
        assert_eq!(5.0, calculate("sqrt(4) + 3").unwrap());
        assert_eq!(0.0, calculate("sin(0)").unwrap());
        assert_eq!(1.0, calculate("cos(0)").unwrap());
        assert_eq!(0.0, calculate("tan(0)").unwrap());
        assert_eq!(0.0, calculate("abs(-0)").unwrap());
        assert_eq!(1.0, calculate("abs(-1)").unwrap());
        assert_eq!(1.0, calculate("round(1.4)").unwrap());
        assert_eq!(120.0, calculate("factorial(5)").unwrap());

        assert!(calculate("error").is_err())
    }
}
