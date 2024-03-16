data "azurerm_resource_group" "rg" {
  name     = "RG-SG-NICKDEV001"
  location = "southeastasia"
}
# Create azure event hub namespace using terraform
resource "azurerm_eventhub_namespace" "kafka_eh" {
  name                = lower("${var.kafka_eh_prefix}-${var.kafka_eh_namespace_name}")
  resource_group_name = data.azurerm_resource_group.rg.name
  location            = data.azurerm_resource_group.rg.location
  sku                 = var.kafka_eh_sku
  capacity            = var.kafka_eh_capacity
}