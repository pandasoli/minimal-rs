use std::io;
use std::io::Write;

mod lexer;
mod parser;
mod diags;

use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::parser::Parser;

fn main() {
  println!("\x1b[1;47;30m  𝙼𝚒𝚗𝚒𝚖𝚊𝚕 - 𝙰𝚗 𝚘𝚕𝚍 𝚗𝚎𝚠 𝚙𝚛𝚘𝚐𝚛𝚊𝚖𝚖𝚒𝚗𝚐 𝚕𝚊𝚗𝚐𝚞𝚊𝚐𝚎 :𝙳  \x1b[0m\n");

  let mut show_tokens = false;
  let mut show_ast = false;

  loop {
    let mut code = String::new();

    print!("\x1b[1m›\x1b[0m ");

    std::io::stdout().flush().expect("stdout flush failed");
    io::stdin().read_line(&mut code).expect("stdin read failed");

    if code == "#show tokens\n".to_string() { show_tokens = true; continue; }
    else if code == "#show ast\n".to_string() { show_ast = true; continue; }

    let mut lex = Lexer::new(code.clone());
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = lex.next() {
      if show_tokens {
        println!("{:?}", token);
      }

      tokens.push(token);
    }

    let mut par = Parser::new(tokens);
    let ast = par.parse();

    let mut diags = lex.diags().clone();
    diags.extend(par.diags());

    if show_ast {
      println!("{:?}", ast);
    }

    if diags.len() > 0 {
      for diag in lex.diags() {
        println!("");

        println!("\x1b[31m{}\x1b[0m", diag.msg);

        let prefix = &code[0..diag.span.start];
        let error  = &code[diag.span.start..diag.span.end];
        let suffix = &code[diag.span.end..diag.span.end + code.len() - diag.span.end];

        println!("  ╰─ {}\x1b[31m{}\x1b[0m{}", prefix, error, suffix);
      }
    }

    if code == "\n" { break }
  }
}
