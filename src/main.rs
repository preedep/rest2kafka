use std::env;

use actix_web::middleware::Logger;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::{debug, error, info};
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use serde_json::Value;

// This struct represents state
struct AppState {
    producer: rdkafka::producer::FutureProducer,
}

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

fn log_produce_result(
    topic: &str,
    result: Result<(i32, i64), (KafkaError, OwnedMessage)>,
) -> Result<(), ()> {
    result
        .and_then(|(p, o)| {
            info!(
                "Successfully produced record to topic {} partition [{}] @ offset {}",
                topic, p, o
            );
            Ok(())
        })
        .map_err(|(err, _)| error!("kafka error: {}", err))
}
async fn post_message_to_kafka(
    producer: &rdkafka::producer::FutureProducer,
    topic_name: &String,
    message: &Value,
) -> OwnedDeliveryResult {
    let result = producer
        .send(
            rdkafka::producer::FutureRecord::to(topic_name)
                .payload(&message.to_string())
                .key("key"),
            Timeout::After(std::time::Duration::from_secs(0)),
        )
        .await;
    result
}

#[post("/post_message")]
async fn post_message_handle(
    req_body: web::Json<RequestData>,
    state: web::Data<AppState>,
) -> impl Responder {
    debug!("Post message handle : {:#?}", req_body);

    let result =
        post_message_to_kafka(&state.producer, &req_body.topic_name, &req_body.message).await;

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

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", &kafka_config.bootstrap_servers);
    config.set("security.protocol", "sasl_ssl");
    config.set("sasl.mechanisms", "PLAIN");
    config.set("sasl.username", "$ConnectionString");
    config.set("sasl.password", &kafka_config.connection_string);

    let producer: rdkafka::producer::FutureProducer =
        config.create().expect("Producer creation error");
    let app_state = web::Data::new(AppState { producer });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(app_state.clone()))
            .service(post_message_handle)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
