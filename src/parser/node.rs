use crate::lexer::token::Token;


#[derive(Debug, Clone)]
pub enum NodeKind {
  Literal,
  Unary,
  Binary
}

#[derive(Debug, Clone)]
pub struct Node {
  // For unary operations the `left` holds the operand
  // and the `op` the operation.

  // For a literal value (number) the `val` holds it.

  pub kind: NodeKind,

  pub left: Option<Box<Node>>,
  pub op: Option<Token>,
  pub right: Option<Box<Node>>,
  pub val: Option<Token>,

  pub pos: usize,
  pub len: usize
}

impl Node {
  pub fn new_lit(token: Token) -> Self {
    Self {
      kind: NodeKind::Literal,

      left: None,
      op: None,
      right: None,
      val: Some(token.clone()),

      pos: token.pos,
      len: token.len
    }
  }

  pub fn new_unary(operand: Node, op: Token) -> Self {
    let len = if operand.pos < op.pos {
      (op.pos + op.len) - operand.pos
    } else {
      (operand.pos + operand.len) - op.pos
    };

    Self {
      kind: NodeKind::Unary,

      left: Some(Box::new(operand.clone())),
      op: Some(op.clone()),
      right: None,
      val: None,

      pos: if operand.pos < op.pos { operand.pos } else { op.pos },
      len
    }
  }

  pub fn new_binary(left: Node, op: Token, right: Node) -> Self {
    Self {
      kind: NodeKind::Binary,

      left: Some(Box::new(left.clone())),
      op: Some(op),
      right: Some(Box::new(right.clone())),
      val: None,

      pos: left.pos,
      len: (right.pos + right.len) - left.pos
    }
  }
}
