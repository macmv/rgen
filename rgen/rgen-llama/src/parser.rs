use crate::{BlockName, Layer, AST};

pub struct Parser<'a> {
  input: &'a str,
  pos:   usize,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Parser { Parser { input, pos: 0 } }

  pub fn parse(&mut self, ast: &mut AST) {
    loop {
      if self.peek() == '\n' {
        self.next();
        continue;
      }

      if self.peek() == '#' {
        while self.peek() != '\n' {
          self.next();
        }
        continue;
      }

      if self.peek() == '\0' {
        break;
      }

      let word = self.next_word();
      match word.as_str() {
        "layer" => self.parse_layer(ast),
        "repeat" => self.parse_repeat(ast),
        _ => {
          if word.len() != 1 {
            self.err("expected single character");
          }

          self.parse_name(ast, word.chars().next().unwrap());
        }
      }

      if self.peek() == '\n' {
        self.next();
      } else if self.pos == self.input.len() {
        break;
      } else {
        self.err("expected newline");
      }
    }
  }

  fn parse_layer(&mut self, ast: &mut AST) {
    self.skip_whitespace();
    let name = self.next_word();

    let mut width = 0;
    let mut rows = vec![];

    if self.peek() != '\n' {
      self.err("expected newline");
    }

    'outer: loop {
      let mut row = vec![];
      loop {
        match self.next() {
          '\n' => break,
          '=' if self.peek() == '=' => {
            self.next();
            break 'outer;
          }
          c => {
            if c != ' ' && !ast.names.contains_key(&c) {
              self.err(format!("unknown block '{c}'"));
            }

            row.push(c);
          }
        }

        match self.next() {
          ' ' => {}
          '\n' => break,
          _ => self.err("expected space or newline"),
        }
      }

      if width < row.len() as u32 {
        width = row.len() as u32;
      }
      rows.push(row);
    }

    let mut blocks = vec![];
    for row in rows {
      for i in 0..width {
        blocks.push(row.get(i as usize).copied().unwrap_or(' '));
      }
    }

    ast.layers.insert(
      name.to_string(),
      Layer { name: name.to_string(), width, height: blocks.len() as u32, blocks },
    );
    ast.ordered.push(name.to_string());
  }

  fn parse_repeat(&mut self, ast: &mut AST) {
    self.skip_whitespace();

    let layer = self.next_word();

    if ast.layers.get(&layer).is_none() {
      self.err(format!("unknown layer '{layer}'"));
    }

    ast.ordered.push(layer.to_string());
  }

  fn parse_name(&mut self, ast: &mut AST, name: char) {
    self.skip_whitespace();
    if self.next() != ':' {
      self.err("expected `:`");
    }
    self.skip_whitespace();

    let category = self.next_word();
    if self.next() != ':' {
      self.err("expected `:`");
    }
    let block = self.next_word();

    let state = if self.peek() == '[' {
      self.next();
      let state = self.next_number();
      self.next();
      Some(state)
    } else {
      None
    };

    ast.names.insert(name, BlockName { category, block, state });
  }

  fn next(&mut self) -> char {
    let ch = self.peek();
    self.pos += ch.len_utf8();
    ch
  }

  #[track_caller]
  fn next_word(&mut self) -> String {
    let start = self.pos;
    while matches!(self.peek(), 'a'..='z' | 'A'..='Z' | '_') {
      self.next();
    }

    if start == self.pos {
      self.err("expected word");
    }

    self.input[start..self.pos].into()
  }

  fn next_number(&mut self) -> u32 {
    let start = self.pos;
    while self.peek().is_digit(10) {
      self.next();
    }
    self.input[start..self.pos].parse().unwrap()
  }

  fn peek(&self) -> char { self.input[self.pos..].chars().next().unwrap_or('\0') }

  fn skip_whitespace(&mut self) {
    while self.peek().is_whitespace() {
      self.next();
    }
  }

  #[track_caller]
  fn err(&self, msg: impl Into<String>) -> ! {
    let line = self.input[..self.pos].lines().count();
    let col = self.input[..self.pos].lines().last().unwrap().len();

    panic!("{} at {line}:{col}", msg.into());
  }
}
