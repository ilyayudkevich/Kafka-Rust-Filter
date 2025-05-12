use std::collections::HashMap;
use std::time::Duration;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, DefaultConsumerContext};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::error::KafkaResult;
use rdkafka::topic_partition_list::TopicPartitionList;
use kafka::error::Error as KafkaError;
use rdkafka::Message;
use std::collections::HashSet;
use sonic_rs::{from_str, pointer, Value, get, json, JsonValueTrait, Error}; 
use faststr::FastStr;
use serde::{ser::SerializeMap, Serializer};
use sonic_rs::{to_object_iter, writer::WriteExt, Serialize, Deserialize};
use kafka::producer::{Producer, Record, RequiredAcks};

#[allow(clippy::mutable_key_type)]
fn filter_json<W: WriteExt>(json: &str, keys: HashSet<FastStr>, w: W) -> sonic_rs::Result<()> {
    // create a new serialize from writer
    let mut outer = sonic_rs::Serializer::new(w);

    // begin to serialize a map
    let mut maper = outer.serialize_map(None)?;
    for ret in to_object_iter(json) {
        let (name, value) = ret.expect("invalid json");
        if keys.contains(name.as_ref()) {
            maper.serialize_entry(&name, &value)?;
        }
    }
    maper.end()
}

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(async {
        let _ = sub_to_topic("0.0.0.0:9092", "default", &["wikimedia"]).await;
    })
    .await;
}

fn produce_message<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    // ~ create a producer. this is a relatively costly operation, so
    // you'll do this typically once in your application and re-use
    // the instance many times.
    let mut producer = Producer::from_hosts(brokers)
        // ~ give the brokers one second time to ack the message
        .with_ack_timeout(Duration::from_secs(1))
        // ~ require only one broker to ack the message
        .with_required_acks(RequiredAcks::One)
        // ~ build the producer with the above settings
        .create()?;

    // ~ now send a single message.  this is a synchronous/blocking
    // operation.

    // ~ we're sending 'data' as a 'value'. there will be no key
    // associated with the sent message.

    // ~ we leave the partition "unspecified" - this is a negative
    // partition - which causes the producer to find out one on its
    // own using its underlying partitioner.
    producer.send(&Record {
        topic,
        partition: -1,
        key: (),
        value: data,
    })?;

    // ~ we can achieve exactly the same as above in a shorter way with
    // the following call
    producer.send(&Record::from_value(topic, data))?;

    Ok(())
}

fn create_consumer(brokers: &str, group_id: &str) -> BaseConsumer {
    ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .create()
        .unwrap()
}

//pub async fn sub_to_topic(brokers: &str, group_id: &str, topics: &[&str]) {
pub async fn sub_to_topic(brokers: &str, group_id: &str, topics: &[&str])-> Result<(), Box<dyn std::error::Error>> {
    let consumer = create_consumer(brokers, group_id);

    consumer
        .subscribe(&topics.to_vec())
        .expect("failed to subscribe");
    
//    let mut data_short: HashMap<String, String> = HashMap::new();    
    let path = pointer!["meta", "dt"];

    for msg in consumer.iter() {
        match msg {
            Err(e) => println!("error retrieving msg: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
//                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
 //                     m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
/*       Processing payload   */

                #[allow(clippy::mutable_key_type)]
                let keys = ["type","timestamp", "title_url", "comment"].iter().map(|s| FastStr::from(*s)).collect();
                let mut buf = Vec::new();
//                println!("{:?}", String::from_utf8(buf));
                filter_json(payload, keys, &mut buf).unwrap();
                let extracted = String::from_utf8(buf).unwrap();
                if extracted.contains("type") && extracted.contains("edit"){
                    let target = get(payload, &path);
                    let dt = target.as_str().unwrap();
    //                println!("{:?}",dt);
                    /* Assemble full message */
                    let mut message = String::from("{\"dt\"");
                    message.push_str(":\"");
                    message.push_str(dt);
                    message.push_str("\",");
    
                   let s = extracted;
                   let slice = &s[1..];
                   message.push_str(slice);
                   /* to Producer  */
//                   tracing_subscriber::fmt::init();

                   let broker = "0.0.0.0:9092";
                   let topic = "short_wikimedia";
               
                   let data = message.as_bytes();
               
                   if let Err(e) = produce_message(data, topic, vec![broker.to_owned()]) {
                       println!("Failed producing messages: {}", e);
                   }
                   /*              */



                   println!("{:?}",message);

                }
            } // Ok
        } // match
    } //for  
    Ok(())
}

