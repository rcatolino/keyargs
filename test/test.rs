#![feature(phase)]

#[phase(syntax)] extern crate keyargs;
extern crate testlib;

fn main() {
  keyargs!("coucou", 1);
  keyargs!("coucou", Some("yop"));
  keyargs!("coucou", "yop");
  keyargs!("coucou");
  keyargs!("coucou", opt1="test1", 3);
  keyargs!("coucou", "t1", opt1=4);
  keyargs!("coucou", "t1", opt2=4);
  keyargs!("coucou", opt1="t1", opt2=4);
  keyargs!("coucou", opt2=4);
}
