use chrono::{Timelike, Utc};
use futures;
use futures::future::Future;
use rdkafka;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::SystemTime;

const PRODUCER_OPTIONS: &[(&str, &str)] = &[("bootstrap.servers", "127.0.0.1")];

fn main() {
    produce(PRODUCER_OPTIONS);
}

fn produce(options: &[(&str, &str)]) {
    let mut config = ClientConfig::new();

    for (key, value) in options {
        config.set(key, value);
    }

    let producer: FutureProducer = config.create().expect("Could not create producer");

    let futures = (0..115)
        .map(|i| {
            producer
                .send(
                    FutureRecord::to("rust-test-topic")
                        .payload(&format!("({}) message {}", Utc::now(), i))
                        .key(&format!("{}", i)),
                    0,
                )
                .map(move |status| {
                    let time = Utc::now();
                    println!("({}) Status received {:#?}", time, status);
                    status
                })
        })
        .collect::<Vec<_>>();

    for future in futures {
        match future.wait() {
            Ok(msg) => {
                println!("Success {:#?}", msg);
            }
            Err(err) => {
                println!("Error {}", err);
            }
        }
    }
}
