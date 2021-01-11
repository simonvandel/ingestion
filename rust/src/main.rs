use std::time::Duration;

use clap::{value_t, App, Arg};
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use log::info;

use kafka::{parse_request, PayloadResponse};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;

mod kafka;
mod operation;

async fn run_async_processor(brokers: String, group_id: String, input_topics: Vec<String>) {
    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", &brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(
            input_topics
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .unwrap_or_else(|x| {
            panic!(format!(
                "Can't subscribe to topics `{:?}`: {:?}",
                input_topics, x
            ))
        });

    // Create the `FutureProducer` to produce asynchronously.
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Create the outer pipeline on the message stream.
    let stream_processor = consumer.start().try_for_each(|borrowed_message| {
        let producer = producer.clone();
        async move {
            let json_payload =
                std::str::from_utf8(borrowed_message.payload().expect("no payload found"))
                    .expect("payload was not UTF8");
            let topic = borrowed_message.topic().parse().expect("unknown topic");
            let request = parse_request(&topic, json_payload).expect("Could not parse operation");
            let res = request.op.eval();
            let response_payload = PayloadResponse::new(request.id, res);
            let response_payload_json: String =
                serde_json::to_string(&response_payload).expect("serialization into json failed");
            let produce_future = producer.send(
                FutureRecord::<(), _>::to(topic.output_topic()).payload(&response_payload_json),
                Duration::from_secs(0),
            );
            match produce_future.await {
                Ok(delivery) => println!("Sent: {:?}", delivery),
                Err((e, _)) => println!("Error: {:?}", e),
            }
            Ok(())
        }
    });

    info!("Starting event loop");
    stream_processor.await.expect("stream processing failed");
    info!("Stream processing terminated");
}

#[tokio::main]
async fn main() {
    let matches = App::new("Async example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Asynchronous computation example")
        .arg(
            Arg::with_name("brokers")
                .short("b")
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("group-id")
                .short("g")
                .long("group-id")
                .help("Consumer group id")
                .takes_value(true)
                .default_value("example_consumer_group_id"),
        )
        .arg(
            Arg::with_name("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input-topic")
                .long("input-topic")
                .help("Input topic")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("num-workers")
                .long("num-workers")
                .help("Number of workers")
                .takes_value(true)
                .default_value("1"),
        )
        .get_matches();

    let brokers = matches.value_of("brokers").unwrap();
    let group_id = matches.value_of("group-id").unwrap();
    let input_topics = matches
        .values_of("input-topic")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let num_workers = value_t!(matches, "num-workers", usize).unwrap();

    (0..num_workers)
        .map(|_| {
            tokio::spawn(run_async_processor(
                brokers.to_owned(),
                group_id.to_owned(),
                input_topics.clone(),
            ))
        })
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await
}
