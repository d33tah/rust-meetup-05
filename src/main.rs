use std::boxed::Box;
use std::collections::HashMap;

enum BencodedObject {
    Int(i32),
    ByteString(String),
    List(Vec<Box<BencodedObject>>),
    Dict(std::map
}

fn main() {
}
