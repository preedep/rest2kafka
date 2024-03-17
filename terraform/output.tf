output "event_hub_connection_string" {
  value = azurerm_eventhub_authorization_rule.kafka_eh_eventhub_topic_tpx_message_auth.primary_connection_string
  sensitive = true
}
