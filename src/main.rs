use std::collections::HashMap;

#[derive(Debug)]
enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<BencodedObject>),
//    Dict(HashMap<BencodedObject, BencodedObject>)
}

fn decode(encoded: String) -> BencodedObject {
    let mut bencoded_type: char;
    for i in encoded.bytes() {
    }
    BencodedObject::Int(1)
}

fn main() {
    println!("{:?}", decode("i1e".into()));
}
