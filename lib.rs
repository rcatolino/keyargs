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
use syntax::parse::{token, common};
use syntax::print::pprust;

mod llist {
  use syntax::ast::Expr;
  pub enum Llist {
    Cons(uint, @Expr, Box<Llist>),
    Nil,
  }

  impl Llist {
    pub fn insert(self, idx: uint, expr: @Expr) -> Llist {
      loop {
        match self {
          Nil => return Cons(idx, expr, box Nil),
          Cons(lidx, lexpr, next) => match lidx.cmp(&idx) {
            Less => return Cons(lidx, lexpr, box next.insert(idx, expr)),
            Greater => return Cons(idx, lexpr, box Cons(lidx, lexpr, next)),
            Equal => fail!("Option for idx {} has already been inserted!", lidx),
          }
        }
      }
    }
  }
}

#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
  // Change this to ident macro once the base works
  let expander = box BasicMacroExpander { expander: keyargs, span: None };
  register(token::intern("keyargs"), NormalTT(expander, None))
}

struct Function {
  name: @ast::Expr,
  options: ~[Vec<ast::Name>],
  mandatory_nb: uint,
}

fn find_opt(fun: &Function, path: &ast::Path, last_idx: uint) -> Option<uint> {
  for (opt, idx) in fun.options.iter().zip(range(fun.mandatory_nb, last_idx)) {
    if path.segments.iter().map(|seg| seg.identifier.name).collect::<Vec<ast::Name>>() == *opt {
      return Some(idx);
    }
  }

  None
}

fn expand_args(cx: &mut ExtCtxt, sp: codemap::Span, exprs: Vec<@ast::Expr>)
               -> Option<(@ast::Expr, Vec<@ast::Expr>)> {
  use llist::{Nil, Cons};

  // We build the Function ourselves until the function-def synext is coded.
  let fun = Function {
    name: quote_expr!(&mut *cx, testlib::test),
    options: ~[vec!(token::intern("opt1")), vec!(token::intern("opt2"))],
    mandatory_nb: 1,
  };

  let mut key_started = false;
  let mut expanded = Vec::with_capacity(fun.mandatory_nb + fun.options.len());
  let mut named_options = llist::Nil;
  for (arg, consumed) in exprs.iter().zip(range(0, exprs.len())) {
    if consumed < fun.mandatory_nb {
      expanded.push(*arg);
      continue;
    }

    // Add the positional arguments, and build a list of the keyword arguments.
    match cx.expand_expr(*arg).node {
      ast::ExprAssign(name, val) => {
        key_started = true;
        match name.node {
          ast::ExprPath(ref path) => {
            match find_opt(&fun, path, expanded.capacity()) {
              Some(idx) if idx < expanded.len () => {
                cx.span_err(name.span, format!("keyword argument `{}` was already given \
                                               as a positional argument.",
                                               pprust::expr_to_str(name)));
                cx.span_note(expanded.get(idx).span, "corresponding positional argument is");
                return None;
              }
              Some(idx) => named_options = named_options.insert(idx, val),
              None => {
                cx.span_err(name.span, format!("invalid keyword argument `{}`.",
                                               pprust::expr_to_str(name)));
                return None;
              }
            }
          }
          _ => {
            cx.span_err(name.span, format!("expected argument name but found expression `{}`",
                                           pprust::expr_to_str(name)));
            return None;
          }
        }
      }
      _ => if key_started {
        // The keywords are always in last position, we can't have non-keywords params
        // once key_started.
        cx.span_err(arg.span, format!("expected keyword argument but found `{}`",
                              pprust::expr_to_str(*arg)));
        return None;
      } else {
        // Push the expr after wrapping it in a Some()
        expanded.push(cx.expr_some(arg.span, *arg));
      },
    }
  }

  // Add the keyword arguments and the leftovers.
  for i in range(expanded.len(), expanded.capacity()) {
    named_options = match named_options {
      Nil => {
        expanded.push(cx.expr_none(sp));
        Nil
      }
      Cons(idx, expr, box next) => if idx == i {
        expanded.push(cx.expr_some(expr.span, expr));
        next
      } else {
        expanded.push(cx.expr_none(sp));
        Cons(idx, expr, box next)
      },
    }
  }

  Some((fun.name, expanded))
}

fn keyargs(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
  let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(),
                                              Vec::from_slice(tts));

  let exprs = parser.parse_seq_to_end(&token::EOF,
                                      common::seq_sep_trailing_disallowed(token::COMMA),
                                      |p| p.parse_expr());

  match expand_args(cx, sp, exprs) {
    Some((name, expanded)) => {
      let call = cx.expr_call(sp, name, expanded);
      cx.span_note(sp, format!("{}", pprust::expr_to_str(call)));
      MacExpr::new(call)
    }
    None => MacExpr::new(quote_expr!(cx, ()))
  }
}


