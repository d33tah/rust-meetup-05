use std::collections::HashMap;

#[derive(Debug)]
enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<BencodedObject>), //    Dict(HashMap<BencodedObject, BencodedObject>)
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
                if (n > 1) {
                    return Err("Nie ustawiono typu".into());
                }
            }

        }
    }
    Ok(BencodedObject::Int(1))
}

fn main() {
    println!("{:?}", decode("i42e".into()));
}
