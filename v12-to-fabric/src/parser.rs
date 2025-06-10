pub struct Parser<'a> {
  src:  &'a str,
  prev: usize,
  pos:  usize,
}

impl<'a> Parser<'a> {
  pub fn new(src: &'a str) -> Self { Parser { src, prev: 0, pos: 0 } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
  Word,
  String,
  Number,
  Punct,
}

impl<'a> Parser<'a> {
  fn char(&self) -> Option<char> { self.src[self.pos..].chars().next() }
  fn advance(&mut self) {
    if let Some(c) = self.char() {
      self.pos += c.len_utf8();
    }
  }
  fn ok(&mut self, token: Token) -> Option<Token> {
    self.advance();
    Some(token)
  }

  fn skip_whitespace(&mut self) {
    while let Some(c) = self.char() {
      if !c.is_whitespace() {
        break;
      }
      self.pos += c.len_utf8();
    }
  }

  pub fn slice(&self) -> &'a str { &self.src[self.prev..self.pos] }

  pub fn next(&mut self) -> Option<Token> {
    self.skip_whitespace();
    self.prev = self.pos;

    match self.char()? {
      'a'..='z' | 'A'..='Z' => {
        self.advance();

        while let Some(c) = self.char() {
          if c.is_alphanumeric() || c == '_' {
            self.advance();
          } else {
            break;
          }
        }

        Some(Token::Word)
      }

      '=' | ';' | '.' => self.ok(Token::Punct),

      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_name() {
    let mut parser = Parser::new("import foo.Bar;");

    assert_eq!(parser.next(), Some(Token::Word));
    assert_eq!(parser.slice(), "import");
    assert_eq!(parser.next(), Some(Token::Word));
    assert_eq!(parser.slice(), "foo");
    assert_eq!(parser.next(), Some(Token::Punct));
    assert_eq!(parser.slice(), ".");
    assert_eq!(parser.next(), Some(Token::Word));
    assert_eq!(parser.slice(), "Bar");
    assert_eq!(parser.next(), Some(Token::Punct));
    assert_eq!(parser.slice(), ";");
    assert_eq!(parser.next(), None);
  }
}
