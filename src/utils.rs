#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub connection_string: String,
}