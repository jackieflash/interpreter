use std::fs::File;
use std::str;
use utf_read::UTFReadIter;

mod utf_read;

fn main() {
    let reader = UTFReadIter::new(File::open("tempfile").unwrap());
    for dat in reader {
        println!("{:?}", dat);
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
