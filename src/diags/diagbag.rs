use crate::diags::diag::Diag;
use crate::diags::textspan::TextSpan;


pub struct DiagBag {
  pub diags: Vec<Diag>
}

impl DiagBag {
  pub fn new() -> Self {
    Self {
      diags: Vec::<Diag>::new()
    }
  }

  fn report(&mut self, span: TextSpan, msg: String) {
    self.diags.push(
      Diag::new(span, msg)
    );
  }


  pub fn report_illegal_ch(&mut self, pos: usize, ch: char) {
    let span = TextSpan::new(pos, 1);
    let msg = format!("Illegal character input: '{}'", ch);
    self.report(span, msg);
  }
}
