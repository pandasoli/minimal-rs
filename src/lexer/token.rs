use crate::diags::textspan::TextSpan;


#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum TokenKind {
  Illegal,
  EOF,

  // Identifiers and literals
  /// Identifier,
  Number,

  // Operators
  // Assign,
  Plus,
  Minus,
  Star,
  Slash,

  // Delimiters
  OpenParen,
  CloseParen
}


#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub literal: String,
  pub pos: usize,
  pub len: usize,
  pub span: TextSpan
}

impl Token {
  pub fn new(kind: TokenKind, literal: String, pos: usize, len: usize) -> Self {
    Self {
      kind,
      literal,
      pos,
      len,
      span: TextSpan::new(pos, len)
    }
  }
}
