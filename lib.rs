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
use syntax::parse::token;

#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
    let expander = box BasicMacroExpander { expander: keyargs, span: None };
    register(token::intern("keyargs"), NormalTT(expander, None))
}

fn keyargs(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
  MacExpr::new(quote_expr!(cx, println!("test")))
}

pub fn test(mandatory: &str, opt1: Option<&'static str>, opt2: Option<int>) {
  println!("Got {}, {}, {}", mandatory, opt1, opt2);
}

