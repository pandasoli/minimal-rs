use crate::{
  lexer::token::{ Token, TokenKind },
  diags::{
    diag::Diag,
    diagbag::DiagBag
  },
  parser::node::*
};


pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
  diags: DiagBag
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      pos: 0,
      diags: DiagBag::new()
    }
  }

  pub fn diags(&self) -> Vec<Diag> {
    self.diags.diags.clone()
  }

  fn peek(&self, offset: usize) -> Token {
    let i = self.pos + offset;

    if i >= self.tokens.len() {
      return self.tokens[self.tokens.len() - 1].clone();
    }

    self.tokens[i].clone()
  }

  fn current(&self) -> Token { self.peek(0) }

  fn read(&mut self) -> Token {
    let current = self.current();
    self.pos += 1;
    current
  }

  fn match_(&mut self, kind: TokenKind) -> Token {
    if self.current().kind == kind {
      return self.read();
    }

    self.diags.report_unexpected_token(self.current().span, self.current().kind, kind.clone());
    Token::new(kind, "".to_owned(), self.current().pos, 0)
  }

  pub fn parse(&mut self) -> Node {
    self.binary_expr(0)
  }

  fn binary_expr(&mut self, parent_prece: i32) -> Node {
    let unary_op_prece = Token::get_unary_op_precedence(self.current().kind);

    let mut left = if unary_op_prece != 0 && unary_op_prece >= parent_prece {
      let op = match self.read().kind {
        TokenKind::Plus => UnaryOp::Positive,
        TokenKind::Minus => UnaryOp::Negative,

        _ => unimplemented!()
      };

      let operand = self.binary_expr(unary_op_prece);
      Node::Unary (Box::new(operand), op)
    }
    else {
      self.prim_expr()
    };

    loop {
      let prece = Token::get_binary_op_precedence(self.current().kind);
      if prece == 0 || prece <= parent_prece {
        break;
      }

      let op = match self.read().kind {
        TokenKind::Plus => BinaryOp::Plus,
        TokenKind::Minus => BinaryOp::Subtract,
        TokenKind::Star => BinaryOp::Multiply,
        TokenKind::Slash => BinaryOp::Divide,

        _ => unimplemented!()
      };

      let right = self.binary_expr(prece);
      left = Node::Binary (Box::new(left), op, Box::new(right));
    }

    return left;
  }

  fn prim_expr(&mut self) -> Node {
    match self.current().kind {
      TokenKind::OpenParen => {
        self.read();
        let expr = self.binary_expr(0);
        self.match_(TokenKind::CloseParen);

        expr
      },

      _ => Node::Number (self.match_(TokenKind::Number))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lexer::lexer::Lexer;

  #[test]
  fn tokens() {
    let suite = vec![
      (
        "123 * -(12 / -2)+6",
        Node::Binary (
          Box::new(Node::Binary (
            Box::new(Node::Number(Token::new(TokenKind::Number, "123".to_owned(), 0, 3))),
            BinaryOp::Multiply,
            Box::new(Node::Unary (
              Box::new(Node::Binary (
                Box::new(Node::Number (Token::new(TokenKind::Number, "12".to_owned(), 8, 2))),
                BinaryOp::Divide,
                Box::new(Node::Unary (
                  Box::new(Node::Number (Token::new(TokenKind::Number, "2".to_owned(), 14, 1))),
                  UnaryOp::Negative
                ))
              )),
              UnaryOp::Negative
            ))
          )),
          BinaryOp::Plus,
          Box::new(Node::Number(Token::new(TokenKind::Number, "6".to_owned(), 17, 1)))
        ),
        0
      )
    ];

    for case in suite {
      let tokens = lexall(case.0).0;

      let mut par = Parser::new(tokens);
      let ast = par.parse();

      assert_eq!(par.diags().len(), case.2);
      test_node(ast, case.1);
    }
  }

  fn test_node(node: Node, expected: Node) {
    match node {
      Node::Number(token) => {
        let expected = match expected {
          Node::Number(token) => token,
          _ => unimplemented!()
        };

        assert_eq!(token.kind, expected.kind);
        assert_eq!(token.literal, expected.literal);
        assert_eq!(token.pos, expected.pos);
        assert_eq!(token.len, expected.len);
      },
      Node::Unary(node, op) => {
        let (exp_node, exp_op) = match expected {
          Node::Unary(node, op) => (node, op),
          _ => unimplemented!()
        };

        test_node(*node, *exp_node);
        assert_eq!(op, exp_op);
      },
      Node::Binary(left, op, right) => {
        let (exp_left, exp_op, exp_right) = match expected {
          Node::Binary(node, op, right) => (node, op, right),
          _ => unimplemented!()
        };

        test_node(*left, *exp_left);
        test_node(*right, *exp_right);
        assert_eq!(op, exp_op);
      }
    }
  }

  fn lexall(text: &str) -> (Vec<Token>, Vec<Diag>) {
    let mut lex = Lexer::new(text.to_owned());
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = lex.next() {
      tokens.push(token);
    }

    (tokens, lex.diags())
  }
}
