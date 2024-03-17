data "azurerm_resource_group" "rg" {
  name = "RG-SG-NICKDEV001"
  #  location = "southeastasia"
}
# Create azure event hub namespace using terraform
resource "azurerm_eventhub_namespace" "kafka_eh_namespace" {
  name                = lower("${var.kafka_eh_prefix}-${var.kafka_eh_namespace_name}")
  resource_group_name = data.azurerm_resource_group.rg.name
  location            = data.azurerm_resource_group.rg.location
  sku                 = var.kafka_eh_sku
  capacity            = var.kafka_eh_capacity
  tags = {
    environment = "development"
  }
}

resource "azurerm_eventhub" "kafka_eh_eventhub_topic_tpx_message" {
  name                = "tpx_message"
  namespace_name      = azurerm_eventhub_namespace.kafka_eh_namespace.name
  resource_group_name = data.azurerm_resource_group.rg.name
  partition_count     = 1
  message_retention   = 1
}

resource "azurerm_eventhub_authorization_rule" "kafka_eh_eventhub_topic_tpx_message_auth" {
  name                = "tpx_message_auth"
  namespace_name      = azurerm_eventhub_namespace.kafka_eh_namespace.name
  eventhub_name       = azurerm_eventhub.kafka_eh_eventhub_topic_tpx_message.name
  resource_group_name = data.azurerm_resource_group.rg.name
  listen              = true
  send                = true
  manage              = false
}