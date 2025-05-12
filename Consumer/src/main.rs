use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn main() {
    let mut consumer =
       Consumer::from_hosts(vec!("0.0.0.0:9092".to_owned()))
          .with_topic("short_wikimedia".to_owned())
          .with_group("my_group".to_owned())
          .with_offset_storage(Some(GroupOffsetStorage::Kafka))
          .with_fallback_offset(FetchOffset::Earliest)
          .create()
          .unwrap();
    loop {
      for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
          let str = String::from_utf8_lossy(m.value);
          println!("{:?}",str);
        }
        let _ = consumer.consume_messageset(ms);
      }
      consumer.commit_consumed().unwrap();
    }
}
