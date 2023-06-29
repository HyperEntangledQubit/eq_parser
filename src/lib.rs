mod token;
use std::str::FromStr;
use token::*;

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
        unimplemented! {}
    }
}

pub type ParseResult<'a,T> => Result<(Tokenizer<'a>,T), String>;

pub fn token_bool<'a,F:Fn(&Token)->bool>(t:Tokenizer<'a>, f:F)->ParseResult<'a,
Token> {
    let mut it = t.clone();
    match it.next() {
        Some(Ok(v) if f(&v) => Ok((it, v)),
        _ => Err("Failed in Token bool test".to_string()),
    }
}

pub fn brackets<'a>(Tokenizer<'a>(&Tokenizer<'a>)->ParseResult<'a, Expr> {
    let it = t.clone();

    let (it, _) = token_bool(t, |t| *t == Token::BrOpen)?;
    // Haven't written this yet
    let (it, res) = ___(&it)?;
    let (it, _) = token_bool(t, |t| *t == Token::BrClose)?;
    Ok((it, Expr::Brackets(Box::new(res))))

}
