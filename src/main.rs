use std::collections::HashMap;

#[derive(Debug)]
enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<BencodedObject>),
//    Dict(HashMap<BencodedObject, BencodedObject>)
}

fn decode(encoded: String) -> Result<BencodedObject,String> {
    let mut bencoded_type: Option<char>;
    for (c, n) in encoded.chars().enumerate() {
        if n == 0 {
            bencoded_type = c;
        }
    }
    Ok(BencodedObject::Int(1))
}

fn main() {
    println!("{:?}", decode("i1e".into()));
}
