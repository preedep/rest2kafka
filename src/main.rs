use std::env;
use std::fmt::{Debug};

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, middleware, post, Responder, ResponseError, web,
};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use log::{debug, error, info};
use rdkafka::ClientConfig;
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::util::Timeout;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::KafkaConfig;

mod utils;

// This struct represents state
struct AppState {
    producer: rdkafka::producer::FutureProducer,
    //client_config: rdkafka::ClientConfig,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct RequestData {
    #[serde(rename = "topicName")]
    pub topic_name: String,
    #[serde(rename = "message")]
    pub message: Value,
}

#[derive(Deserialize, Serialize, Clone, Debug, derive_more::Display, derive_more::Error)]
struct Error {
    message: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Rest2KafkaResponse {
    pub message: String,
}

impl Rest2KafkaResponse {
    pub fn new(message: &str) -> Rest2KafkaResponse {
        Rest2KafkaResponse {
            message: message.to_string(),
        }
    }
}

impl Responder for Rest2KafkaResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

type Rest2KafkaResult = Result<Rest2KafkaResponse, Rest2KafkaError>;

#[derive(Deserialize, Serialize, Clone, Debug, derive_more::Display, derive_more::Error)]
struct Rest2KafkaError {
    message: String,
}

impl Rest2KafkaError {
    pub fn new(message: &str) -> Rest2KafkaError {
        Rest2KafkaError {
            message: message.to_string(),
        }
    }
}

impl ResponseError for Rest2KafkaError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().json(self)
    }
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
) -> Rest2KafkaResult {
    debug!("Post message handle : {:#?}", req_body);

    let result =
        post_message_to_kafka(&state.producer, &req_body.topic_name, &req_body.message).await;
    return match result {
        Ok(result) => {
            //log_produce_result(&req_body.topic_name, result);
            debug!(
                "Message sent to Kafka with topic : {}  partition [{}] @ offset {}",
                &req_body.topic_name, result.0, result.1
            );

            let response = Rest2KafkaResponse::new(&format!(
                "Message sent to Kafka with topic : {}  partition [{}] @ offset {}",
                &req_body.topic_name, result.0, result.1
            ));
            Ok(response)
        }
        Err(err) => {
            error!("Error sending message to Kafka : {:?}", err);

            Err(Rest2KafkaError::new(&format!(
            "Error sending message to Kafka : {:?}",
            err
        )))},
    };
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
    debug!("Kafka config = :{:#?}", kafka_config);

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", &kafka_config.bootstrap_servers);
    config.set("security.protocol", "sasl_ssl");
    config.set("sasl.mechanisms", "PLAIN");
    config.set("sasl.username", "$ConnectionString");
    config.set("sasl.password", &kafka_config.connection_string);

    let producer = config.create().expect("Producer creation error");
    let app_state = web::Data::new(AppState {
        producer,
        //client_config: config,
    });
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_state.clone())
            .service(post_message_handle)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
