use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, info};
use serde_json::Value;

async fn post_message_to_kafka(topic_name: &String, message: &Value) {}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct RequestData {
    #[serde(rename = "topicName")]
    pub topic_name: String,
    #[serde(rename = "message")]
    pub message: Value,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Response {}
#[post("/post_message")]
async fn post_message_handle(req_body: web::Json<RequestData>) -> impl Responder {
    debug!("Post message handle : {:#?}", req_body);

    post_message_to_kafka(&req_body.topic_name, &req_body.message).await;

    let response = Response {};
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting server at");

    HttpServer::new(|| App::new().service(post_message_handle))
        .bind(("0.0.0.0", 8888))?
        .run()
        .await
}
