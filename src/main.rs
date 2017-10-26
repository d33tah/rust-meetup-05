use std::collections::HashMap;

#[derive(Debug)]
enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<BencodedObject>),
//    Dict(HashMap<BencodedObject, BencodedObject>)
}

fn decode(encoded: String) -> Result<BencodedObject,String> {
    let mut bencoded_type: Option<char> = None;
    let mut buf: Vec<char>;
    for (n, c) in encoded.chars().enumerate() {
        if n == 0 {
            bencoded_type = Some(c);
        }
        match bencoded_type {
            Some('i') => {
                buf.append(c);
            },
            None => if (n > 1) { return Err("Nie ustawiono typu".into()) }
        }
    }
    Ok(BencodedObject::Int(1))
}

fn main() {
    println!("{:?}", decode("i1e".into()));
}
