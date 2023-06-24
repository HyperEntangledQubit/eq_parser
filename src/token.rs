use std::str::Chars;
use std::iter::Peekable;

type IT<'a> = Peekable<Chars<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	BrOpen,
	BrClose,
	Add,
	Sub,
	Div,
	Mul,
	Num(i64),
}

pub struct Tokenizer<'a> {
	it: IT<'a>,
}

impl<'a> Tokenizer<'a> {
	pub fn new(s: &'a str) -> Self {
		Tokenizer {
			it: s.chars().peekable(),
		}
	}
}

fn is_digit(c: char) -> bool {
	c >= '0' && c <= '9'
}

impl <'a> Iterator for Tokenizer<'a> {
	// Using String error here to be lazy.
	type Item = Result<Token, String>;
	fn next(&mut self) -> Option<Self::Item> {
		while let Some(c) = self.it.next() {
			match c {
				' ' | '\n' | '\t' => {}
				'(' => return Some(Ok(Token::BrOpen)),
				')' => return Some(Ok(Token::BrClose)),
				'+' => return Some(Ok(Token::Add)),
				'-' => return Some(Ok(Token::Sub)),
				'/' => return Some(Ok(Token::Div)),
				'*' => return Some(Ok(Token::Mul)),
				v if is_digit(v) => {},
				c => return Some(Err(format!("unexpected '{}'", c))),
			}
		}
		None
	}
}
