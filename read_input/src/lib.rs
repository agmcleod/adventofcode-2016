use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn read_text(path: &str) -> Result<String> {
  let mut text = String::new();
  let mut file = try!(File::open(path));
  try!(file.read_to_string(&mut text));
  Ok(text)
}