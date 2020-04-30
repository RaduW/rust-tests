#![allow(warnings)]
use bytes::{Bytes, BytesMut};
use core::fmt;
use rmp_serde::{self, encode::Serializer as MpSerializer};
use serde::{
    ser::{SerializeSeq, Serializer},
    Deserialize, Deserializer, Serialize,
};

use serde_json::{self, Serializer as JsonSerializer};

fn main() {
    println!("Hello from serde_mp");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::io::Write;

    #[test]
    fn test_that() {
        let x: &[&[&str]] = &[&["a", "b"], &["y", "z"]];

        let result = rmp_serde::to_vec_named(x).unwrap();
        let x2: Value = rmp_serde::from_read_ref(&result).unwrap();
        println!("serialized {:?}", result);
        println!("deserialized {:?}", x2);
    }

    #[test]
    fn test_serialize_message_pack_sequence() {
        let mut ser = MpSerializer::new(Vec::new());
        let mut seq = ser.serialize_seq(Some(1)).unwrap();
        //let mut seq = ser.serialize_seq(None).unwrap();
        seq.serialize_element("hello");
        seq.end();
    }

    #[test]
    fn test_serialize_message_pack_steam() {
        let mut data = Vec::new();

        let mut ser = MpSerializer::new(&mut data);
        ser.serialize_i32(22);
        ser.serialize_i32(23);
        ser.serialize_i32(24);
        println!("{:?}", &data);
    }

    #[test]
    fn test_serialize_json_sequence() {
        let mut ser = JsonSerializer::new(Vec::new());
        //let mut seq = ser.serialize_seq(Some(1)).unwrap();
        let mut seq = ser.serialize_seq(None).unwrap();
        seq.serialize_element("hello");
        seq.end();
    }

    #[test]
    fn test_json_stream_serialization_int() {
        let mut v = Vec::<u8>::new();

        //put some values in the writer
        serde_json::ser::to_writer(&mut v, &24_i32);
        v.write_all(b" ");
        serde_json::ser::to_writer(&mut v, &25_i32);
        v.write_all(b" ");
        serde_json::ser::to_writer(&mut v, &26_i32);

        let mut stream = serde_json::Deserializer::from_slice(&*v).into_iter();

        loop {
            match stream.next() {
                None => {
                    println!("End");
                    break;
                }
                Some(Err(err)) => {
                    println!("Error :{:?}", err);
                    break;
                }
                Some(Ok(val)) => {
                    let v: i32 = val;
                    println!("val {}", v);
                }
            }
        }
    }

    #[test]
    fn test_json_stream_serialization_string_pairs() {
        let mut v = Vec::<u8>::new();

        #[derive(Serialize, Deserialize, Debug)]
        struct Pair<'a>(pub &'a str, pub &'a str);

        //put some values in the writer
        serde_json::ser::to_writer(&mut v, &["abc", "cde"]);
        //v.write_all(b" ");
        serde_json::ser::to_writer(&mut v, &["efg", "hij"]);
        //v.write_all(b" ");
        serde_json::ser::to_writer(&mut v, &Pair("klm", "nop"));

        let mut stream = serde_json::Deserializer::from_slice(&*v).into_iter();

        loop {
            match stream.next() {
                None => {
                    println!("End");
                    return;
                }
                Some(Err(err)) => {
                    println!("Error :{:?}", err);
                    return;
                }
                Some(Ok(val)) => {
                    let v: Pair = val;
                    println!("val {:?}", v);
                }
            }
        }
    }
}
