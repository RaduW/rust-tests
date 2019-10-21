use core::fmt;

//use serde::export::fmt::Error;
//use serde::export::Formatter;
use serde::{Deserialize, Deserializer};

use serde::de::{self, MapAccess, Visitor};

#[derive(Deserialize, Debug)]
struct MyStruct {
    pub first: i32,
    pub second: String,
    pub third: SubStruct,
    pub fourth: TheFourth,
}

#[derive(Deserialize, Debug)]
struct SubStruct {
    pub ss_1: i32,
    pub ss_2: String,
}

#[derive(Debug)]
struct TheFourth;

impl<'de> Deserialize<'de> for TheFourth {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        //deserializer.deserialize_any()
        deserializer.deserialize_any(TheFourthVistor)
    }
}

struct TheFourthVistor;

impl<'de> Visitor<'de> for TheFourthVistor {
    type Value = TheFourth;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = i32::from(value);
        Ok(TheFourth)
    }
    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = i32::from(value);
        Ok(TheFourth)
    }
    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = i32::from(value);
        Ok(TheFourth)
    }
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = i64::from(value);
        Ok(TheFourth)
    }
    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = u64::from(value);
        Ok(TheFourth)
    }
    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = u64::from(value);
        Ok(TheFourth)
    }
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = u64::from(value);
        Ok(TheFourth)
    }
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = u64::from(value);
        Ok(TheFourth)
    }
}

#[derive(Debug)]
pub enum MessageType {
    Message1,
    Message2,
    Message3,
    Unknown,
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
        let mut msg_type: Option<MessageType> = None;

        while let Some(key) = access.next_key()? {
            match key {
                "message1" => return Ok(MessageType::Message1),
                "message2" => return Ok(MessageType::Message2),
                "message3" => return Ok(MessageType::Message3),
                _ => {
                    println!("visited: {:?}", key);
                }
            }
        }
        Ok(MessageType::Unknown)
    }
}

fn main() {
    println!("Hello from Serde");

    //    let test = r#"{
    //        "first": 33,
    //        "second": "the second",
    //        "third": {
    //            "ss_1": 44,
    //            "ss_2": "ss_2 value",
    //            "ss_3": "something else"
    //        },
    //        "fourth": 23423
    //    }"#;
    //
    //    let x: MyStruct = serde_json::from_str(test).unwrap();

    //    println!("{:?}", x);

    let test = r#"{
        "message1": 33, 
        "second": "the second",
        "third": "abc", 
        "fourth": 23423
    }"#;

    let x: MessageType = serde_json::from_str(test).unwrap();

    println!("{:?}", x);
}
