#![feature(phase)]

#[phase(syntax)] extern crate keyargs;
extern crate testlib;

fn main() {
  keyargs!();                   //~error: missing mandatory argument at call site.
  keyargs!(opt1="option");      //~ error: missing mandatory argument at call site.
  keyargs!("man", opt3="test"); //~ error: invalid keyword argument `opt3`.
  keyargs!("man", "test", opt1="test2"); //~ error: keyword argument `opt1` was already given as a positional argument. //~ note: corresponding positional argument is:
  keyargs!("man", "test", opt4=4); //~ error: invalid keyword argument `opt4`.
  keyargs!("man", opt1="test", opt1="test2"); //~ error: keyword argument `opt1` has already been given.//~ note: its previous position is:
  keyargs!("man", opt()="invalid"); //~ error: expected argument name but found expression `opt()`
  keyargs!("man", "option", 1+2=3); //~ error: expected argument name but found expression `1 + 2`
}
