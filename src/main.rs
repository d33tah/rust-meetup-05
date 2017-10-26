use std::collections::HashMap;

enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<BencodedObject>),
    Dict(HashMap<BencodedObject, BencodedObject>)
}

fn decode(encoded: String) -> BencodedObject {
}

fn main() {
}
