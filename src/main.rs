use std::io::Read;
use std::str;

fn main() {}

fn next_char(mut reader: &mut Read) -> char{
  let mut buf: Vec<u8> = Vec::new();
  let mut next = reader.take(1);
  next.read_to_end(&mut buf);
  let s = str::from_utf8(&mut buf).unwrap();
  //deal with utf-8 here
  println!("{:?}", s);
  let c = s.chars().next().unwrap();
  c
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::Read;
  use std::io::BufReader;
  use std::str;
  use next_char;

  #[test]
  fn test_next_char_ascii() {
    let file = File::open("tests/simple_ascii").unwrap();
    let mut reader = BufReader::new(file);
    let mut next = next_char(&mut reader);
    assert_eq!('a', next);
    next = next_char(&mut reader);
    assert_eq!('b', next);
    next = next_char(&mut reader);
    assert_eq!('c', next);
    next = next_char(&mut reader);
    assert_eq!('d', next);
    next = next_char(&mut reader);
    assert_eq!(' ', next);
    next = next_char(&mut reader);
    assert_eq!('0', next);
    next = next_char(&mut reader);
    assert_eq!('1', next);
    next = next_char(&mut reader);
    assert_eq!('2', next);
    next = next_char(&mut reader);
    assert_eq!('3', next);
  }
}
