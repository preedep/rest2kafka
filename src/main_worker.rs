use std::env;
use dotenv::dotenv;
use log::{debug, info};
use rdkafka::{ClientConfig, ClientContext, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use crate::utils::KafkaConfig;

mod utils;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance<'a>(&self, rebalance: &Rebalance<'a>) {
        info!("Pre rebalance {:#?}", rebalance);
    }

    fn post_rebalance<'a>(&self, rebalance: &Rebalance<'a>) {
        info!("Post rebalance {:#?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, offsets: &TopicPartitionList) {
        info!("Committing result : {:#?}  offsets: {:#?}", result , offsets);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;
#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    info!("Starting Rest2Kafka main worker");

    let kafka_config = KafkaConfig {
        bootstrap_servers: env::var("KAFKA_EVENTHUB_ENDPOINT")
            .expect("KAFKA_BOOTSTRAP_SERVERS must be set"),
        connection_string: env::var("KAFKA_EVENTHUB_CONNECTION_STRING")
            .expect("KAFKA_CONNECTION_STRING must be set"),
    };

    debug!("Kafka config = :{:#?}", kafka_config);

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", &kafka_config.bootstrap_servers);
    config.set("group.id", "$Default");
    config.set("enable.partition.eof", "false");
    config.set("session.timeout.ms", "6000");
    config.set("enable.auto.commit", "true");
    config.set("security.protocol", "sasl_ssl");
    config.set("sasl.mechanisms", "PLAIN");
    config.set("sasl.username", "$ConnectionString");
    config.set("sasl.password", &kafka_config.connection_string);
    config.set_log_level(RDKafkaLogLevel::Debug);


    let context = CustomContext;
    let consumer: LoggingConsumer = config.create_with_context(context).expect("Consumer creation error");
    consumer.subscribe(&["tpx_message"]).expect("Can't subscribe to specified topics");
    loop {
        match consumer.recv().await {
            Err(e) => {
                println!("Error while receiving message: {:?}", e);
            }
            Ok(msg) => {
                println!("Received message: {:#?}", msg);

                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
        }
    }
    //let consumer = config.create().expect("Consumer creation error");
}
