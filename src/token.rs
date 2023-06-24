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

pub fn num_token(it: &mut IT, c:char) -> i64 {
	// Subtract 0 from char after cast as i64
	let mut result = c as i64 - 48; // 48 here is 0 in ASCII
	// Peek because not sure how long the number is.
	// Could be some other operator
	while let Some(c) = it.peek() {
		if is_digit(*c) {
			result = result * 10 + *c as i64 - 48;
		} else {
			return result;
		}
		it.next(); // Need this because peek wont move iterator forward
							 // resulting in inf loop
	}
	result
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
				v if is_digit(v) => return Some(Ok(Token::Num(
							num_token(&mut self.it, v)))),
				c => return Some(Err(format!("unexpected '{}'", c))),
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_tokenizer() {
		let mut tk  = Tokenizer::new("38+12       + (34*2)");
		assert_eq!(tk.next(), Some(Ok(Token::Num(38))));
		assert_eq!(tk.next(), Some(Ok(Token::Add)));
		assert_eq!(tk.next(), Some(Ok(Token::Num(12))));
		assert_eq!(tk.next(), Some(Ok(Token::Add)));
		assert_eq!(tk.next(), Some(Ok(Token::BrOpen)));
		assert_eq!(tk.next(), Some(Ok(Token::Num(34))));
		assert_eq!(tk.next(), Some(Ok(Token::Mul)));
		assert_eq!(tk.next(), Some(Ok(Token::Num(2))));
		assert_eq!(tk.next(), Some(Ok(Token::BrClose)));
	}
}
