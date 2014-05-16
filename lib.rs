#![crate_id = "keyargs"]
#![crate_type = "dylib"]
#![feature(quote, managed_boxes, macro_registrar, macro_rules)]

extern crate syntax;
use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{
    SyntaxExtension, ExtCtxt, MacResult, MacExpr,
    NormalTT, BasicMacroExpander,
};
use syntax::ext::build::AstBuilder;
use syntax::parse;
use syntax::parse::{parser, token, common};

#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
  // Change this to ident macro once the base works
  let expander = box BasicMacroExpander { expander: keyargs, span: None };
  register(token::intern("keyargs"), NormalTT(expander, None))
}

fn parse_expr(parser: &mut parser::Parser) -> @ast::Expr {
  parser.parse_expr()
}

fn keyargs(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
  use syntax::ast::TTDelim;
  let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(),
                                              Vec::from_slice(tts));

  let exprs = parser.parse_seq_to_end(&token::EOF,
                                      common::seq_sep_trailing_disallowed(token::COMMA),
                                      parse_expr);
  for arg in exprs.iter() {
    match cx.expand_expr(*arg).node {
      ast::ExprAssign(name, val) => {
        cx.span_note(name.span, format!("{:?}", name));
        cx.span_note(val.span, format!(" -> {:?}", val));
        match name.node {
          ast::ExprPath(ref path) => cx.span_note(path.span, format!("{:?}", path)),
          _ => cx.span_err(name.span,
                           format!("expected argument name but found expression `{}`",
                                   syntax::print::pprust::expr_to_str(name))),

        }
      }
      _ => (),
    }
  }

  MacExpr::new(quote_expr!(cx, println!("test")))
}

pub fn test(mandatory: &str, opt1: Option<&'static str>, opt2: Option<int>) {
  println!("Got {}, {}, {}", mandatory, opt1, opt2);
}

