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
      let op = self.read();
      let operand = self.binary_expr(unary_op_prece);

      Node::new_unary(operand, op)
    }
    else {
      self.prim_expr()
    };

    loop {
      let prece = Token::get_binary_op_precedence(self.current().kind);
      if prece == 0 || prece <= parent_prece {
        break;
      }

      let op = self.read();
      let right = self.binary_expr(prece);

      left = Node::new_binary(left, op, right);
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

      _ => Node::new_lit(self.match_(TokenKind::Number))
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
        Node::new_binary(
          Node::new_binary(
            Node::new_lit(Token::new(TokenKind::Number, "123".to_owned(), 0, 3)),
            Token::new(TokenKind::Star, "*".to_owned(), 3, 1),
            Node::new_unary(
              Node::new_binary(
                Node::new_lit(Token::new(TokenKind::Number, "12".to_owned(), 8, 2)),
                Token::new(TokenKind::Slash, "/".to_owned(), 10, 1),
                Node::new_unary(
                  Node::new_lit(Token::new(TokenKind::Number, "2".to_owned(), 14, 1)),
                  Token::new(TokenKind::Minus, "-".to_owned(), 12, 1)
                )
              ),
              Token::new(TokenKind::Minus, "-".to_owned(), 5, 1)
            )
          ),
          Token::new(TokenKind::Plus, "+".to_owned(), 15, 1),
          Node::new_lit(Token::new(TokenKind::Number, "6".to_owned(), 17, 1))
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
    match node.kind {
      NodeKind::Literal => {
        let val = node.val.unwrap();
        let exp_val = expected.val.unwrap();

        assert_eq!(val.kind, exp_val.kind);
        assert_eq!(val.literal, exp_val.literal);
        assert_eq!(node.pos, expected.pos);
        assert_eq!(node.len, expected.len);
      },
      NodeKind::Unary => {
        let expr = node.left.unwrap();
        let op = node.op.unwrap();

        let exp_expr = expected.left.unwrap();
        let exp_op = expected.op.unwrap();

        test_node(*expr, *exp_expr);
        assert_eq!(op.kind, exp_op.kind);
      },
      NodeKind::Binary => {
        let left = node.left.unwrap();
        let op = node.op.unwrap();
        let right = node.right.unwrap();

        let exp_left = expected.left.unwrap();
        let exp_op = expected.op.unwrap();
        let exp_right = expected.right.unwrap();

        test_node(*left, *exp_left);
        test_node(*right, *exp_right);
        assert_eq!(op.kind, exp_op.kind);
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
