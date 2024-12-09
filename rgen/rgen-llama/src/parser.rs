use crate::{Ast, BlockName, Layer, LayerKey, Orientation};

pub struct Parser<'a> {
  input: &'a str,
  pos:   usize,

  seen_orientation: bool,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self { Parser { input, pos: 0, seen_orientation: false } }

  pub fn parse(&mut self, ast: &mut Ast) {
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
        if !self.seen_orientation {
          self.err("missing orientation declaration");
        }

        break;
      }

      let word = self.next_word();
      match word.as_str() {
        "layer" => self.parse_layer(ast),
        "repeat" => self.parse_repeat(ast),
        "orientation" => self.parse_orientation(ast),
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

  fn parse_layer(&mut self, ast: &mut Ast) {
    self.skip_whitespace();
    let name = self.next_word_opt();

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

    // Rows in the file are defined top-down, but everything else wants them
    // bottom-up.
    rows.reverse();

    let height = rows.len() as u32;

    let mut blocks = vec![];
    for row in rows {
      for i in 0..width {
        blocks.push(row.get(i as usize).copied().unwrap_or(' '));
      }
    }

    let key = match name {
      Some(name) => {
        if ast.layers.contains_key(&LayerKey::Name(name.clone())) {
          self.err(format!("duplicate layer '{name}'"));
        }

        LayerKey::Name(name)
      }
      None => LayerKey::Ord(ast.ordered.len() as u64),
    };
    ast.layers.insert(key.clone(), Layer { width, height, blocks });
    ast.ordered.push(key);
  }

  fn parse_repeat(&mut self, ast: &mut Ast) {
    self.skip_whitespace();

    let layer = self.next_word();
    let key = LayerKey::Name(layer.clone());

    if !ast.layers.contains_key(&key) {
      self.err(format!("unknown layer '{layer}'"));
    }

    ast.ordered.push(key);
  }

  fn parse_orientation(&mut self, ast: &mut Ast) {
    self.skip_whitespace();

    let orientation = self.next_word();

    if self.seen_orientation {
      self.err("duplicate orientation declaration");
    }

    match orientation.as_str() {
      "vertical" => ast.orientation = Orientation::Vertical,
      "horizontal" => ast.orientation = Orientation::Horizontal,
      _ => self.err(format!("unknown orientation '{orientation}'")),
    }

    self.seen_orientation = true;
  }

  fn parse_name(&mut self, ast: &mut Ast, name: char) {
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
  fn next_word_opt(&mut self) -> Option<String> {
    let start = self.pos;
    while matches!(self.peek(), 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') {
      self.next();
    }

    if start == self.pos {
      None
    } else {
      Some(self.input[start..self.pos].into())
    }
  }

  #[track_caller]
  fn next_word(&mut self) -> String {
    self.next_word_opt().unwrap_or_else(|| self.err("expected word"))
  }

  fn next_number(&mut self) -> u32 {
    let start = self.pos;
    while self.peek().is_ascii_digit() {
      self.next();
    }
    self.input[start..self.pos].parse().unwrap()
  }

  fn peek(&self) -> char { self.input[self.pos..].chars().next().unwrap_or('\0') }

  fn skip_whitespace(&mut self) {
    while self.peek().is_whitespace() && self.peek() != '\n' {
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
