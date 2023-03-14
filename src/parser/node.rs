use crate::lexer::token::Token;


#[derive(Debug, PartialEq)]
pub enum UnaryOp {
  Positive,
  Negative
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
  Plus,
  Subtract,
  Multiply,
  Divide
}

#[derive(Debug)]
pub enum Node {
  Number(Token),
  Binary(Box<Node>, BinaryOp, Box<Node>),
  Unary(Box<Node>, UnaryOp)
}
