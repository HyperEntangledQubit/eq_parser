mod token;

use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
	Brackets(Box<Expr>),
	Op(Oper, Box<Expr>, Box<Expr>),
	Num(i64),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Oper {
	Add,
	Sub,
	Div,
	Mul,
}

pub fn op(o: Oper, a: Expr, b: Expr) -> Expr {
	Expr::Op(o, Box::new(a), Box::new(b))
}

impl FromStr for Expr {
	type Err = String;
	fn from_str(_s: &str) -> Result<Expr, String> {
		unimplemented!{}
	}
}
