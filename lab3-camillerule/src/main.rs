// from lab specs
use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}
// end of code from lab

// required functions

pub fn eval(expr: Expr) -> Value {
  match expr {
    ArithExpr(arith_expr) => IntValue(eval_arith_expr(arith_expr)), 
    BoolExpr(bool_expr) => BoolValue(eval_bool_expr(bool_expr)), 
  }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
  match arith_expr{
    IntLit(x) => x,
    BinArithExpr{left, right, op} => {
      let left_expr = eval_arith_expr(*left);
      let right_expr = eval_arith_expr(*right);
      match op {
        AddOp => left_expr + right_expr, 
        SubOp => left_expr - right_expr, 
        MulOp => left_expr * right_expr, 
        IntDivOp => left_expr / right_expr, 
      }
    }
  }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
  match bool_expr {
    BoolLit(y) => y,
    NotExpr(expr) => !eval_bool_expr(*expr), 
    ArithCmpExpr {left, right, op} => { 
      let left_expr = eval_arith_expr(*left); 
      let right_expr = eval_arith_expr(*right); 
      match op { 
        LtOp => left_expr < right_expr, 
        LteOp => left_expr <= right_expr, 
        GtOp => left_expr > right_expr, 
        GteOp => left_expr >= right_expr, 
        ArithEqOp => left_expr == right_expr, 
        ArithNeqOp => left_expr != right_expr, 
      } 
    }
    BinBoolExpr {left, right, op} => { 
      let left_expr = eval_bool_expr(*left); 
      let right_expr = eval_bool_expr(*right); 
      match op { 
        AndOp => left_expr && right_expr, 
        OrOp => left_expr || right_expr, 
        BoolEqOp => left_expr == right_expr, 
        BoolNeqOp => left_expr != right_expr, 
      } 
    }
  }
}
// end of required functions

fn main() {}

// start of tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_one() {
    // 10 < 6 = True
    // (6 + 4) < (2 * 3)
    let left_expr = eval_arith_expr(BinArithExpr{
      left: Box::new(IntLit(6)),
      right: Box::new(IntLit(4)),
      op: AddOp,
    }); 

    let right_expr = eval_arith_expr(BinArithExpr{
      left: Box::new(IntLit(2)),
      right: Box::new(IntLit(3)),
      op: MulOp,
    });

    let expr = BoolExpr(ArithCmpExpr{
      left: Box::new(IntLit(left_expr)),
      right: Box::new(IntLit(right_expr)),
      op: LtOp,
    });

    let answer = BoolValue(false);

    assert_eq!(eval(expr), answer); 
  }

  #[test]
  fn test_two() {
    // 2 == 2 = True
    // (6 - 4) == (6 / 3)
    let left_expr = eval_arith_expr(BinArithExpr{
      left: Box::new(IntLit(6)),
      right: Box::new(IntLit(4)),
      op: SubOp,
    }); 

    let right_expr = eval_arith_expr(BinArithExpr{
      left: Box::new(IntLit(6)),
      right: Box::new(IntLit(3)),
      op: IntDivOp,
    });

    let expr = BoolExpr(ArithCmpExpr{
      left: Box::new(IntLit(left_expr)),
      right: Box::new(IntLit(right_expr)),
      op: ArithEqOp,
    });

    let answer = BoolValue(true);

    assert_eq!(eval(expr), answer); 
  }

  #[test]
  fn test_three() {
    // T or F = T
    // (6 > 4) or (6 < 4)
    let left_expr = eval_bool_expr(ArithCmpExpr{
      left: Box::new(IntLit(6)),
      right: Box::new(IntLit(4)),
      op: GteOp,
    }); 

    let right_expr = eval_bool_expr(ArithCmpExpr{
      left: Box::new(IntLit(6)),
      right: Box::new(IntLit(4)),
      op: LteOp,
    });

    let expr = BoolExpr(BinBoolExpr{
      left: Box::new(BoolLit(left_expr)),
      right: Box::new(BoolLit(right_expr)),
      op: OrOp,
    });

    let answer = BoolValue(true);

    assert_eq!(eval(expr), answer);  
  }

  #[test]
  fn test_four() {
    // T and ~T = false
    // (6 > 4) and !(6 != 4)
    let left_expr = eval_bool_expr(ArithCmpExpr{
      left: Box::new(IntLit(6)),
      right: Box::new(IntLit(4)),
      op: GtOp,
    }); 

    let right_expr = eval_bool_expr(ArithCmpExpr{
      left: Box::new(IntLit(6)),
      right: Box::new(IntLit(4)),
      op: ArithNeqOp,
    });

    let not_right_expr = eval_bool_expr(NotExpr(Box::new(BoolLit(right_expr))));

    let expr = BoolExpr(BinBoolExpr{
      left: Box::new(BoolLit(left_expr)),
      right: Box::new(BoolLit(not_right_expr)),
      op: AndOp,
    });

    let answer = BoolValue(false);

    assert_eq!(eval(expr), answer); 
  }

  #[test]
  fn test_five() {
    // F == F = true
    // (T == F) == (F != F)
    let left_expr = eval_bool_expr(BinBoolExpr {  
      left: Box::new(BoolLit(true)), 
      right: Box::new(BoolLit(false)), 
      op: BoolEqOp, 
    });  

    let right_expr = eval_bool_expr(BinBoolExpr {  
      left: Box::new(BoolLit(false)), 
      right: Box::new(BoolLit(false)), 
      op: BoolNeqOp, 
    });  

    let expr = BoolExpr(BinBoolExpr{
      left: Box::new(BoolLit(left_expr)),
      right: Box::new(BoolLit(right_expr)),
      op: BoolEqOp,
    });

    let answer = BoolValue(true);

    assert_eq!(eval(expr), answer);
  }

  #[test]
  fn test_six() {
    let expr = ArithExpr(IntLit(0)); 
    let answer = IntValue(0); 

    assert_eq!(eval(expr), answer); 
  }

  #[test]
  fn test_sample() {
    let expr = BoolExpr(BoolLit(true));
    let answer = BoolValue(true);

    assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
  }

  #[test]
  fn test_others() {
    main();
    println!("{:?}", BoolValue(true));
  }
}