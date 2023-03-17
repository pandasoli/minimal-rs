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
  let mut text_builder = String::new();

  loop {
    // Here I can check if the `text_builder` is empty or not,
    // If it is, it means this is the first time the user is inputing something.
    // If it is not empty it means the user inputed some thing incomplete.
    print!("\x1b[1m›\x1b[0m ");
    std::io::stdout().flush().expect("stdout flush failed");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin read failed");

    input = input.trim().to_string();
    let is_blank = input.is_empty();

    if text_builder.is_empty() {
      if is_blank {
        break
      }
      else if input == "#show tokens" {
        show_tokens = !show_tokens;

        let color = if show_ast { "\x1b[32m" } else { "\x1b[33m" };
        let state = if show_ast { "Showing" } else { "Not showing" };
        println!("  🦴 {}{} lex tokens.\x1b[0m", color, state);

        continue;
      }
      else if input == "#show tree" {
        show_ast = !show_ast;

        let color = if show_ast { "\x1b[32m" } else { "\x1b[33m" };
        let state = if show_ast { "Showing" } else { "Not showing" };
        println!("  ☠️ {}{} parse trees.\x1b[0m", color, state);

        continue;
      }
    }

    text_builder.push_str(&input);
    let text = text_builder.to_string();

    let mut lex = Lexer::new(text.clone());
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = lex.next() {
      tokens.push(token);
    }

    let mut par = Parser::new(tokens.clone());
    let ast = par.parse();

    if !is_blank && par.diags().len() > 0 {
      continue;
    }

    let mut diags = lex.diags().clone();
    diags.extend(par.diags());

    if show_tokens {
      for token in tokens {
        println!("{:?}", token);
      }

      println!("");
    }

    if show_ast {
      println!("{:?}\n", ast);
    }

    if diags.len() > 0 {
      for diag in lex.diags() {
        println!("");

        println!("\x1b[31m{}\x1b[0m", diag.msg);

        let prefix = &text[0..diag.span.start];
        let error  = &text[diag.span.start..diag.span.end];
        let suffix = &text[diag.span.end..diag.span.end + input.len() - diag.span.end];

        println!("  ╰─ {}\x1b[31m{}\x1b[0m{}", prefix, error, suffix);
      }
    }

    text_builder.clear();
  }
}
