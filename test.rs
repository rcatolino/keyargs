#![crate_id = "test"]
#![crate_type = "bin"]
#![feature(phase)]

#[phase(syntax)] extern crate keyargs;

fn main() {
  keyargs!("coucou", None, Some(1));
  keyargs!("coucou", Some("yop"), None);
  keyargs!("coucou", Some("yop"));
  keyargs!("coucou");
}
