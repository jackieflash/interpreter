use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::str;

fn main() {
  let file = File::open("tests/simple").unwrap();
  let mut reader = BufReader::new(file);
  let mut buf: Vec<u8> = Vec::new();
  let mut next = reader.take(1);
  next.read_to_end(&mut buf);
  println!("{:?}", str::from_utf8(&buf).unwrap());
}
