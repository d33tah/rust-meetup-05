use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<BencodedObject>),
//    Dict(HashMap<BencodedObject, BencodedObject>)
}

fn decode(encoded: String) -> Result<BencodedObject,Error> {
    let mut bencoded_type: char;
    for (i, n) in encoded.chars().enumerate() {

    }
    BencodedObject::Int(1)
}

fn main() {
    println!("{:?}", decode("i1e".into()));
}
