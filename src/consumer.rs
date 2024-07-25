use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::client::KafkaClient;

fn main() {
    let mut client = KafkaClient::new(vec!("localhost:9092".to_owned()));
    client.load_metadata_all().unwrap();
    let partitions = client.topics().partitions("my-topic").unwrap().available_ids();
    let mut consumer =
        Consumer::from_client(client)
            .with_topic_partitions("my-topic".to_owned(), &partitions)
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group("my-group".to_owned())
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()
            .unwrap();
    println!("Start listening");
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}", String::from_utf8_lossy(m.value));
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}