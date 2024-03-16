// ==========================  azure event hubs variables ==========================
variable "kafka_eh_prefix" {
  type        = string
  default     = "ehns"
  description = "Prefix of the Azure Event Hub (Kafka) name that's combined with name of the event hub namespace."
}
variable "kafka_eh_namespace_name" {
  type        = string
  default     = "nickeventhub1"
  description = "(Required) Specifies the resource group name of the Event Hub namespace name that's combined with name of the event hub namespace."
}
variable "kafka_eh_resource_group_name" {
  description = "(Required) Specifies the resource group name of the Event Hub."
  type        = string
  default     = "replace me"
}
variable "kafka_eh_location" {
  description = "(Required) Specifies the location where the Event Hub will be deployed."
  type        = string
  default     = "replace me"
}
variable "kafka_eh_sku" {
  description = "(Required) Defines which tier to use. Valid options are Basic, Standard, and Premium. Please note that setting this field to Premium will force the creation of a new resource."
  type        = string
  default     = "Standard"
  validation {
    condition     = contains(["Standard", "Premium"], var.kafka_eh_sku)
    error_message = "The sku of the event hub is invalid."
  }
}
variable "kafka_eh_capacity" {
  description = "(Optional) Specifies the Capacity / Throughput Units for a Standard SKU namespace. Default capacity has a maximum of 2, but can be increased in blocks of 2 on a committed purchase basis."
  type        = number
  default     = 2
}
variable "kafka_eh_partition_count" {
  description = "(Optional) Specifies the  number of partitions for a Kafka topic."
  type        = number
  default     = 10
}
variable "kafka_eh_message_retention" {
  description = "(Optional) Specifies the  number of message_retention "
  type        = number
  default     = 1
}
variable "kafka_eh_topics" {
  description = "(Optional) An array of strings that indicates values of kafka topics."
  type        = list(string)
  default = [
    "eventhub-1",
    "eventhub-2",
    "eventhub-3",
    "eventhub-4",
    "eventhub-5",
  ]
}
variable "kafka_eh_tags" {
  description = "(Optional) Specifies the tags of the Kafka event hub"
  type        = map(any)
  default     = {}
}