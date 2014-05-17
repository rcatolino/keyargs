#![crate_id = "testlib"]
#![crate_type = "dylib"]

pub fn test(mandatory: &str, opt1: Option<&'static str>, opt2: Option<int>) {
  println!("Got {}, {}, {}", mandatory, opt1, opt2);
}

