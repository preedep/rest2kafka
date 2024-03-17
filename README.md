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
- Step 1: run **initial.sh** 
- Step 2: run **validate.sh**
- Step 3: **apply.sh** 

```bash
    ./initial.sh
    ./validate.sh
    ./apply.sh
```

After that you will have the infrastructure created in Azure.

## Running the tests  
In the folder rest2kafka i have the script for the rest2kafka and the consumer.
- Step 1: Create .env file with the following variables

```bash
  APP_PORT=<<PORT>> ex. 8888
  KAFKA_EVENTHUB_ENDPOINT="<<azure event hub endpoint>>:9093"
  KAFKA_EVENTHUB_CONNECTION_STRING="<<azure event hub connection string primary or secondary>>"
```
Azure Event Hub endpoint and connection string can be found in the Azure portal in the Event Hub resource. after provisioning the Event Hub.

- Step 2: run the rest2kafka with **run_rest2kafka.sh**
- Step 3: run the consumer with **run_worker.sh**
- Step 4: send a POST request to the rest2kafka with the payload with **test_curl.sh**

```bash
    ./run_rest2kafka.sh
    ./run_worker.sh
    ./test_curl.sh
```


