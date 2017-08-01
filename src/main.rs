use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::str;
use utf_read::UTFReadIter;

fn main() {
    let mut reader = UTFReadIter::new(File::open("tempfile").unwrap());
    loop {
        match reader.next() {
            None => break,
            Some(dat) => println!("{:?}", dat),
        }
    }
}

mod utf_read {

    use std::io::Read;
    use std::io::{BufReader, BufRead};
    use std::str;
    use std::str::Chars;
    use std::fs::File;

    enum UTFType {
        Ascii,
        Utf8(usize),
        Invalid,
    }

    pub struct UTFReadIter<R> {
        reader: R,
    }

    impl<R> UTFReadIter<R> {
        pub fn new(reader: R) -> UTFReadIter<R> {
            UTFReadIter { reader }
        }
    }

    impl<R: Read> Iterator for UTFReadIter<R> {
        type Item = char;
        fn next(&mut self) -> Option<char> {
            let first_byte = read_byte(&mut self.reader);
            match first_byte {
                None => None,
                Some(first_byte) => {
                    match utf_detect(first_byte) {
                        UTFType::Ascii => Some(first_byte as char),
                        UTFType::Utf8(size) => {
                            let mut buf = [first_byte, 0, 0, 0];
                            for i in 1..size {
                                buf[i] = read_byte(&mut self.reader).unwrap();
                            }
                            Some(
                                str::from_utf8(&buf[..size])
                                    .unwrap()
                                    .chars()
                                    .next()
                                    .unwrap(),
                            )
                        }
                        UTFType::Invalid => None,
                    }
                }
            }
        }
    }

    fn read_byte(r: &mut Read) -> Option<u8> {
        let mut byte = [0];
        match r.read(&mut byte) {
            Ok(0) => None,
            Ok(..) => Some(byte[0]),
            Err(..) => None,
        }
    }
    // From http://www.fileformat.info/info/unicode/utf8.htm
    // The value of each individual byte indicates its UTF-8 function, as follows:

    // 00 to 7F hex (0 to 127): first and only byte of a sequence.
    // 80 to BF hex (128 to 191): continuing byte in a multi-byte sequence.
    // C2 to DF hex (194 to 223): first byte of a two-byte sequence.
    // E0 to EF hex (224 to 239): first byte of a three-byte sequence.
    // F0 to FF hex (240 to 255): first byte of a four-byte sequence.

    fn utf_detect(first_byte: u8) -> UTFType {
        match first_byte {
            0...127 => UTFType::Ascii,
            194...223 => UTFType::Utf8(2),
            224...239 => UTFType::Utf8(3),
            240...255 => UTFType::Utf8(4),
            _ => UTFType::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::str;
    use utf_read::UTFReadIter;

    #[test]
    fn test_next_char_ascii() {
        let mut reader = UTFReadIter::new(File::open("tests/simple_ascii").unwrap());
        let mut next = reader.next();
        assert_eq!('a', next.unwrap());
        next = reader.next();
        assert_eq!('b', next.unwrap());
        next = reader.next();
        assert_eq!('c', next.unwrap());
        next = reader.next();
        assert_eq!('d', next.unwrap());
        next = reader.next();
        assert_eq!(' ', next.unwrap());
        next = reader.next();
        assert_eq!('0', next.unwrap());
        next = reader.next();
        assert_eq!('1', next.unwrap());
        next = reader.next();
        assert_eq!('\n', next.unwrap());
        next = reader.next();
        assert_eq!('2', next.unwrap());
        next = reader.next();
        assert_eq!('3', next.unwrap());
    }
    #[test]
    fn test_next_char_utf8() {
        let mut reader = UTFReadIter::new(File::open("tests/simple_utf8").unwrap());
        let mut next = reader.next();
        assert_eq!('a', next.unwrap());
        next = reader.next();
        assert_eq!('チ', next.unwrap());
        next = reader.next();
        assert_eq!('😀', next.unwrap());
        next = reader.next();
        assert_eq!('¢', next.unwrap());
        next = reader.next();
        assert_eq!('€', next.unwrap());
    }
}
