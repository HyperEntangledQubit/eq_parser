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
	pub fn new(s: &str) -> Self {
		Tokenizer {
			it: s.chars().peekable(),
		}
	}
}
