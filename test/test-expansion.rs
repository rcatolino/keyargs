#![feature(phase)]

#[phase(syntax)] extern crate keyargs;
extern crate testlib;

fn main() {
  keyargs!(3);
  keyargs!("mandatory");
  keyargs!("mandatory", "opt");
  keyargs!("mandatory", "opt", 3);
  keyargs!("mandatory", opt1="opt", opt2=3);
  keyargs!("mandatory", opt2=3, opt1="opt");
  keyargs!("mandatory", "opt", opt2=3);
}
