mod consumer;

use std::thread;
use kafka::producer::{Producer, Record};
use kafka::client::KafkaClient;
use chrono;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092".to_owned()));
    client.load_metadata_all().unwrap();
    let mut producer = Producer::from_client(client).create().unwrap();
    loop {
        let current_time = chrono::Local::now().to_rfc2822();
        match producer.send(
            // текущее время
            &Record::from_value("my-topic", format!("Current time: {}", current_time))
        ) {
            Ok(_) => println!("Sent message. Ok."),
            Err(e) => eprintln!("Error: {:?}", e),
        }
        thread::sleep(std::time::Duration::from_secs(2));
    };
}
