use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::str;

fn main() {
    let file = File::open("tempfile").unwrap();
    let mut reader = BufReader::new(file);
    let mut next = next_char(&mut reader);
    if next != None {
        print!("{:?}", next.unwrap());
    }
    while next != None {
        next = next_char(&mut reader);
        if next != None {
            print!("{:?}", next.unwrap());
        }
    }
}

fn next_char(mut reader: &mut Read) -> Option<char> {
    let mut buf: Vec<u8> = Vec::new();
    let mut next = reader.take(1);
    let result = next.read_to_end(&mut buf);
    if result.unwrap() == 1 {
        let s = str::from_utf8(&mut buf).unwrap();
        //TODO: handle utf here
        let c = s.chars().next().unwrap();
        Some(c)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::str;
    use next_char;

    #[test]
    fn test_next_char_ascii() {
        let file = File::open("tests/simple_ascii").unwrap();
        let mut reader = BufReader::new(file);
        let mut next = next_char(&mut reader);
        assert_eq!('a', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!('b', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!('c', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!('d', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!(' ', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!('0', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!('1', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!('2', next.unwrap());
        next = next_char(&mut reader);
        assert_eq!('3', next.unwrap());
    }
}
