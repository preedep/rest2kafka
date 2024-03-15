use std::env;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use kafka::client::{
    Compression, KafkaClient, RequiredAcks, DEFAULT_CONNECTION_IDLE_TIMEOUT_MILLIS,
};
use kafka::producer::{AsBytes, Producer, Record, DEFAULT_ACK_TIMEOUT_MILLIS};
use log::{debug, error, info};
use serde_json::Value;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct KafkaConfig {
    pub bootstrap_servers: String,
    pub connection_string: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct RequestData {
    #[serde(rename = "topicName")]
    pub topic_name: String,
    #[serde(rename = "message")]
    pub message: Value,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Response {}

async fn post_message_to_kafka(topic_name: &String, message: &Value) {}

#[post("/post_message")]
async fn post_message_handle(req_body: web::Json<RequestData>) -> impl Responder {
    debug!("Post message handle : {:#?}", req_body);

    post_message_to_kafka(&req_body.topic_name, &req_body.message).await;

    let response = Response {};
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    pretty_env_logger::init();

    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let kafka_config = KafkaConfig {
        bootstrap_servers: env::var("KAFKA_EVENTHUB_ENDPOINT")
            .expect("KAFKA_BOOTSTRAP_SERVERS must be set"),
        connection_string: env::var("KAFKA_EVENTHUB_CONNECTION_STRING")
            .expect("KAFKA_CONNECTION_STRING must be set"),
    };

    info!("Starting server at 0.0.0.0:{}", port);
    debug!("Kafak config = :{:#?}", kafka_config);

    let mut client = KafkaClient::new(vec![kafka_config.bootstrap_servers.clone()]);
    client.set_client_id("kafka-rust-console-producer".into());
    let result = client.load_metadata_all();
    match result {
        Ok(_) => {
            info!("Kafka client connected successfully");
        }
        Err(e) => {
            error!("Kafka client connection failed : {:#?}", e);
        }
    }

    HttpServer::new(|| App::new().service(post_message_handle))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
