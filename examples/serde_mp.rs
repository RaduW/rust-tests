use bytes::{Bytes, BytesMut};
use core::fmt;

use rmp_serde::{self, encode::Serializer as MpSerializer};
use serde::{Deserialize, Deserializer};

fn main() {
    println!("Hello from serde_mp");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that() {
        let x: &[&[&str]] = &[&["a", "b"], &["y", "z"]];

        let mut ser = MpSerializer::new(Vec::new());

        let result = rmp_serde::to_vec_named(x).unwrap();

        let x2 = rmp_serde::from_read_ref(&result);
        println!("{:?}", result);
    }
}
