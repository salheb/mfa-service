use std::time::Duration;

use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use log::{info, warn};

use crate::{core::util, domain::message::Message};

fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", util::get_env_value("kakfa_broker"))
        .set("group.id", util::get_env_value("group_id"))
        .set("queue.buffering.max.ms", "0") // Do not buffer
        .set("session.timeout.ms", util::get_env_value("session_timeout_ms"))
        .create()
        .expect("Producer creation failed")
}


pub async fn message(message: Message) -> bool{
    info!("Starting kafka::message async fn");
    let producer = create_producer();
    let topic = util::get_env_value("topic_messaging");
    //let topic_dlq = util::get_env_value("topic_messaging_dlq");
    log::info!("Topic name {}", topic);

    match producer.send(
        FutureRecord::to(&topic)
            .key("0")
            .payload(&message.to_bytes()),
        Duration::from_secs(0),
    )
        .await {
            Ok(delivery) => { info!("Sent: {:?}", delivery); true},
            Err((e, _)) => { warn!("Error: {:?}", e); false},
        }
}

