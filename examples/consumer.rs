#![allow(warnings)]
/*
use futures::Stream;
use rdkafka::consumer::{CommitMode, Consumer, DefaultConsumerContext, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::ClientConfig;

const CONSUMER_OPTIONS: &[(&str, &str)] = &[
    ("bootstrap.servers", "127.0.0.1"),
    ("group.id", "1"),
    ("enable.auto.commit", "true"),
];

fn main() {
    consume(CONSUMER_OPTIONS);
}
fn consume(options: &[(&str, &str)]) {
    let mut config = ClientConfig::new();

    for (key, value) in options {
        config.set(key, value);
    }

    let consumer: StreamConsumer<DefaultConsumerContext> =
        config.create().expect("Failed to create stream consumer");

    consumer.subscribe(&["rust-test-topic"]);

    let message_stream = consumer.start();

    for message in message_stream.wait() {
        match message {
            Err(e) => {
                println!("Stream error {:?}", e);
            }
            Ok(Err(e)) => {
                println!("Deserialization error {:?}", e);
            }
            Ok(Ok(msg)) => {
                let payload = match msg.payload_view::<str>() {
                    None => "".to_string(),
                    Some(Ok(s)) => s.to_string(),
                    Some(Err(e)) => format!("Error {:?}", e),
                };
                println!("Message received: {}", payload);
                consumer.commit_message(&msg, CommitMode::Async);
            }
        }
    }
}
*/

fn main() {}
