use std::io::Write;
use rustc_serialize::{json,Encodable};

/// Write an encodable object to the given output stream as a JSON object.
pub fn write<E: Encodable>(object: &E, out: &mut Write) {
    let encoded = json::encode(&object).expect("Error encoding");
    out.write_all(&encoded.as_bytes()).expect("Error writing to output");
    out.write(&[10]).expect("Error writing to output");
}
