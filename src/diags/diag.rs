use crate::diags::textspan::TextSpan;


#[derive(Clone)]
pub struct Diag {
  pub span: TextSpan,
  pub msg: String
}


impl Diag {
  pub fn new(span: TextSpan, msg: String) -> Self {
    Self { span, msg }
  }
}
