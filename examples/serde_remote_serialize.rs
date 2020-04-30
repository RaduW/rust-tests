#![allow(warnings)]

use log;
use serde::{Deserialize, Serialize};
use simple_logger;

fn default_44() -> i32 {
    44
}

fn default_someone() -> String {
    "someone".to_owned()
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Test {
    pub an_int: i32,
    pub a_string: String,
    pub internal_x: String,
    #[serde(skip, default = "default_44")]
    pub internal_y: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Test")]
struct Test2 {
    #[serde(skip)]
    pub an_int: i32,
    pub a_string: String,
    #[serde(skip, default = "default_someone")]
    pub internal_x: String,
    #[serde(skip, default = "default_44")]
    pub internal_y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MotherShip {
    pub x: i32,
    pub y: String,
    pub child: Test,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "MotherShip")]
struct MotherShipConverter {
    #[serde(skip)]
    pub x: i32,
    pub y: String,
    #[serde(with = "Test2")]
    pub child: Test,
}
#[derive(Serialize, Debug)]
struct MotheShipWrapper<'a>(#[serde(with = "MotherShipConverter")] pub &'a MotherShip);

#[derive(Serialize, Debug)]
struct Test2Wrapper<'a>(#[serde(with = "Test2")] pub &'a Test);

fn init() {
    simple_logger::init_with_level(log::Level::Trace);
}

fn main() {
    init();
    log::info!("{}", "Hello from serde_remote_serialize");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use serde_json::{self, Serializer as JsonSerializer};
    use std::io::Write;

    #[test]
    fn test_simple_deserialization() {
        init();
        let v1 = Test {
            a_string: "hello".to_owned(),
            internal_x: "internal_x".to_owned(),
            internal_y: 44,
            ..Test::default()
        };

        let serialized = serde_json::to_string(&v1).unwrap();
        log::debug!("Serialize via Test {}", serialized);
        let serialized = serde_json::to_string(&Test2Wrapper(&v1)).unwrap();
        log::debug!("Serialize via Test2 {}", serialized);
    }

    #[test]
    fn test_composed_deserialization() {
        init();
        let v1 = Test {
            an_int: 143,
            a_string: "hello".to_owned(),
            internal_x: "internal_x".to_owned(),
            internal_y: 44,
        };

        let ms = MotherShip {
            x: 123,
            y: "some val".to_owned(),
            child: v1,
        };

        let serialized = serde_json::to_string(&ms).unwrap();
        log::debug!("Serialize via MotherShip {}", serialized);
        let serialized = serde_json::to_string(&MotheShipWrapper(&ms)).unwrap();
        log::debug!("Serialize via MotherShipWrapper {}", serialized);
    }
}
