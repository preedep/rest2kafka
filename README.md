# rest2kafka
Project Rest2Kafka is a simple REST API that receives a JSON payload and sends it to a Kafka topic. and also it has a simple consumer that reads the messages from the Kafka topic and prints them to the console.
for Kafka i used the Azure Event Hub to be Kafka endpoint. 

Example payload
```json
{
  "topicName": "tpx_message",
  "message": {
    "username": "xyz",
    "password": "xyz"
  }
}
```
- topicName: the name of the topic in Kafka
- message: the message that will be sent to the topic (it can be any JSON object)

## Getting Started
In folders terraform i have the terraform code to create the infrastructure in Azure 
- Step 1: run initial.sh 
- Step 2: run validate.sh
- Step 3: apply.sh 

After that you will have the infrastructure created in Azure.

