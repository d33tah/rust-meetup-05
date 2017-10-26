use std::boxed::Box;

enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Box<BencodedObject>),
    Dict
}

fn main() {
}
