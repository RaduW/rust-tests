//! Example of a custom deserialization that detects

use core::fmt;

use serde::{Deserialize, Deserializer};

use serde::de::Error;
use serde::de::{MapAccess, Visitor};
use serde_json::Value;

#[derive(Debug)]
pub enum MessageType {
    Message1,
    Message2,
    Message3,
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MessageTypeVisitor)
    }
}

struct MessageTypeVisitor;

impl<'de> Visitor<'de> for MessageTypeVisitor {
    type Value = MessageType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting a dictionary.")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut message_type: Option<MessageType> = None;

        while let Some((key, _)) = access.next_entry::<&str, Value>()? {
            match key {
                "message1" => {
                    message_type = Some(MessageType::Message1);
                }
                "message2" => {
                    message_type = Some(MessageType::Message2);
                }
                "message3" => {
                    message_type = Some(MessageType::Message3);
                }
                _ => {}
            }
        }

        message_type.ok_or(M::Error::custom("Invalid message type"))
    }
}

fn main() {
    println!("Hello from Serde");

    let test = r#"{
        "second": "the second",
        "third": { "x":"abc", "w": 1234}, 
        "fourth": 23423,
        "fifth": 23423,
        "sixth": 23423,
        "message1": { "x":"abc", "w": 1234},
        "xx": 33
    }"#;

    let x = serde_json::from_str::<MessageType>(test);

    println!("{:?}", x);
}
