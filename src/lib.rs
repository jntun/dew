//
// Created by Justin Tunheim on 11/24/24
//

mod ast;

use byteyarn;

pub enum Error {
	Build(BuildError),
	Execution(ExecutionError),
}

pub enum BuildError {
}

pub enum ExecutionError {
}

pub struct File {
	pub name: byteyarn::Yarn,
}

pub struct Source {
	pub main: File,
	pub secondary: Vec<File>,
}

pub struct SourceBuilder {
	source: Source,
}

impl File {
	pub fn new(name: String) -> Self {
		Self {
			name: byteyarn::Yarn::from(name),
		}
	}
}

impl Source {
	pub fn new(main: File) -> SourceBuilder {
		SourceBuilder {
			source: Source {
				main,
				secondary: Vec::new(),
			}
		}
	}
	
	pub fn execute(&mut self) -> Result<ast::Literal, ExecutionError> {
		Ok(ast::Literal::Integer(0))
	} 
}

impl SourceBuilder {
	pub fn add(&mut self, file: File) -> Result<(), BuildError> {
		self.source.secondary.push(file);
		Ok(())
	}

	pub fn finish(self) -> Result<Source, BuildError> {
		Ok(self.source)
	}
}

impl std::fmt::Display for BuildError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", "build error, fixme")
	}
}

impl std::fmt::Display for ExecutionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", "execution error, fixme")
	}
}
