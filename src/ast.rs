//
// Created by Justin Tunheim 11/24/24
//

use byteyarn;

pub struct Token {
	pos: usize,
	identifier: Option<byteyarn::Yarn>,
}

pub enum Literal {
	Pointer(usize),
	Integer(i64),
	Float(f32),
	Boolean(bool),

	SmallString(byteyarn::Yarn),
	HeapString(String),
}

pub enum Lexeme {
	If(Token),
}

pub fn truthy(value: Literal) -> bool {
	match value {
		Literal::Pointer(ptr) => {
			if ptr != 0 {
				return true
			}
		},
		Literal::Integer(int) => {
			if int != 0 {
				return true
			}
		},
		Literal::Float(_) => (),
		Literal::Boolean(bool) => return bool,
		Literal::SmallString(_) => (),
		Literal::HeapString(_) => (),
	}
	false
}

impl std::fmt::Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Pointer(ptr) => write!(f, "pointer: {}", ptr),
			Self::Integer(int) => write!(f, "integer: {}", int),
			Self::Float(float) => write!(f, "float: {}", float),
			Self::Boolean(bool) => write!(f, "bool: {}", bool), 
			Self::SmallString(str) => write!(f, "small_str: {}", str),
			Self::HeapString(str) => write!(f, "heap_str: {}", str),
		}
	}
}
