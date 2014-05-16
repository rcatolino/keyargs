#![crate_id = "test"]
#![crate_type = "bin"]
#![feature(phase)]

#[phase(syntax)] extern crate keyargs;

fn main() {
  keyargs!("coucou", 1);
  keyargs!("coucou", Some("yop"));
  keyargs!("coucou", "yop");
  keyargs!("coucou");
  keyargs!("coucou", opt1="test1", 3);
  keyargs!("coucou", "t1", opt1=4);
}
