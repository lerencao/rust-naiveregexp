use std::fmt;
use std::fmt::Show;


pub trait Pattern: Show {
  fn precedence(&self) -> uint;

  fn bracket(&self, outer_precedence: uint) -> String {
    if self.precedence() < outer_precedence {
      format!("({})", self)
    } else {
      format!("{}", self)
    }
  }

  fn inspect(&self) -> String {
    format!("/{}/", self)
  }
}


pub struct Empty;

impl Show for Empty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}

impl Pattern for Empty {
  fn precedence(&self) -> uint { 3 }
}


pub struct Literal {
  pub character: char
}

impl Show for Literal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.character.fmt(f)
  }
}

impl Pattern for Literal {
  fn precedence(&self) -> uint { 3 }
}


/// Concatenate node
pub struct Concatenate<S, T> {
  pub first: S,
  pub second: T
}

/// implement Show trait for Concatenate
impl<S: Show, T: Show> Show for Concatenate<S, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}|{}", self.first, self.second)
  }
}

impl<S, T> Pattern for Concatenate<S, T>
  where S: Show + Pattern, T: Show + Pattern {
  fn precedence(&self) -> uint { 1 }
}



#[cfg(test)]
mod test {
  use super::Pattern;
  use super::Empty;
  use super::Literal;
  use super::Concatenate;

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

  #[test]
  fn test_concatenate() {
    let literal = Literal { character: 'a' };
    let concat = Concatenate { first: Empty, second: literal };

    assert_eq!(concat.precedence(), 1);
    assert_eq!(concat.to_string().as_slice(), "|a");
  }
}
