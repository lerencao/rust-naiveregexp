pub trait Pattern: ToString {
  fn precedence(&self) -> uint;

  fn bracket(&self, outer_precedence: uint) -> String {
    if self.precedence() < outer_precedence {
      format!("({})", self.to_string())
    } else {
      self.to_string()
    }
  }

  fn inspect(&self) -> String {
    format!("/{}/", self.to_string())
  }
}

#[deriving(Show)]
pub struct Empty;

impl Empty {
  // override the ToString
  fn to_string(&self) -> String {
    "".to_string()
  }
}

impl Pattern for Empty {
  fn precedence(&self) -> uint { 3 }
}

#[deriving(Show)]
pub struct Literal {
  pub character: char
}

impl Literal {
  fn to_string(&self) -> String {
    String::from_char(1, self.character)
  }
}

impl Pattern for Literal {
  fn precedence(&self) -> uint { 3 }
}

#[cfg(test)]
mod test {
  use super::Pattern;
  use super::Empty;
  use super::Literal;

  #[test]
  fn test_empty() {
    assert_eq!(Empty.precedence(), 3);
    assert_eq!(Empty.to_string().as_slice(), "");
  }

  #[test]
  fn test_literal() {
    let literal = Literal { character: 'a' };

    assert_eq!(literal.precedence(), 3);
    assert_eq!(literal.to_string().as_slice(), "a");
  }
}
