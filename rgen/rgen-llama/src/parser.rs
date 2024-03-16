use crate::AST;

pub struct Parser<'a> {
  input: &'a str,
  pos:   usize,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Parser { Parser { input, pos: 0 } }

  pub fn parse(&mut self, ast: &mut AST) { todo!() }
}
