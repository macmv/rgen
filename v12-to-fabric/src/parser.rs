use std::ops::Range;

pub struct Parser<'a> {
  pub src: &'a str,
  prev:    usize,
  pos:     usize,
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

  FabricComment,
  V12Comment,
}

impl<'a> Parser<'a> {
  fn char(&self) -> Option<char> { self.src[self.pos..].chars().next() }
  fn advance(&mut self) -> Option<char> {
    let c = self.char();
    if let Some(c) = c {
      self.pos += c.len_utf8();
    }
    c
  }

  fn skip_whitespace(&mut self) -> Option<Token> {
    let mut fabric_comment = false;
    let mut v12_comment = false;
    let start = self.pos;

    'outer: while let Some(c) = self.char() {
      match c {
        '/' if self.src[self.pos + c.len_utf8()..].chars().next() == Some('/') => {
          // Skip the `//`
          self.pos += 2;
          let comment_start = self.pos;

          while let Some(c) = self.char() {
            if self.src[comment_start..self.pos].trim() == "#v12-start" {
              v12_comment = true;
              break 'outer;
            }

            if c == '\n' {
              self.pos += c.len_utf8();
              break;
            }
            self.pos += c.len_utf8();
          }

          continue;
        }

        '/' if self.src[self.pos + c.len_utf8()..].chars().next() == Some('*') => {
          // Skip the `/*`
          self.pos += 2;

          while let Some(c) = self.char() {
            if self.src[start..self.pos].trim() == "/* #fabric:" {
              fabric_comment = true;
            }

            if c == '*' && self.src[self.pos + c.len_utf8()..].chars().next() == Some('/') {
              self.pos += 2;
              break;
            }
            self.pos += c.len_utf8();
          }

          continue;
        }

        _ => {}
      }

      if !c.is_whitespace() {
        break;
      }
      self.pos += c.len_utf8();
    }

    if fabric_comment {
      return Some(Token::FabricComment);
    }

    if v12_comment {
      while let Some(c) = self.char() {
        match c {
          '/' if self.src[self.pos + c.len_utf8()..].chars().next() == Some('/') => {
            // Skip the `//`
            self.pos += 2;
            let comment_start = self.pos;

            while let Some(c) = self.char() {
              if self.src[comment_start..self.pos].trim() == "#v12-end" {
                self.skip_whitespace();
                return Some(Token::V12Comment);
              }

              if c == '\n' {
                self.pos += c.len_utf8();
                break;
              }
              self.pos += c.len_utf8();
            }

            continue;
          }

          _ => self.pos += c.len_utf8(),
        }
      }

      panic!("no #v12-end comment found");
    }

    None
  }

  pub const fn range(&self) -> Range<usize> { self.prev..self.pos }
  pub fn slice(&self) -> &'a str { &self.src[self.range()] }

  pub fn next(&mut self) -> Option<Token> {
    let start = self.pos;
    if let Some(t) = self.skip_whitespace() {
      self.prev = start;
      return Some(t);
    }
    self.prev = self.pos;

    match self.advance()? {
      'a'..='z' | 'A'..='Z' => {
        while let Some(c) = self.char() {
          if c.is_alphanumeric() || c == '_' {
            self.advance();
          } else {
            break;
          }
        }

        Some(Token::Word)
      }

      '0'..='9' => {
        while let Some(c) = self.char() {
          if c.is_digit(10) || c == '.' || c == '_' {
            self.advance();
          } else {
            break;
          }
        }

        Some(Token::Number)
      }

      '=' | ';' | '.' | ',' | '{' | '}' | '[' | ']' | '(' | ')' | '@' | ':' | '&' | '!' | '|'
      | '?' | '+' | '-' | '*' | '/' | '%' | '^' | '<' | '>' => Some(Token::Punct),

      '"' => {
        while let Some(c) = self.char() {
          if c == '"' {
            self.advance();
            return Some(Token::String);
          } else if c == '\\' {
            // TODO: Multi-character escapes
            self.advance();
          }
          self.advance();
        }

        panic!("unclosed string literal");
      }

      c => panic!("unknown character '{c}'"),
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
