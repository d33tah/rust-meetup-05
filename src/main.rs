use std::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq)]
enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<BencodedObject>),
    Dict(HashMap<BencodedObject, BencodedObject>)
}

impl Hash for BencodedObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            Int(x) => x.hash(state),
            ByteString(x) => x.hash(state),
            _ => {}
        }
    }

}

fn decode(encoded: String) -> Result<BencodedObject, String> {
    let mut bencoded_type: Option<char> = None;
    let mut buf: Vec<char> = vec![];
    for (n, c) in encoded.chars().enumerate() {
        if n == 0 {
            bencoded_type = Some(c);
            continue;
        }
        match bencoded_type {
            Some('i') => {
                if c != 'e' {
                    buf.push(c);
                } else {
                    let s: String = buf.iter().collect();
                    let i = s.parse::<i32>().unwrap();
                    return Ok(BencodedObject::Int(i))
                }
            }
            Some(_) => return Err("Niespodziewany typ".into()),
            None => {
                if n > 1 {
                    return Err("Nie ustawiono typu".into());
                }
            }

        }
    }
    Err("Niedokonczony obiekt".into())
}

fn main() {
    println!("{:?}", decode("i42e".into()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_one() {
        assert_eq!(BencodedObject::Int(1), decode("i1e".into()).unwrap());
    }

    #[test]
    fn test_integer_two_digits() {
        assert_eq!(BencodedObject::Int(22), decode("i22e".into()).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_unexpected_type() {
        decode("z".into()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_integer_but_not_integer() {
        decode("iie".into()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_integer_missing_e() {
        decode("i1".into()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_empty_buffer() {
        decode("ie".into()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_empty_string() {
        decode("".into()).unwrap();
    }
}
